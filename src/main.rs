use std::{fs, path::PathBuf, process};

use clap::Parser;

use sous::{self, cookbook::Cookbook, render::RenderSettings, Recipe};

#[derive(Parser, Debug)]
#[command()]
struct Args {
    /// Cookbook or single YAML-formatted recipe to convert
    #[arg()]
    input: PathBuf,

    /// Output file/directory
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Override number of servings
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..))]
    servings: Option<u32>,

    #[arg(short, long)]
    front_matter: bool,
}

fn main() {
    let args = Args::parse();

    let settings = RenderSettings {
        servings: args.servings,
        front_matter: args.front_matter,
        ..Default::default()
    };

    if args.input.is_dir() {
        let cookbook = Cookbook::open(&args.input).unwrap_or_else(|e| {
            eprintln!("failed to open cookbook: {e}");
            process::exit(1);
        });

        let output = args.output.unwrap_or_else(|| PathBuf::from("render"));
        if !output.is_dir() {
            fs::create_dir(&output).unwrap_or_else(|e| {
                eprintln!("failed to open output directory: {e}");
            });
        }

        for file in cookbook.recipes() {
            let recipe = cookbook.load_recipe(file).unwrap_or_else(|e| {
                eprintln!("failed to load recipe {file}: {e}");
                process::exit(1);
            });

            recipe
                .to_file(
                    &output.join(PathBuf::from(file).with_extension("md")),
                    &settings,
                )
                .unwrap_or_else(|e| {
                    eprintln!("failed to write file for recipe {file}: {e}");
                })
        }
    } else {
        let recipe = Recipe::from_file(&args.input).unwrap_or_else(|e| {
            eprintln!("failed to load recipe: {e}");
            process::exit(1);
        });

        match args.output {
            Some(file) => {
                recipe.to_file(&file, &settings).unwrap_or_else(|e| {
                    eprintln!("failed to write file: {e}");
                    process::exit(2);
                });
            }
            None => {
                print!("{}", recipe.to_markdown(&settings));
            }
        }
    }
}
