use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

use crate::{Recipe, SousError};

pub struct Cookbook {
    path: PathBuf,
    recipes: Vec<String>,
}

impl Cookbook {
    pub fn open(path: &Path) -> Result<Cookbook, SousError> {
        let dir = read_dir(path)?.filter(|entry| match entry {
            Ok(entry) => match entry.path().extension() {
                Some(extension) => extension == "yml",
                None => false,
            },
            Err(_) => true,
        });

        let mut recipes: Vec<String> = Vec::new();
        for entry in dir {
            recipes.push(match entry?.file_name().into_string() {
                Ok(filename) => filename,
                Err(_) => return Err(SousError::Unknown),
            });
        }
        let path = path.to_path_buf();

        Ok(Cookbook { path, recipes })
    }

    pub fn recipes(&self) -> &Vec<String> {
        &self.recipes
    }

    pub fn load_recipe(&self, name: &str) -> Result<Recipe, SousError> {
        Ok(Recipe::from_file(&self.path.join(name))?)
    }
}
