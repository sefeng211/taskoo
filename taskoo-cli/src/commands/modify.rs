use clap::ArgMatches;
use taskoo_core::operation::{execute, ModifyOperation};

use crate::option_parser::{parse_command_option, parse_task_ids, CommandOption};
//use crate::option_parser::parse_command_option;
use log::{debug, error, info, log_enabled, Level};
pub struct Modify;

impl Modify {
    pub fn modify(matches: &ArgMatches) {
        info!("Processing Modify Task");

        let mut option = CommandOption {
            scheduled_at: None,
            due_date: None,
            tag_names: vec![],
            context_name: None,
            body: None,
            remove_tag_names: vec![],
        };

        let mut task_ids: Vec<i64> = vec![];

        if matches.is_present("task_id") {
            let task_id = matches.value_of("task_id").unwrap();
            println!("Task id provided {}", task_id);
            task_ids = parse_task_ids(&task_id.to_string()).unwrap();
            println!("Task id provided {:?}", task_ids);
        }

        if matches.is_present("options") {
            let config: Vec<&str> = matches.values_of("options").unwrap().collect();
            option = parse_command_option(&config, false, true).unwrap();
        }

        debug!("Context Name {:?}", option.context_name);
        let mut operation = ModifyOperation {
            database_manager: None,
            result: vec![],
            task_ids: task_ids,
            body: None,
            priority: None,
            context_name: option.context_name,
            tag_names: option.tag_names,
            due_date: None,
            scheduled_at: option.scheduled_at,
            is_repeat: None,
            is_recurrence: None,
        };

        match execute(&mut operation) {
            Ok(_) => {
                println!("Finished modify tasks");
            }
            Err(e) => {
                eprintln!("Failed {}", e);
            }
        }
    }
}
