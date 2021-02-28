use clap::ArgMatches;
use ini::Ini;

use taskoo_core::command::ContextCommand;
use taskoo_core::command::SimpleCommand;
use taskoo_core::error::CoreError;
use taskoo_core::operation::{Get as GetOp};

use crate::display::Display;
use crate::option_parser::{CommandOption, parse_command_option};

pub struct List {
    config: Ini,
}

impl List {
    pub fn new(config: Ini) -> List {
        List { config: config }
    }

    pub fn list(&self, matches: &ArgMatches) -> Result<(), CoreError> {
        // TODO Read context from the configuration file
        match matches.values_of("arguments") {
            Some(arguments) => {
                let config: Vec<&str> = arguments.collect();
                let option = parse_command_option(&config, false, false, false).unwrap();

                match option.context_name {
                    Some(ref context) => {
                        let context_name = context.clone();
                        let mut operations_tuple =
                            List::get_operations(option, Some(vec![context_name.to_string()]))?;

                        assert_eq!(operations_tuple.len(), 1);
                        let operation_tuple = &mut operations_tuple[0];
                        let tabbed_string = String::from(&self.process_operation(
                            &operation_tuple.0,
                            &mut operation_tuple.1,
                            matches.is_present("all"),
                        )?);
                        Display::print(&tabbed_string);
                        Ok(())
                    }
                    None => {
                        // Apply the filter to all context
                        let mut operations_tuple = List::get_operations(option, None)?;
                        for operation_tuple in operations_tuple.iter_mut() {
                            let final_tabbed_string = String::from(&self.process_operation(
                                &operation_tuple.0,
                                &mut operation_tuple.1,
                                matches.is_present("all"),
                            )?);
                            // Skip the contexts that doesn't have tasks
                            if !final_tabbed_string.is_empty() {
                                Display::print(&final_tabbed_string);
                            }
                        }
                        Ok(())
                    }
                }
            }
            None => {
                let mut operation_tuples = List::get_operations(CommandOption::new(), None)?;
                for operation_tuple in operation_tuples.iter_mut() {
                    let final_tabbed_string = String::from(&self.process_operation(
                        &operation_tuple.0,
                        &mut operation_tuple.1,
                        matches.is_present("all"),
                    )?);
                    // Skip the contexts that doesn't have tasks
                    if !final_tabbed_string.is_empty() {
                        Display::print(&final_tabbed_string);
                    }
                }
                Ok(())
            }
        }
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
                let command = ContextCommand::new()?;
                let context_names = command.get_all()?;
                context_names
            }
        };

        let mut result = vec![];
        for context in context_names.iter() {
            let mut operation = GetOp::new();
            operation.context_name = Some(context.to_string());
            operation.tag_names = command_option.tag_names.clone();
            operation.due_date = command_option.due_date;
            operation.scheduled_at = command_option.scheduled_at;
            result.push((context.to_string(), operation));
        }
        Ok(result)
    }
}
