use std::backtrace::Backtrace;
use clap::ArgMatches;
use log::info;
use anyhow::Result;

use taskoo_core::core::Operation;
use taskoo_core::operation::{execute, Get as GetOp};

use crate::error::ClientError;

pub struct Info;

impl Info {
    pub fn new() -> Info {
        Info
    }

    pub fn run(&self, matches: &ArgMatches) -> Result<String, ClientError> {
        info!("Running info command");
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

        info!("Task id: {:?}", task_id);
        let mut operation = GetOp::new();
        operation.task_id = Some(task_id);
        execute(&mut operation)?;

        let tasks = &operation.get_result();
        if tasks.is_empty() {
            return Err(ClientError::UnexpectedFailure(
                String::from(format!("Unable to find task with id : {}", task_id)),
                Backtrace::capture(),
            ));
        }

        assert_eq!(tasks.len(), 1);

        if let Some(attr) = matches.value_of("attribute") {
            println!("{}", tasks[0].get_property_value(attr)?);
        } else {
            println!("{:?}", tasks[0]);
        }
        Ok(String::new())
    }
}
