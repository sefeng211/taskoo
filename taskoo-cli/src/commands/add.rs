use clap::ArgMatches;
use taskoo_core::error::TaskooError;
use taskoo_core::operation::{Add as AddOp, execute};

use crate::option_parser::{generate_default_command_option, parse_command_option};
//use crate::option_parser::parse_command_option;
//use log::{debug, error, info, log_enabled, Level};
pub struct Add;

impl Add {
    pub fn add(matches: &ArgMatches) -> Result<(), TaskooError> {
        let mut option = generate_default_command_option();

        if matches.is_present("config") {
            let config: Vec<&str> = matches.values_of("config").unwrap().collect();
            option = parse_command_option(&config, true, false, false).unwrap();
        }

        let body = option.body.unwrap();

        let mut operation = AddOp::new(&body);
        operation.context_name = option.context_name;
        operation.tag_names = option.tag_names;
        operation.due_date = option.due_date;
        operation.scheduled_at = option.scheduled_at;
        operation.repeat = option.repetition;
        operation.state_name = option.state_name;

        execute(&mut operation)?;
        Ok(())
    }
}
