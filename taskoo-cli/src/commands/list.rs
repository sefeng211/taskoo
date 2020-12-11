use comfy_table::*;
use clap::ArgMatches;
use taskoo_core::operation::{execute, GetAllForContextOperation, GetOperation};
use yansi::Paint;

use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;

use std::io::Write;
use tabwriter::TabWriter;

pub struct List;

impl List {
    pub fn list(matches: &ArgMatches) {
        // TODO List all context
        if matches.is_present("context_name") {
            List::display_all(matches.value_of("context_name").unwrap());
        } else {
            List::display_all("Inbox");
            List::display_all("Work");
            List::display_all("Life");
        }
    }

    fn display_all(context_name: &str) {
        //let mut operation = GetOperation {
        //priority: None,
        //context_name: Some(context_name.to_string()),
        //tag_names: vec![],
        //due_date: None,
        //scheduled_at: None,
        //is_repeat: None,
        //is_recurrence: None,
        //database_manager: None,
        //result: vec![],
        //};
        let mut operation = GetAllForContextOperation {
            context_name: Some(context_name.to_string()),
            database_manager: None,
            result: vec![],
        };

        match execute(&mut operation) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed {}", e);
            }
        }

        let mut tabbedOutput = format!(
            "{}\t{}\t{}\t{}\t{}\n",
            Paint::red("Id").bold(),
            Paint::blue("Body"),
            Paint::yellow("Tag").bold(),
            Paint::yellow("Created At"),
            Paint::white("Scheduled At")
        );

        println!("{} ", Paint::red(context_name).underline(),);
        for task in operation.result.iter() {
            let mut formated_tag_names: String = "".to_string();
            for tag_name in task.tag_names.iter() {
                formated_tag_names.push_str("+");
                formated_tag_names.push_str(tag_name);
                formated_tag_names.push_str(" ");
            }
            let mut row: String;
            if !&task.is_completed {
                row = format!(
                    "{}\t{}\t{}\t{}\t{}\n",
                    Paint::red(&task.id.to_string()).bold(),
                    Paint::blue(&task.body.to_string()),
                    Paint::yellow(formated_tag_names).bold(),
                    Paint::yellow(&task.created_at),
                    Paint::white(&task.scheduled_at)
                );
            } else {
                row = format!(
                    "{}\t{}\t{}\t{}\t{}\n",
                    Paint::green(&task.id.to_string()).bold(),
                    Paint::green(&task.body.to_string()),
                    Paint::green(formated_tag_names).bold(),
                    Paint::green(&task.created_at),
                    Paint::green(&task.scheduled_at)
                );
            }
            tabbedOutput.push_str(&row);
        }

        let mut tw = TabWriter::new(vec![]);
        write!(&mut tw, "{}", tabbedOutput).unwrap();

        tw.flush().unwrap();
        let written = String::from_utf8(tw.into_inner().unwrap()).unwrap();
        println!("{}", written);
    }
}
