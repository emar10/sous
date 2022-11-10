use std::{path::Path, fs::{self, File}};
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;

use serde::{Serialize, Deserialize};

use crate::{SousError, render::RenderSettings};

/// Container for ingredient metadata
#[derive(Serialize, Deserialize, Debug)]
pub struct Ingredient {
    /// The ingredient's display name
    pub name: String,
    /// Optional unit description
    pub unit: Option<String>,
    /// Optional amount of the ingredient to be used
    pub amount: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    /// Display name for the recipe
    pub name: String,
    /// Original author of the recipe
    pub author: String,
    /// Servings yielded by the recipe as written
    pub servings: u32,
    /// Optional URL source of the recipe
    pub url: Option<String>,
    /// Optional time in minutes estimated for prep
    pub prep_minutes: Option<u32>,
    /// Time in minutes estimated for cooking
    pub cook_minutes: u32,
}

/// A Recipe describing how to make a dish
#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    /// Recipe metadata
    #[serde(flatten)]
    metadata: Metadata,
    /// List of steps required to make the dish
    steps: Vec<String>,
    /// List of `Ingredient`s required to make the dish
    ingredients: Vec<Ingredient>,
}

impl Recipe {
    /// Construct a Markdown-formatted `String` representation of the Recipe
    pub fn to_markdown(&self, settings: &RenderSettings) -> String {
        let mut output = String::new();

        let servings = match settings.servings {
            Some(servings) => servings,
            None => self.metadata.servings,
        };

        if settings.meta {
            if settings.front_matter {
                write!(output, "---\n").unwrap();

                write!(output, "title: {}\n", self.metadata.name).unwrap();
                write!(output, "author: {}\n", self.metadata.author).unwrap();

                write!(output, "---\n\n").unwrap();
            }
            else {
                write!(output, "# {}\n", self.metadata.name).unwrap();

                write!(output, "**{}", self.metadata.author).unwrap();
                if let Some(url) = &self.metadata.url {
                    write!(output, " | {}", url).unwrap();
                }
                write!(output, "**\n").unwrap();
            }

            write!(output, "**{} servings", servings).unwrap();
            if let Some(prep) = &self.metadata.prep_minutes {
                write!(output, " | {} minutes prep", prep).unwrap();
            }
            write!(output, " | {} minutes cook time**\n\n", self.metadata.cook_minutes).unwrap();
        }

        if settings.ingredients {
            let multiplier = servings as f32 / self.metadata.servings as f32;

            output.push_str("## Ingredients\n");
            for ingredient in self.ingredients.iter() {
                write!(output, "*").unwrap();
                if let Some(amount) = ingredient.amount {
                    write!(output, " {}", amount * multiplier).unwrap();
                };
                if let Some(unit) = &ingredient.unit {
                    write!(output, " {}", unit).unwrap();
                }
                write!(output, " {}\n", &ingredient.name).unwrap();
            }
            output.push_str("\n");
        }

        if settings.steps {
            output.push_str("## Method\n");
            for (i, step) in self.steps.iter().enumerate() {
                write!(output, "{}. {}\n", i + 1, step).unwrap();
            }
        }

        output.push_str("\n");
        output
    }

    pub fn to_file(&self, path: &Path, settings: &RenderSettings) -> Result<(), SousError> {
        let mut output = File::create(path)?;
        write!(output, "{}", self.to_markdown(settings))?;
        Ok(())
    }

    pub fn from_yaml(content: &str) -> Result<Recipe, SousError> {
        Ok(serde_yaml::from_str(content)?)
    }

    pub fn from_file(path: &Path) -> Result<Recipe, SousError> {
        let content = fs::read_to_string(path)?;
        
        Ok(Self::from_yaml(&content)?)
    }
}

