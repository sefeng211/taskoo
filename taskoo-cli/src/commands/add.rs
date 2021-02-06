use clap::ArgMatches;
use taskoo_core::error::TaskooError;
use taskoo_core::operation::{Add as AddOp, AddAnnotation, Get as GetOp, execute};

use crate::option_parser::{CommandOption, parse_command_option};
use taskoo_core::core::Operation;
use dialoguer::Editor;
use terminal_size::{Width, Height, terminal_size};

//use crate::option_parser::parse_command_option;
//use log::{debug, error, info, log_enabled, Level};
pub struct Add;

impl Add {
    pub fn add(matches: &ArgMatches) -> Result<(), TaskooError> {
        let mut option = CommandOption::new();

        if matches.is_present("config") {
            let config: Vec<&str> = matches.values_of("config").unwrap().collect();
            option = parse_command_option(&config, true, false, false).unwrap();
        }

        let body = option.body.unwrap();

        let mut operation = AddOp::new(&body);
        operation.context_name = option.context_name;
        operation.tag_names = option.tag_names;
        operation.due_date = option.due_date;
        operation.due_repeat = option.due_repeat;
        operation.scheduled_at = option.scheduled_at;
        operation.scheduled_repeat = option.scheudled_repeat;
        operation.state_name = option.state_name;

        execute(&mut operation)?;
        let result = &operation.get_result();
        assert_eq!(result.len(), 1);

        println!("{:?}", result[0]);
        Ok(())
    }

    pub fn add_annoation(matches: &ArgMatches) -> Result<(), TaskooError> {
        let delete_config: Vec<_> = matches.values_of("task_ids").unwrap().collect();

        let mut task_ids: Vec<i64> = vec![];

        if delete_config.len() == 1 {
            if delete_config[0].contains("..") {
                let ranged_selection = delete_config[0].split("..").collect::<Vec<&str>>();
                if ranged_selection.len() != 2 {
                    eprintln!("Invalid range provided {}", delete_config[0]);
                }
                let start = ranged_selection[0]
                    .parse::<i64>()
                    .expect("Can't find valid start from provided range");
                let end = ranged_selection[1]
                    .parse::<i64>()
                    .expect("Can't find valid end from provided range");
                task_ids = (start..=end).collect::<Vec<i64>>();
            } else {
                task_ids.push(delete_config[0].parse().expect("Invalid task id provided"));
            }
        } else {
            for item in delete_config.iter() {
                task_ids.push(item.parse().expect("Invalid task id provided"));
            }
        }

        assert_eq!(task_ids.len(), 1);

        // Get the existing annotation of this task
        let mut operation = GetOp::new();
        operation.task_id = Some(task_ids[0]);
        execute(&mut operation)?;

        if let Some(rv) = Editor::new()
            .edit(&operation.get_result()[0].annotation)
            .unwrap()
        {
            let mut operation = AddAnnotation::new(task_ids[0], rv);
            execute(&mut operation)?;
        } else {
            println!("Unable to open $Editor");
        }

        let size = terminal_size();
        if let Some((Width(w), Height(h))) = size {
            println!("Your terminal is {} cols wide and {} lines tall", w, h);
        } else {
            println!("Unable to get terminal size");
        }

        Ok(())
    }
}
