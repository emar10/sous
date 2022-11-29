//! Library for handling culinary recipes.
//!
//! This crate provides structs representing various components of a culinary
//! recipe, along with utilities for converting them between various formats.
//!
//! ```rust
//! # use sous::ingredient::Ingredient;
//! # use sous::recipe::Recipe;
//! # use sous::render::RenderSettings;
//! fn main() {
//!     let mut recipe = Recipe::new();
//!
//!     recipe.metadata.name = "Test Recipe".to_string();
//!     recipe.metadata.author = "Cook Cookerson".to_string();
//!     recipe.metadata.servings = 2;
//!     recipe.metadata.cook_minutes = 10;
//!
//!     recipe.ingredients.push(Ingredient {
//!         name: "Ingredient".to_string(),
//!         amount: Some(1.0),
//!         ..Default::default()
//!     });
//!     recipe.steps.push("First step".to_string());
//!     recipe.steps.push("Second step".to_string());
//!
//!     let md = recipe.to_markdown(&RenderSettings::default());
//! }
//! ```

#![warn(missing_docs)]

pub use crate::cookbook::Cookbook;
pub use crate::error::SousError;
pub use crate::ingredient::Ingredient;
pub use crate::metadata::Metadata;
pub use crate::recipe::Recipe;
pub use crate::render::Markdown;

pub mod cookbook;
pub mod error;
pub mod ingredient;
pub mod metadata;
pub mod recipe;
pub mod render;
