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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_yaml() -> Result<(), SousError> {
        let yaml = "
name: test recipe
author: test author
servings: 1
cook_minutes: 1
steps:
  - step 1
ingredients:
  - name: test ingredient
    amount: 1
    unit: test unit
        ";

        println!("{}", yaml);
        let recipe = Recipe::from_yaml(yaml)?;
        assert_eq!(recipe.metadata.name, "test recipe");
        assert_eq!(recipe.metadata.author, "test author");
        assert_eq!(recipe.metadata.servings, 1);
        assert_eq!(recipe.metadata.cook_minutes, 1);
        assert_eq!(recipe.steps, vec!["step 1"]);
        assert_eq!(
            recipe.ingredients,
            vec![Ingredient {
                name: "test ingredient".to_string(),
                amount: Some(1.0),
                unit: Some("test unit".to_string())
            }]
        );
        Ok(())
    }

    #[test]
    #[should_panic]
    fn from_yaml_missing_name() {
        let yaml = "
author: test
servings: 1
cook_minutes: 1
steps:
  - step 1
ingredients:
  - name: test ingredient
  - amount: 1
  - unit: test unit
        ";

        Recipe::from_yaml(yaml).unwrap();
    }

    #[test]
    #[should_panic]
    fn from_yaml_bad_value() {
        let yaml = "
author: test
servings: one
cook_minutes: 1
steps:
  - step 1
ingredients:
  - name: test ingredient
  - amount: 1
  - unit: test unit
        ";

        Recipe::from_yaml(yaml).unwrap();
    }
}
