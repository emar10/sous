//! Types representing errors that can occur within Sous.

use thiserror::Error;

/// Errors that can occur within Sous.
#[derive(Error, Debug)]
pub enum SousError {
    /// An error that occurs when parsing YAML.
    #[error(transparent)]
    YamlError(#[from] serde_yaml::Error),

    /// An error that occurs when parsing JSON.
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),

    /// An error involving file I/O; wraps [std::io::Error].
    #[error(transparent)]
    FileError(#[from] std::io::Error),

    /// An error reading or rendering a template.
    #[error(transparent)]
    TemplateError(#[from] tera::Error),

    /// An error thrown when [crate::import::extract_schema_recipe] cannot find recipe data.
    #[error("no recipe data was found in the provided string")]
    NoSchemaFoundError(),

    /// Error related to importing other recipe formats.
    #[error("missing/bad value for {key}")]
    ImportError {
        /// Key with missing or malformed value.
        key: String,
    },

    /// An unknown internal error, likely indicates a bug.
    #[error("Unknown internal error")]
    Unknown,
}
