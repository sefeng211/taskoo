use crate::error::ClientError;

use anyhow::{Result};
use std::backtrace::Backtrace;
use clap::ArgMatches;
use log::{info};

use taskoo_core::command::{TagCommand, ContextCommand, StateCommand, SimpleCommand};

use dialoguer::{theme::ColorfulTheme, Select};

pub struct Clean;

impl Clean {
    pub fn run(provided_type: &String) -> Result<String, ClientError> {
        info!("Processing clean task");

        match provided_type.as_str() {
            "context" => {
                let command = ContextCommand::new()?;
                return Clean::process_remove_context(command);
            }
            "tag" => {
                let command = TagCommand::new()?;
                return Clean::process_remove_tag(command);
            }
            "state" => {
                let command = StateCommand::new()?;
                return Clean::process_remove_state(command);
            }
            &_ => {
                return Err(ClientError::UnexpectedFailure(
                String::from("The provided type is neither 'context' nor 'tag', so we can't process it, but how come?"), Backtrace::capture()));
            }
        }
    }
    fn process_remove_context<'a>(command: impl SimpleCommand<'a>) -> Result<String, ClientError> {
        return Clean::process_remove(command, "context");
    }

    fn process_remove_tag<'a>(command: impl SimpleCommand<'a>) -> Result<String, ClientError> {
        return Clean::process_remove(command, "tag");
    }

    fn process_remove_state<'a>(command: impl SimpleCommand<'a>) -> Result<String, ClientError> {
        return Clean::process_remove(command, "state");
    }

    fn process_remove<'a>(
        mut command: impl SimpleCommand<'a>,
        removal_type: &str,
    ) -> Result<String, ClientError> {
        let potential_options = command.get_all()?;
        let mut possible_options = vec![];

        for name in potential_options.iter() {
            if command.get_count(name)? == 0 {
                possible_options.push(name.clone());
            }
        }

        if possible_options.is_empty() {
            return Ok(String::from(format!(
                "All {} have at least a task associated, no {} can be removed",
                removal_type, removal_type
            )));
        }

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Select a {} to delete", removal_type))
            .default(0)
            .items(&possible_options)
            .interact()
            .map_err(|error| ClientError::TerminalError { source: error })?;

        command.delete(vec![possible_options[selection].clone()])?;
        Ok("clean command run successfully".to_string())
    }
}
