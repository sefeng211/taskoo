use thiserror::Error;
use std::backtrace::Backtrace;

use taskoo_core::error::{CoreError, ArgumentError};

use crate::extra::CommandError;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("{attr} is missing, unable to process the command")]
    MissingAttrError { attr: String, backtrace: Backtrace },
    #[error(transparent)]
    CoreError(#[from] CoreError),
    #[error("{0}")]
    UnexpectedFailure(String, Backtrace),
    #[error("Failed to parse {0} to format {0}")]
    ParseError(String, String),
    #[error(transparent)]
    CommandError(#[from] CommandError),
    #[error("Terminal error, abort!")]
    TerminalError(),
    #[error("ArgumentError: {0}")]
    ArgumentError(String),
}

impl From<ArgumentError> for ClientError {
    fn from(err: ArgumentError) -> Self {
        ClientError::ArgumentError(format!("{}", err))
    }
}
