//! Types for rendering recipes to other formats.

use tera::{Context, Tera};

use crate::{Recipe, SousError};
use std::{
    fmt::Write,
    path::{Path, PathBuf},
};

/// A type that can render a recipe to a String.
pub trait Renderer {
    /// Construct a String representation of the provided [Recipe].
    fn render(&self, recipe: &Recipe) -> Result<String, SousError>;
}

/// Renders recipes in Markdown format.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct MarkdownRenderer {
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

impl MarkdownRenderer {
    /// Create default render self.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Renderer for MarkdownRenderer {
    fn render(&self, recipe: &Recipe) -> Result<String, SousError> {
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
        Ok(output)
    }
}

/// Renders recipes using a [Tera] template.
#[derive(Clone, Debug)]
pub struct TemplateRenderer {
    env: Tera,
}

impl Renderer for TemplateRenderer {
    fn render(&self, recipe: &Recipe) -> Result<String, SousError> {
        let ctx = Context::from_serialize(recipe)?;
        Ok(self.env.render("template", &ctx)?)
    }
}

impl TemplateRenderer {
    /// Create a new renderer using the provided template file.
    pub fn from_path(path: &Path) -> Result<Self, SousError> {
        let mut env = Tera::default();
        let path = PathBuf::from(path);

        env.add_template_file(&path, Some("template"))?;

        Ok(TemplateRenderer { env })
    }

    /// Create a new renderer using the provided raw string.
    pub fn from_str<S: AsRef<str>>(template: S) -> Result<Self, SousError> {
        let mut env = Tera::default();

        env.add_raw_template("template", template.as_ref())?;

        Ok(TemplateRenderer { env })
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
    fn test_md_render() {
        let recipe = gen_recipe();

        let renderer = MarkdownRenderer::new();
        let md = renderer.render(&recipe).unwrap();

        assert!(md.contains("# test recipe\n**test author**\n**1 servings | 1 minutes cook time**"));
        assert!(md.contains("## Ingredients\n* 1 test ingredient"));
        assert!(md.contains("## Method\n1. Step one"));
    }

    #[test]
    fn test_md_render_front_matter() {
        let recipe = gen_recipe();

        let renderer = MarkdownRenderer {
            front_matter: true,
            ..Default::default()
        };
        let md = renderer.render(&recipe).unwrap();

        assert!(md.contains("---\ntitle: test recipe\nauthor: test author\n---"));
    }

    #[test]
    fn test_md_render_servings() {
        let recipe = gen_recipe();

        let renderer = MarkdownRenderer {
            servings: Some(2),
            ..Default::default()
        };
        let md = renderer.render(&recipe).unwrap();

        assert!(md.contains("2 servings"));
        assert!(md.contains("2 test ingredient"));
    }

    #[test]
    fn test_template_render_metadata() {
        let recipe = gen_recipe();
        let template =
            "{{ name }}\n{{ author }}\n{{ servings }} serving \n{{ cook_minutes }} minute";

        let renderer = TemplateRenderer::from_str(template).unwrap();
        let output = renderer.render(&recipe).unwrap();

        assert!(output.contains("test recipe"));
        assert!(output.contains("test author"));
        assert!(output.contains("1 serving"));
        assert!(output.contains("1 minute"));
    }

    #[test]
    fn test_template_render_ingredients() {
        let recipe = gen_recipe();
        let template = "{% for ingredient in ingredients %}\n{{ ingredient.name }}\n{% endfor %}";

        let renderer = TemplateRenderer::from_str(template).unwrap();
        let output = renderer.render(&recipe).unwrap();

        assert!(output.contains("test ingredient"));
    }

    #[test]
    fn test_template_render_steps() {
        let recipe = gen_recipe();
        let template = "{% for step in steps %}\n{{ step }}\n{% endfor %}";

        let renderer = TemplateRenderer::from_str(template).unwrap();
        let output = renderer.render(&recipe).unwrap();

        assert!(output.contains("Step one"));
    }
}
