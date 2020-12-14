use crate::option_parser::parse_command_option;
use clap::ArgMatches;
use ini::Ini;
use log::debug;
use taskoo_core::operation::{execute, GetOperation};
use yansi::Color;
use yansi::Paint;

use std::io::Write;
use tabwriter::TabWriter;

pub struct List;

fn colorize<'a>(text: &'a str, is_bold: &str, color: &str) -> Paint<&'a str> {
    debug!(
        "Colorized {} with bold {} and color {}",
        text, is_bold, color
    );
    let mut paint = Paint::new(text);
    if is_bold == "true" || is_bold == "True" {
        paint = paint.bold();
    }

    match color {
        "yellow" | "Yellow" => {
            paint = paint.fg(Color::Yellow);
        }
        "black" | "Black" => {
            paint = paint.fg(Color::Black);
        }
        "red" | "Red" => {
            paint = paint.fg(Color::Red);
        }
        "green" | "Green" => {
            paint = paint.fg(Color::Green);
        }
        "blue" | "Blue" => {
            paint = paint.fg(Color::Blue);
        }
        "magenta" | "Magenta" => {
            paint = paint.fg(Color::Magenta);
        }
        "cyan" | "Cyan" => {
            paint = paint.fg(Color::Cyan);
        }
        "white" | "White" => {
            paint = paint.fg(Color::White);
        }
        _ => {}
    }
    return paint;
}

impl List {
    pub fn list(matches: &ArgMatches, cli_config: Ini) {
        // TODO Read context from the configuration file
        if matches.is_present("arguments") {
            let config: Vec<&str> = matches.values_of("arguments").unwrap().collect();
            let option = parse_command_option(&config, false, false, false).unwrap();
            if option.context_name.is_some() {
                List::display_all(
                    None,
                    option.context_name,
                    option.tag_names,
                    option.due_date,
                    option.scheduled_at,
                    None,
                    None,
                    &cli_config,
                );
            } else {
                List::display_all(
                    None,
                    Some("Inbox".to_string()),
                    option.tag_names.clone(),
                    option.due_date.clone(),
                    option.scheduled_at.clone(),
                    None,
                    None,
                    &cli_config,
                );

                List::display_all(
                    None,
                    Some("Work".to_string()),
                    option.tag_names.clone(),
                    option.due_date.clone(),
                    option.scheduled_at.clone(),
                    None,
                    None,
                    &cli_config,
                );
                List::display_all(
                    None,
                    Some("Life".to_string()),
                    option.tag_names.clone(),
                    option.due_date.clone(),
                    option.scheduled_at.clone(),
                    None,
                    None,
                    &cli_config,
                );
            }
        } else {
            List::display_all(
                None,
                Some("Inbox".to_string()),
                vec![],
                None,
                None,
                None,
                None,
                &cli_config,
            );
            List::display_all(
                None,
                Some("Work".to_string()),
                vec![],
                None,
                None,
                None,
                None,
                &cli_config,
            );
            List::display_all(
                None,
                Some("Life".to_string()),
                vec![],
                None,
                None,
                None,
                None,
                &cli_config,
            );
        }
    }

    fn display_all(
        priority: Option<u8>,
        context_name: Option<String>,
        tag_names: Vec<String>,
        due_date: Option<&str>,
        scheduled_at: Option<&str>,
        is_repeat: Option<u8>,
        is_recurrence: Option<u8>,
        config: &Ini,
    ) {
        let context_name_copy = context_name.clone().unwrap();

        let mut operation = GetOperation {
            priority: priority,
            context_name: context_name,
            tag_names: tag_names,
            due_date: due_date,
            scheduled_at: scheduled_at,
            is_repeat: is_repeat,
            is_recurrence: is_recurrence,
            database_manager: None,
            result: vec![],
        };

        match execute(&mut operation) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed {}", e);
            }
        }

        let mut tabbed_output = format!(
            "{}\t{}\t{}\t{}\t{}\n",
            colorize(
                "Id",
                &config
                    .section(Some("Id"))
                    .unwrap()
                    .get("bold")
                    .unwrap()
                    .to_lowercase(),
                &config
                    .section(Some("Id"))
                    .unwrap()
                    .get("color")
                    .unwrap()
                    .to_lowercase()
            ),
            colorize(
                "Body",
                &config
                    .section(Some("Body"))
                    .unwrap()
                    .get("bold")
                    .unwrap()
                    .to_lowercase(),
                &config
                    .section(Some("Body"))
                    .unwrap()
                    .get("color")
                    .unwrap()
                    .to_lowercase()
            ),
            colorize(
                "Tag",
                &config
                    .section(Some("Tag"))
                    .unwrap()
                    .get("bold")
                    .unwrap()
                    .to_lowercase(),
                &config
                    .section(Some("Tag"))
                    .unwrap()
                    .get("color")
                    .unwrap()
                    .to_lowercase()
            ),
            colorize(
                "Created At",
                &config
                    .section(Some("Created_At"))
                    .unwrap()
                    .get("bold")
                    .unwrap()
                    .to_lowercase(),
                &config
                    .section(Some("Created_At"))
                    .unwrap()
                    .get("color")
                    .unwrap()
                    .to_lowercase()
            ),
            colorize(
                "Scheduled At",
                &config
                    .section(Some("Scheduled_At"))
                    .unwrap()
                    .get("bold")
                    .unwrap()
                    .to_lowercase(),
                &config
                    .section(Some("Scheduled_At"))
                    .unwrap()
                    .get("color")
                    .unwrap()
                    .to_lowercase()
            ),
        );

        println!("{} ", Paint::red(context_name_copy).underline(),);
        for task in operation.result.iter() {
            let mut formated_tag_names: String = "".to_string();
            for tag_name in task.tag_names.iter() {
                formated_tag_names.push_str("+");
                formated_tag_names.push_str(tag_name);
                formated_tag_names.push_str(" ");
            }
            let row = format!(
                "{}\t{}\t{}\t{}\t{}\n",
                colorize(
                    &task.id.to_string(),
                    &config
                        .section(Some("Id"))
                        .unwrap()
                        .get("bold")
                        .unwrap()
                        .to_lowercase(),
                    &config
                        .section(Some("Id"))
                        .unwrap()
                        .get("color")
                        .unwrap()
                        .to_lowercase()
                ),
                colorize(
                    &task.body.to_string(),
                    &config
                        .section(Some("Body"))
                        .unwrap()
                        .get("bold")
                        .unwrap()
                        .to_lowercase(),
                    &config
                        .section(Some("Body"))
                        .unwrap()
                        .get("color")
                        .unwrap()
                        .to_lowercase()
                ),
                colorize(
                    &formated_tag_names.to_string(),
                    &config
                        .section(Some("Tag"))
                        .unwrap()
                        .get("bold")
                        .unwrap()
                        .to_lowercase(),
                    &config
                        .section(Some("Tag"))
                        .unwrap()
                        .get("color")
                        .unwrap()
                        .to_lowercase()
                ),
                colorize(
                    &task.created_at.to_string(),
                    &config
                        .section(Some("Created_At"))
                        .unwrap()
                        .get("bold")
                        .unwrap()
                        .to_lowercase(),
                    &config
                        .section(Some("Created_At"))
                        .unwrap()
                        .get("color")
                        .unwrap()
                        .to_lowercase()
                ),
                colorize(
                    &task.scheduled_at.to_string(),
                    &config
                        .section(Some("Scheduled_At"))
                        .unwrap()
                        .get("bold")
                        .unwrap()
                        .to_lowercase(),
                    &config
                        .section(Some("Scheduled_At"))
                        .unwrap()
                        .get("color")
                        .unwrap()
                        .to_lowercase()
                )
            );
            tabbed_output.push_str(&row);
        }

        let mut tw = TabWriter::new(vec![]);
        write!(&mut tw, "{}", tabbed_output).unwrap();

        tw.flush().unwrap();
        let written = String::from_utf8(tw.into_inner().unwrap()).unwrap();
        println!("{}", written);
    }
}
