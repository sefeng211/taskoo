use anyhow::{Context, Result};
use clap::ArgMatches;
use taskoo_core::error::TaskooError;
use taskoo_core::operation::{execute, ModifyOperation};

use crate::option_parser::{generate_default_command_option, parse_command_option};
//use crate::option_parser::parse_command_option;
use log::{debug, info};
pub struct Modify;

impl Modify {
    pub fn modify(matches: &ArgMatches) -> Result<()> {
        info!("Processing Modify Task");

        let mut option = generate_default_command_option();

        if matches.is_present("args") {
            let config: Vec<&str> = matches.values_of("args").unwrap().collect();
            option = parse_command_option(&config, false, true, true)
                .context("Unable to parse the provided option for modify")?;
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
            due_date: option.due_date,
            scheduled_at: option.scheduled_at,
            repeat: None,
            recurrence: None,
            state_name: None,
        };
        execute(&mut operation)?;
        Ok(())
    }
}
