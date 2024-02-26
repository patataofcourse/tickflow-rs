use std::num::IntErrorKind;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("file IO error: {0}")]
    IoError(std::io::Error),
    #[error("tickflow error on {fname}:{line} - {error}")]
    OldTfError {
        fname: String,
        line: usize,
        error: OldTfError,
    },
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
    #[error("unknown directive \"#{}\"", **_0)]
    InvalidDirective(crate::old::Identifier),
    #[error("invalid string prefix {0}\"\"")]
    InvalidStrPrefix(String),
    #[error("number out of range (0x00000000-0xFFFFFFFF)")]
    IntOutOfRange,
    #[error("syntax error")]
    SyntaxError,
    #[error("missing required directive \"#{0}\"")]
    MissingRequiredDirective(&'static str),
    #[error("included files cannot have #index, #start, or #assets directives")]
    IncludedDirective,
    #[error("undefined constant \"{}\"", **_0)]
    UndefinedConstant(crate::old::Identifier),
    #[error("operations can only be applied to integers")]
    InvalidOpType,
    #[error("arg0 of any command must be an integer")]
    InvalidArg0Type,
    #[error("arg0 value {0:05x} is out of range (must be 18 bits at most)")]
    OOBArg0(i32),
}

impl OldTfError {
    pub fn with_ctx(self, fname: &str, line_num: usize) -> Error {
        Error::OldTfError {
            error: self,
            fname: fname.to_owned(),
            line: line_num,
        }
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
