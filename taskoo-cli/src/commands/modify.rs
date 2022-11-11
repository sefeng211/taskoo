use anyhow::{Context, Result};

use taskoo_core::operation::{execute, ModifyOperation};
use crate::option_parser::parse_command_option;
use log::{debug, info};

pub struct Modify;

impl Modify {
    pub fn modify(matches: &Vec<String>) -> Result<String> {
        info!("Modifying Task");

        let v2: Vec<&str> = matches.iter().map(|s| &**s).collect();
        let option = parse_command_option(&v2, false, true, true)
            .context("Unable to parse the provided option for modify")?;

        let mut operation = ModifyOperation::new(option.task_ids);
        operation.context_name = option.context;
        operation.tag_names = option.tags;
        operation.due_date = option.date_due;
        operation.scheduled_at = option.date_scheduled;
        operation.due_repeat = option.repetition_due;
        operation.scheduled_repeat = option.repetition_scheduled;
        operation.tags_to_remove = option.tags_to_remove;
        if option.state == Some(String::from("started")) {
            operation.set_state_to_started();
        } else if option.state == Some(String::from("completed")) {
            operation.set_state_to_completed();
        } else if option.state == Some(String::from("ready")) {
            operation.set_state_to_ready();
        } else if option.state == Some(String::from("blocked")) {
            operation.set_state_to_blocked();
        } else if option.state.is_some() {
            operation.set_custom_state(option.state.unwrap().to_string());
        }
        operation.priority = option.priority;

        debug!("Executing ModifyOperation {:?}", operation);
        execute(&mut operation)?;
        Ok(String::new())
    }
}
