use clap::ArgMatches;
use taskoo_core::operation::{execute, AddOperation};

use crate::option_parser::{parse_command_option, CommandOption};
//use crate::option_parser::parse_command_option;
use log::{debug, error, info, log_enabled, Level};
pub struct Add;

impl Add {
    pub fn add(matches: &ArgMatches) {
        //let mut context_name = None;
        //let mut tag_names: Vec<String> = vec![];
        //let mut scheduled_at: Option<&str> = None;

        let mut option = CommandOption {
            scheduled_at: None,
            due_date: None,
            tag_names: vec![],
            context_name: None,
            body: None,
            remove_tag_names: vec![],
        };

        if matches.is_present("config") {
            let config: Vec<&str> = matches.values_of("config").unwrap().collect();
            option = parse_command_option(&config, true, false).unwrap();
        }

        let mut operation = AddOperation {
            body: &option.body.unwrap(),
            priority: None,
            context_name: option.context_name,
            tag_names: option.tag_names,
            due_date: option.due_date,
            scheduled_at: option.scheduled_at,
            is_repeat: None,
            is_recurrence: None,
            database_manager: None,
            result: None,
        };

        match execute(&mut operation) {
            Ok(_) => {
                println!("Successfully added task");
            }
            Err(e) => {
                eprintln!("Failed {}", e);
            }
        }
        println!("Add is called!");
    }
}
