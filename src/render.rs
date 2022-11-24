//! Types for rendering recipes to other formats.

/// Options for rendering a [Recipe](crate::Recipe).
#[derive(Debug)]
pub struct RenderSettings {
    /// Whether to use YAML front matter instead of pure Markdown.
    pub front_matter: bool,
    /// Whether to output meta information.
    pub meta: bool,
    /// Whether to output ingredient list.
    pub ingredients: bool,
    /// Whether to output the procedure.
    pub steps: bool,
    /// Optionally override the serving count when outputting.
    pub servings: Option<u32>,
}

impl RenderSettings {
    /// Create default render settings.
    pub fn new() -> Self {
        Self {
            front_matter: false,
            meta: true,
            ingredients: true,
            steps: true,
            servings: None,
        }
    }
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self::new()
    }
}
