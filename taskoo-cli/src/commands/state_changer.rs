use clap::ArgMatches;
use log::{debug, info};
use anyhow::{Result, Context};

use taskoo_core::operation::{execute, ModifyOperation};
use taskoo_core::core::Operation;

use taskoo_core::option_parser::{parse_command_option, CommandOption};

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

    pub fn run(&self, task_ids: &Vec<u64>) -> Result<String> {
        let v2: Vec<String> = task_ids.iter().map(|s| s.to_string()).collect();
        let v3: Vec<&str> = v2.iter().map(|s| &**s).collect();

        let option = parse_command_option(&v3, false, true, true)
            .context("Unable to parse the provided option for modify")?;

        debug!("Running state_changer with {:?}", option.task_ids);

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
            operation.set_custom_state(self.custom_state.unwrap().to_string());
        }

        execute(&mut operation)?;

        let modified_task = operation.get_result();

        Ok(String::from(format!("Task: {}", modified_task[0].body)))
    }
}
