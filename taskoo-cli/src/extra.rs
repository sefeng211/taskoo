use thiserror::Error;
use std::num::ParseIntError;
use taskoo_core::error::TaskooError;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Invalid scheduled at {0}")]
    InvalidScheduleAt(String),
    #[error("Invalid due date {0}")]
    InvalidDueDate(String),
    #[error("Invalid context name {0}")]
    InvalidContextName(String),
    #[error("Invalid tag name {0}")]
    InvalidTagName(String),
    #[error("Invalid tag name {0}")]
    InvalidTaskId(String),
    #[error(transparent)]
    ParsingError(#[from] ParseIntError),
}
