//! Types for rendering recipes to other formats.

use crate::Recipe;
use std::fmt::Write;

/// A type that can render a recipe to a String.
pub trait Renderer {
    /// Construct a String representation of the provided [Recipe].
    fn render(&self, recipe: &Recipe) -> String;
}

/// Renders recipes as Markdown.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Markdown {
    /// Whether to use YAML front matter instead of pure Markdown.
    pub front_matter: bool,
    /// Whether to output meta information.
    pub skip_meta: bool,
    /// Whether to output ingredient list.
    pub skip_ingredients: bool,
    /// Whether to output the procedure.
    pub skip_steps: bool,
    /// Optionally override the serving count when outputting.
    pub servings: Option<u32>,
}

impl Markdown {
    /// Create default render self.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Renderer for Markdown {
    fn render(&self, recipe: &Recipe) -> String {
        let mut output = String::new();

        let servings = match self.servings {
            Some(servings) => servings,
            None => recipe.metadata.servings,
        };

        if !self.skip_meta {
            if self.front_matter {
                write!(output, "---\n").unwrap();

                write!(output, "title: {}\n", recipe.metadata.name).unwrap();
                write!(output, "author: {}\n", recipe.metadata.author).unwrap();

                write!(output, "---\n\n").unwrap();
            } else {
                write!(output, "# {}\n", recipe.metadata.name).unwrap();

                write!(output, "**{}", recipe.metadata.author).unwrap();
                if let Some(url) = &recipe.metadata.url {
                    write!(output, " | {}", url).unwrap();
                }
                write!(output, "**\n").unwrap();
            }

            write!(output, "**{} servings", servings).unwrap();
            if let Some(prep) = &recipe.metadata.prep_minutes {
                write!(output, " | {} minutes prep", prep).unwrap();
            }
            write!(
                output,
                " | {} minutes cook time**\n\n",
                recipe.metadata.cook_minutes
            )
            .unwrap();
        }

        if !self.skip_ingredients {
            let multiplier = servings as f32 / recipe.metadata.servings as f32;

            output.push_str("## Ingredients\n");
            for ingredient in recipe.ingredients.iter() {
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

        if !self.skip_steps {
            output.push_str("## Method\n");
            for (i, step) in recipe.steps.iter().enumerate() {
                write!(output, "{}. {}\n", i + 1, step).unwrap();
            }
        }

        output.push_str("\n");
        output
    }
}
