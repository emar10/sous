//! Types for representing culinary recipes.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::ingredient::Ingredient;
use crate::metadata::Metadata;
use crate::SousError;

/// A culinary recipe describing how to make a dish.
#[derive(Clone, Debug, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct Recipe {
    /// Recipe [Metadata].
    #[serde(flatten)]
    pub metadata: Metadata,
    /// List of steps required to make the dish.
    pub steps: Vec<String>,
    /// List of [Ingredient]s required to make the dish.
    pub ingredients: Vec<Ingredient>,
}

impl Recipe {
    /// Create a new, empty recipe.
    pub fn new() -> Self {
        Default::default()
    }

    /// Load a recipe from the provided YAML string slice.
    pub fn from_yaml(content: &str) -> Result<Recipe, SousError> {
        Ok(serde_yaml::from_str(content)?)
    }

    /// Load a recipe from the provided file path.
    pub fn from_file(path: &Path) -> Result<Recipe, SousError> {
        let content = fs::read_to_string(path)?;

        Ok(Self::from_yaml(&content)?)
    }
}
