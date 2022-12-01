//! Types for rendering recipes to other formats.

use crate::Recipe;
use std::fmt::Write;

/// A type that can render a recipe to a String.
pub trait Renderer {
    /// Construct a String representation of the provided [Recipe].
    fn render(&self, recipe: &Recipe) -> String;
}

/// Renders recipes in Markdown format.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Ingredient;
    use crate::Metadata;

    fn gen_recipe() -> Recipe {
        Recipe {
            metadata: Metadata {
                name: "test recipe".to_string(),
                author: "test author".to_string(),
                servings: 1,
                cook_minutes: 1,
                ..Default::default()
            },
            steps: vec!["Step one".to_string()],
            ingredients: vec![Ingredient {
                name: "test ingredient".to_string(),
                amount: Some(1.0),
                ..Default::default()
            }],
        }
    }

    #[test]
    fn test_render() {
        let recipe = gen_recipe();

        let renderer = Markdown::new();
        let md = renderer.render(&recipe);

        assert!(md.contains("# test recipe\n**test author**\n**1 servings | 1 minutes cook time**"));
        assert!(md.contains("## Ingredients\n* 1 test ingredient"));
        assert!(md.contains("## Method\n1. Step one"));
    }

    #[test]
    fn test_render_front_matter() {
        let recipe = gen_recipe();

        let renderer = Markdown {
            front_matter: true,
            ..Default::default()
        };
        let md = renderer.render(&recipe);

        assert!(md.contains("---\ntitle: test recipe\nauthor: test author\n---"));
    }

    #[test]
    fn test_render_servings() {
        let recipe = gen_recipe();

        let renderer = Markdown {
            servings: Some(2),
            ..Default::default()
        };
        let md = renderer.render(&recipe);

        assert!(md.contains("2 servings"));
        assert!(md.contains("2 test ingredient"));
    }
}
