use chrono::format::ParseError;
use rusqlite::Error as DbError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("Sqlite error: {source:?}")]
    SqliteError { source: DbError },
    #[error("Failed to parse given period {0}")]
    PeriodParsingError(String),
    #[error("Chrono parsing error: {source:?}")]
    PeriodChronoParseError { source: ParseError },
    #[error("Invalid context is provided {0}")]
    InvalidContext(String),
    #[error("Invalid Option {0}")]
    InvalidOption(String),
    #[error("Invalid tag {0}")]
    InvalidTag(String),
}
