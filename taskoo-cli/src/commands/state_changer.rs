use clap::ArgMatches;
use log::{debug, info};
use anyhow::{Result, Context};

use taskoo_core::operation::{execute, ModifyOperation};

use crate::option_parser::{parse_command_option, CommandOption};

pub struct StateChanger<'a> {
    custom_state: Option<&'a str>,
    to_started: bool,
    to_completed: bool,
    to_blocked: bool,
    to_ready: bool,
}

impl<'a> StateChanger<'a> {
    pub fn to_custom(state: &str) -> StateChanger {
        StateChanger {
            custom_state: Some(state),
            to_started: false,
            to_completed: false,
            to_ready: false,
            to_blocked: false,
        }
    }

    pub fn to_completed() -> StateChanger<'a> {
        StateChanger {
            custom_state: None,
            to_started: false,
            to_completed: true,
            to_ready: false,
            to_blocked: false,
        }
    }

    pub fn to_started() -> StateChanger<'a> {
        StateChanger {
            custom_state: None,
            to_started: true,
            to_completed: false,
            to_ready: false,
            to_blocked: false,
        }
    }

    pub fn to_ready() -> StateChanger<'a> {
        StateChanger {
            custom_state: None,
            to_started: false,
            to_completed: false,
            to_ready: true,
            to_blocked: false,
        }
    }

    pub fn to_blocked() -> StateChanger<'a> {
        StateChanger {
            custom_state: None,
            to_started: false,
            to_completed: false,
            to_ready: false,
            to_blocked: true,
        }
    }

    pub fn run(&self, matches: &ArgMatches) -> Result<String> {
        info!("Running done command");

        let mut option = CommandOption::new();
        if matches.is_present("task_ids") {
            let config: Vec<&str> = matches.values_of("task_ids").unwrap().collect();
            option = parse_command_option(&config, false, true, true)
                .context("Unable to parse the provided option for modify")?;
        }

        debug!("Running Modify with {:?}", option.task_ids);

        let task_ids_copy = option.task_ids.clone();
        let mut operation = ModifyOperation::new(option.task_ids);
        if self.to_started {
            operation.set_state_to_started();
        } else if self.to_completed {
            operation.set_state_to_completed();
        } else if self.to_ready {
            operation.set_state_to_ready();
        } else if self.to_blocked {
            operation.set_state_to_blocked();
        } else {
            assert!(self.custom_state.is_some());
            operation.state_name = Some(self.custom_state.unwrap());
        }

        execute(&mut operation)?;
        Ok(String::from(format!("{:?}", task_ids_copy)))
    }
}
