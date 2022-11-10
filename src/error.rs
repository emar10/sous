use thiserror::Error;

#[derive(Error, Debug)]
pub enum SousError {
    #[error(transparent)]
    YamlError(#[from] serde_yaml::Error),

    #[error(transparent)]
    FileError(#[from] std::io::Error),

    #[error("Unknown internal error")]
    Unknown,
}

