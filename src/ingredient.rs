use std::fmt;
use std::fmt::Write;

use serde::{Deserialize, Serialize};

/// Represents an ingredient used in a recipe.
#[derive(Clone, Debug, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct Ingredient {
    /// The ingredient's display name
    pub name: String,
    /// Optional amount of the ingredient to be used
    pub amount: Option<f32>,
    /// Optional unit description
    pub unit: Option<String>,
}

impl Ingredient {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn to_string(&self) -> String {
        let mut ret = String::new();

        if let Some(unit) = &self.unit {
            write!(ret, "{} ", unit).unwrap();
        }

        if let Some(amount) = &self.amount {
            write!(ret, "{} ", amount).unwrap();
        }

        write!(ret, "{}", self.name).unwrap();
        ret
    }
}

impl fmt::Display for Ingredient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
