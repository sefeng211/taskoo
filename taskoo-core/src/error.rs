use chrono::format::ParseError;
use ini::Error as IniParseError;
use rusqlite::Error as SqlError;
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InitialError {
    #[error("IniError: Failed to parse the given ini file")]
    IniError {
        #[from]
        source: IniParseError,
    },
    #[error("IniError: Failed to write the default config to file {path}")]
    IoError { path: String, source: IoError },
    #[error("Unable to find the path of the config directory")]
    DirError(),
}

#[derive(Error, Debug)]
pub enum ArgumentError {
    #[error("Invalid context is provided {0}")]
    InvalidContext(String),
    #[error("Invalid Option {0}")]
    InvalidOption(String),
    #[error("Invalid tag {0}")]
    InvalidTag(String),
    #[error("Invalid state {0}")]
    InvalidState(String),
    #[error("Invalid view type: {0}")]
    InvalidViewType(String),
}

#[derive(Error, Debug)]
pub enum CoreError {
    #[error(transparent)]
    SqliteError(#[from] SqlError),
    #[error("Failed to parse given period {0}")]
    DateParseError(String),
    #[error("Unable to parse the provided {period} to a real time")]
    ChronoParseError { period: String, source: ParseError },
    #[error("RRule parsing error: Can't parse {0}")]
    RRuleParseError(String),
    #[error(transparent)]
    InitialErrorIni(#[from] InitialError),
    #[error("ArgumentError: {0}")]
    ArgumentError(String),
    #[error("UnexpetedError: {0}")]
    UnexpetedError(String),
}

impl From<ArgumentError> for CoreError {
    fn from(err: ArgumentError) -> Self {
        CoreError::ArgumentError(format!("{}", err))
    }
}

