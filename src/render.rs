#[derive(Debug)]
pub struct RenderSettings {
    pub front_matter: bool,
    pub meta: bool,
    pub ingredients: bool,
    pub steps: bool,
    pub servings: Option<u32>,
}

impl RenderSettings {
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

