#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("regex library error: {0}")]
    RegexError(regex::Error),
    #[error("invalid identifier name \"{0}\"")]
    InvalidIdentifier(String),
}

impl From<regex::Error> for Error {
    fn from(value: regex::Error) -> Self {
        Self::RegexError(value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
