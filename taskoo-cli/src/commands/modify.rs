use clap::ArgMatches;
use taskoo_core::operation::{execute, ModifyOperation};

use crate::option_parser::{generate_default_command_option, parse_command_option};
//use crate::option_parser::parse_command_option;
use log::{debug, info};
pub struct Modify;

impl Modify {
    pub fn modify(matches: &ArgMatches) {
        info!("Processing Modify Task");

        let mut option = generate_default_command_option();

        if matches.is_present("args") {
            let config: Vec<&str> = matches.values_of("args").unwrap().collect();
            option = parse_command_option(&config, false, true, true).unwrap();
        }

        debug!("Context Name {:?}", option.context_name);
        let mut operation = ModifyOperation {
            database_manager: None,
            result: vec![],
            task_ids: option.task_ids,
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
