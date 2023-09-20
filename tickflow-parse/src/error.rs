use std::convert::Infallible;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("regex library error: {0}")]
    RegexError(regex::Error),
    #[error("tickflow error: {0}")]
    OldTfError(OldTfError),
}

#[derive(Debug, Error)]
pub enum OldTfError {
    #[error("invalid identifier name \"{0}\"")]
    InvalidIdentifier(String),
    #[error("unknown directive \"{}\"", **_0)]
    InvalidDirective(crate::old::Identifier),
}

impl From<regex::Error> for Error {
    fn from(value: regex::Error) -> Self {
        Self::RegexError(value)
    }
}

impl From<OldTfError> for Error {
    fn from(value: OldTfError) -> Self {
        Self::OldTfError(value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
