use anyhow::{Context, Result};
use clap::ArgMatches;
use taskoo_core::error::TaskooError;
use taskoo_core::operation::{execute, ModifyOperation};

use crate::option_parser::{parse_command_option, CommandOption};
use log::{debug, info};
pub struct Modify;

impl Modify {
    pub fn modify(matches: &ArgMatches) -> Result<()> {
        info!("Processing Modify Task");

        let mut option = CommandOption::new();
        if matches.is_present("args") {
            let config: Vec<&str> = matches.values_of("args").unwrap().collect();
            option = parse_command_option(&config, false, true, true)
                .context("Unable to parse the provided option for modify")?;
        }

        debug!("Context Name {:?}", option.context_name);
        let mut operation = ModifyOperation::new(option.task_ids);
        operation.context_name = option.context_name;
        operation.tag_names = option.tag_names;
        operation.due_date = option.due_date;
        operation.scheduled_at = option.scheduled_at;
        operation.due_repeat = option.due_repeat;
        operation.scheduled_repeat = option.scheudled_repeat;
        operation.state_name = option.state_name.as_deref();

        execute(&mut operation)?;
        Ok(())
    }
}
