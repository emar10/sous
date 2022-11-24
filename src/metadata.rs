//! Types for recipe meta information.

use std::fmt;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Container for recipe meta information.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct Metadata {
    /// Display name for the recipe.
    pub name: String,
    /// Original author of the recipe.
    pub author: String,
    /// Servings yielded by the recipe as written.
    pub servings: u32,
    /// Optional URL source of the recipe.
    pub url: Option<String>,
    /// Optional time in minutes estimated for prep.
    pub prep_minutes: Option<u32>,
    /// Time in minutes estimated for cooking.
    pub cook_minutes: u32,
}

impl Metadata {
    /// Create new, empty metadata.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} by {}", self.name, self.author)
    }
}
