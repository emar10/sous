//! Types for managing collections of recipes.

use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

use crate::{Recipe, SousError};

/// Directory of recipe files.
///
/// Stores both the directory's path on the filesystem and a list of found YAML files.
pub struct Cookbook {
    path: PathBuf,
    recipes: Vec<String>,
}

impl Cookbook {
    /// Open a cookbook directory.
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

    /// Get a borrowed [Vec] of available recipe names.
    pub fn recipes(&self) -> &Vec<String> {
        &self.recipes
    }

    /// Load a [Recipe] matching the given name.
    pub fn load_recipe(&self, name: &str) -> Result<Recipe, SousError> {
        Ok(Recipe::from_file(&self.path.join(name))?)
    }
}
