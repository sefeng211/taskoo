use crate::display::Display;
use crate::option_parser::parse_command_option;
use clap::ArgMatches;
use ini::Ini;
use log::debug;
use taskoo_core::command::Command;
use taskoo_core::error::TaskooError;
use taskoo_core::operation::{execute, Get as GetOp};

use std::io::Write;
use tabwriter::TabWriter;

pub struct List {
    config: Ini,
}

impl List {
    pub fn new(config: Ini) -> List {
        List { config: config }
    }

    pub fn list(&self, matches: &ArgMatches) -> Result<(), TaskooError> {
        // TODO Read context from the configuration file
        if matches.is_present("arguments") {
            let config: Vec<&str> = matches.values_of("arguments").unwrap().collect();
            let option = parse_command_option(&config, false, false, false).unwrap();
            if option.context_name.is_some() {
                let mut final_tabbed_string = String::new();
                final_tabbed_string.push_str(&self.get_tasks(
                    &option.context_name.unwrap(),
                    option.tag_names,
                    option.due_date,
                    option.scheduled_at,
                )?);
                Display::print(&final_tabbed_string);
            } else {
                let context_names = Command::context(None)?;
                let mut final_tabbed_string = String::new();
                for context in context_names.iter() {
                    final_tabbed_string.push_str(&self.get_tasks(
                        context,
                        option.tag_names.clone(),
                        option.due_date.clone(),
                        option.scheduled_at.clone(),
                    )?);
                }
                Display::print(&final_tabbed_string);
            }
        } else {
            let context_names = Command::context(None).unwrap();
            let mut final_tabbed_string = String::new();
            for context in context_names.iter() {
                final_tabbed_string.push_str(&self.get_tasks_for_context(context)?);
            }
            Display::print(&final_tabbed_string);
        }
        Ok(())
    }

    fn get_tasks_for_context(&self, context_name: &str) -> Result<String, TaskooError> {
        return self.get_tasks(context_name, vec![], None, None);
    }

    fn get_tasks(
        &self,
        context_name: &str,
        tag_names: Vec<String>,
        due_date: Option<&str>,
        scheduled_at: Option<&str>,
    ) -> Result<String, TaskooError> {
        // Rows
        let mut operation = GetOp::new();
        operation.context_name = Some(context_name.to_string());
        operation.tag_names = tag_names;
        operation.due_date = due_date;
        operation.scheduled_at = scheduled_at;

        return Display::display(&context_name, &mut operation, &self.config);
    }
}
