use log::{debug, info};
use anyhow::{Result, Context};

use taskoo_core::operation::{execute, ModifyOperation};
use taskoo_core::core::Operation;

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
        let mut tokens: Vec<String> = task_ids.iter().map(|s| s.to_string()).collect();
        if self.to_started {
            tokens.push(String::from("@started"));
        } else if self.to_completed {
            tokens.push(String::from("@completed"));
        } else if self.to_ready {
            tokens.push(String::from("@ready"));
        } else if self.to_blocked {
            tokens.push(String::from("@blocked"));
        } else {
            assert!(self.custom_state.is_some());
            tokens.push(format!("@{}", self.custom_state.unwrap()));
        }

        debug!("Running state_changer with {:?}", tokens);

        let mut operation = ModifyOperation::new(&tokens)
            .context("Unable to parse the provided option for modify")?;

        execute(&mut operation)?;

        let modified_task = operation.get_result();

        Ok(String::from(format!("Task: {}", modified_task[0].body)))
    }
}
