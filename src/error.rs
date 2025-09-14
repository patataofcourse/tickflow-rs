use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    // wrappers
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("YAML (de)serialization error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    // the temporary yaml format we're using
    #[error("Sanity check for {fname} failed: {info}")]
    YamlSanityError { fname: String, info: String },
}

impl Error {
    pub fn yaml_sanity(fname: String, info: String) -> Self {
        Self::YamlSanityError { fname, info }
    }
}
