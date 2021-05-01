use clap::ArgMatches;
use taskoo_core::operation::{Add as AddOp, AddAnnotation, Get as GetOp, execute};

use crate::option_parser::{CommandOption, parse_command_option};
use crate::error::ClientError;
use taskoo_core::core::Operation;
use dialoguer::Editor;
use std::backtrace::Backtrace;

//use crate::option_parser::parse_command_option;
use log::debug;
pub struct Add;

impl Add {
    pub fn add(matches: &ArgMatches) -> Result<String, ClientError> {
        let mut option = CommandOption::new();
        if matches.is_present("arguments") {
            let config: Vec<&str> = matches.values_of("arguments").unwrap().collect();
            option = parse_command_option(&config, true, false, false).unwrap();
            debug!("Option {:?}", option);
        }

        let body = option.body.ok_or(ClientError::MissingAttrError {
            attr: String::from("<Body> of the task"),
            backtrace: Backtrace::capture(),
        })?;

        let mut operation = AddOp::new(&body);
        operation.context = option.context;
        operation.tags = option.tags;
        operation.date_due = option.date_due;
        operation.repetition_due = option.repetition_due;
        operation.date_scheduled = option.date_scheduled;
        operation.repetition_scheduled = option.repetition_scheduled;
        if option.state == Some(String::from("ready")) {
            operation.set_state_to_ready();
        } else if option.state == Some(String::from("completed")) {
            operation.set_state_to_completed();
        } else if option.state == Some(String::from("blocked")) {
            operation.set_state_to_blocked();
        } else if option.state == Some(String::from("started")) {
            operation.set_state_to_started();
        } else if option.state.is_some() {
            operation.set_custom_state(option.state.unwrap());
        }

        operation.priority = option.priority;
        operation.parent_task_ids = option.parent_task_ids;

        let annotation = if matches.is_present("annotation") {
            if let Some(rv) = Editor::new().edit("").unwrap() {
                Some(rv.clone())
            } else {
                return Err(ClientError::UnexpectedFailure(
                    String::from("Unable to get the annotation text, abort!"),
                    Backtrace::capture(),
                ));
            }
        } else {
            None
        };
        operation.annotation = annotation.as_deref();

        execute(&mut operation)?;

        let added_tasks = &operation.get_result();
        if added_tasks.len() != 1 {
            return Err(ClientError::UnexpectedFailure(
                String::from(
                    "Add operation failed in an unexpected way, please consider to report it",
                ),
                Backtrace::capture(),
            ));
        }

        let task = &added_tasks[0];
        Ok(String::from(format!(
            "Added [id: {}, body: {}]",
            task.id, task.body
        )))
    }

    pub fn add_annoation(matches: &ArgMatches) -> Result<String, ClientError> {
        let task_id = match matches.value_of("task_id") {
            Some(raw_task_id) => raw_task_id.parse::<i64>().map_err(|_error| {
                ClientError::ParseError(String::from(raw_task_id), String::from("i64"))
            })?,
            None => {
                return Err(ClientError::MissingAttrError {
                    attr: String::from("task_id"),
                    backtrace: Backtrace::capture(),
                });
            }
        };

        // Get the existing annotation of this task
        let mut operation = GetOp::new();
        operation.task_id = Some(task_id);
        execute(&mut operation)?;

        let tasks = &operation.get_result();
        if tasks.len() != 1 {
            return Err(ClientError::UnexpectedFailure(
                String::from(
                    "AddAnnotation operation failed in an unexpected way, please consider to report it",
                ),
                Backtrace::capture(),
            ));
        }

        if let Some(rv) = Editor::new().edit(&tasks[0].annotation).unwrap() {
            let mut operation = AddAnnotation::new(task_id, rv);
            execute(&mut operation)?;
        } else {
            return Err(ClientError::UnexpectedFailure(
                String::from("Unable to get the annotation text, abort!"),
                Backtrace::capture(),
            ));
        }

        Ok(String::from(format!(
            "Added Annotation to task: {}",
            tasks[0].id
        )))
    }
}
