use std::backtrace::Backtrace;
use clap::ArgMatches;
use log::info;
use anyhow::Result;

use taskoo_core::core::Operation;
use taskoo_core::operation::{execute, Get as GetOp};
use taskoo_core::command::{SimpleCommand, TagCommand};

use crate::error::ClientError;

pub struct Info;

impl Info {
    pub fn new() -> Info {
        Info
    }

    pub fn run(
        &self,
        task_id: &Option<u64>,
        attribute: &Option<String>,
    ) -> Result<String, ClientError> {
        info!("Running info command");
        if task_id.is_none() && attribute.is_none() {
            return Err(ClientError::MissingAttrError {
                attr: String::from("None of task_id or attribute is provided"),
                backtrace: Backtrace::capture(),
            });
        }
        // let task_id = match matches.value_of("task_id") {
        //     Some(raw_task_id) => raw_task_id.parse::<i64>().map_err(|_error| {
        //         ClientError::ParseError(String::from(raw_task_id), String::from("i64"))
        //     })?,
        //     None => {
        //         return Err(ClientError::MissingAttrError {
        //             attr: String::from("task_id"),
        //             backtrace: Backtrace::capture(),
        //         });
        //     }
        // };

        if let Some(id) = task_id {
            if *id > std::i64::MAX as u64 {
                return Err(ClientError::MissingAttrError {
                    attr: String::from("Provided task_id is greater than i64::MAX"),
                    backtrace: Backtrace::capture(),
                });
            }

            let real_id = *id as i64;
            info!("Task id: {:?}", real_id);
            let mut operation = GetOp::new();
            operation.task_id = Some(real_id);
            execute(&mut operation)?;

            let tasks = &operation.get_result();
            if tasks.is_empty() {
                return Err(ClientError::UnexpectedFailure(
                    String::from(format!("Unable to find task with id : {}", real_id)),
                    Backtrace::capture(),
                ));
            }

            assert_eq!(tasks.len(), 1);

            if let Some(attr) = attribute {
                println!("{}", tasks[0].get_property_value(attr)?);
            } else {
                println!("{:?}", tasks[0]);
            }
            return Ok(String::new());
        }

        if let Some(attr) = attribute {
            match attr.as_str() {
                "tag" => {
                    let mut tag_command = TagCommand::new()?;
                    let tags = tag_command.get_all()?;
                    println!("{:?}", tags);
                }
                _ => {
                    // Should crash
                }
            }
        }
        Ok(String::new())
    }
}
