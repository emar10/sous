use std::{fs, path::PathBuf, process};

use clap::Parser;
use sous::{Cookbook, MarkdownRenderer, Recipe, Renderer};

/// Convert YAML culinary recipes to Markdown.
#[derive(Parser, Debug)]
#[command()]
struct Args {
    /// Cookbook or single YAML-formatted recipe to convert.
    ///
    /// Single-file mode or Cookbook mode will automatically be selected based on whether INPUT
    /// points to a file or directory.
    #[arg()]
    input: PathBuf,

    /// Output path.
    ///
    /// In single-file mode, output to the specified file instead of printing to stdout. In
    /// Cookbook mode, specify the directory in which to output (will be created if necessary).
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Override number of servings.
    ///
    /// Render recipes with a specific number of servings. Ingredient amounts will be adjusted
    /// accordingly.
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..))]
    servings: Option<u32>,

    /// Use front matter instead of pure Markdown.
    ///
    /// This option enables outputting some metadata content to YAML front matter instead of using
    /// Markdown headers. Useful for static site generators.
    #[arg(short, long)]
    front_matter: bool,
}

fn main() {
    let args = Args::parse();

    let renderer = MarkdownRenderer {
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

            let rendered = renderer.render(&recipe).unwrap_or_else(|e| {
                eprintln!("failed to render recipe {file}: {e}");
                process::exit(3);
            });

            fs::write(
                &output.join(PathBuf::from(file).with_extension("md")),
                rendered,
            )
            .unwrap_or_else(|e| {
                eprintln!("failed to write file for recipe {file}: {e}");
                process::exit(2);
            });
        }
    } else {
        let recipe = Recipe::from_file(&args.input).unwrap_or_else(|e| {
            eprintln!("failed to load recipe: {e}");
            process::exit(1);
        });

        let output = renderer.render(&recipe).unwrap_or_else(|e| {
            eprintln!("failed to render recipe: {e}");
            process::exit(3);
        });

        match args.output {
            Some(file) => {
                fs::write(&file, output).unwrap_or_else(|e| {
                    eprintln!("failed to write file: {e}");
                    process::exit(2);
                });
            }
            None => {
                print!("{}", output);
            }
        }
    }
}
