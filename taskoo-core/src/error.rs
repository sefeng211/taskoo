use chrono::format::ParseError;
use rusqlite::Error as SqlError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskooError {
    #[error(transparent)]
    SqliteError(#[from] SqlError),
    #[error("Failed to parse given period {0}")]
    PeriodParsingError(String),
    #[error("Unable to parse the provided {period} to a real time")]
    PeriodChronoParseError { period: String, source: ParseError },
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
    #[error("RRule parsing error: Can't parse {0}")]
    RRuleParseError(String)
}
