use clap::ArgMatches;
use ini::Ini;

use taskoo_core::command::ContextCommand;
use taskoo_core::command::SimpleCommand;
use taskoo_core::error::CoreError;
use taskoo_core::operation::{Get as GetOp};
use taskoo_core::option_parser::{CommandOption, parse_command_option};

use crate::display::Display;

pub struct List {
    config: Ini,
}

impl List {
    pub fn new(config: Ini) -> List {
        List { config: config }
    }

    pub fn list(&self, all: bool, matches: &Vec<String>) -> Result<String, CoreError> {
        let mut operations = GetOp::new2(&matches)?;
        for operation_tuple in operations.iter_mut() {
            let final_tabbed_string = String::from(&self.process_operation(
                &operation_tuple.0,
                &mut operation_tuple.1,
                all,
            )?);
            // Skip the contexts that doesn't have tasks
            if !final_tabbed_string.is_empty() {
                Display::print(&final_tabbed_string);
            }
        }
        Ok(String::new())
    }

    fn process_operation(
        &self,
        context_name: &str,
        operation: &mut GetOp,
        display_completed: bool,
    ) -> Result<String, CoreError> {
        return Display::display(&context_name, operation, &self.config, display_completed);
    }

    pub fn get_operations(
        command_option: CommandOption,
        some_context_names: Option<Vec<String>>,
    ) -> Result<Vec<(String, GetOp)>, CoreError> {
        let context_names = match some_context_names {
            Some(context_names) => context_names,
            None => {
                // If no context names are passed, use all context
                let mut command = ContextCommand::new()?;
                let context_names = command.get_all()?;
                context_names
            }
        };

        let mut result = vec![];
        for context in context_names.iter() {
            let mut operation = GetOp::new();
            operation.context = Some(context.to_string());
            operation.tags = command_option.tags.clone();
            operation.date_due = command_option.date_due;
            operation.date_scheduled = command_option.date_scheduled;
            operation.not_tags = command_option.not_tags.clone();
            result.push((context.to_string(), operation));
        }
        Ok(result)
    }
}
