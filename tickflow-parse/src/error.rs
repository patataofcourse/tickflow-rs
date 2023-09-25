use std::num::IntErrorKind;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("regex library error: {0}")]
    RegexError(regex::Error),
    #[error("file IO error: {0}")]
    IoError(std::io::Error),
    //TODO: store line numbers
    #[error("tickflow error on line {1}: {0}")]
    OldTfError(OldTfError, usize),
}

pub fn nom_ok<I, O, E: nom::error::ParseError<I>>(
    out: O,
    remaining: I,
) -> nom::IResult<I, Result<O>, E> {
    Ok((remaining, Ok(out)))
}

impl Error {
    pub fn wrap_nom<I, O, E: nom::error::ParseError<I>>(
        self,
        remaining: I,
    ) -> nom::IResult<I, Result<O>, E> {
        Ok((remaining, Err(self)))
    }
}

#[derive(Debug, Error)]
pub enum OldTfError {
    #[error("invalid identifier name \"{0}\"")]
    InvalidIdentifier(String),
    #[error("unknown directive \"{}\"", **_0)]
    InvalidDirective(crate::old::Identifier),
    #[error("invalid string prefix {0}\"\"")]
    InvalidStrPrefix(String),
    #[error("number out of range (0x00000000-0xFFFFFFFF)")]
    IntOutOfRange,
    #[error("syntax error")]
    SyntaxError,
}

impl OldTfError {
    pub fn with_ctx(self, line_num: usize) -> Error {
        Error::OldTfError(self, line_num)
    }
}

impl From<regex::Error> for Error {
    fn from(value: regex::Error) -> Self {
        Self::RegexError(value)
    }
}

impl From<IntErrorKind> for OldTfError {
    fn from(_: IntErrorKind) -> Self {
        Self::IntOutOfRange
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
