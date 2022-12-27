//! Types representing errors that can occur within Sous.

use thiserror::Error;

/// Errors that can occur within Sous.
#[derive(Error, Debug)]
pub enum SousError {
    /// An error that occurs when parsing YAML.
    #[error(transparent)]
    YamlError(#[from] serde_yaml::Error),

    /// An error involving file I/O; wraps [std::io::Error].
    #[error(transparent)]
    FileError(#[from] std::io::Error),

    /// An error reading or rendering a template.
    #[error(transparent)]
    TemplateError(#[from] tera::Error),

    /// An unknown internal error, likely indicates a bug.
    #[error("Unknown internal error")]
    Unknown,
}
