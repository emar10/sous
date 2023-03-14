use std::{
    fs,
    io::{stdin, Read},
    path::PathBuf,
    process,
};

use clap::{Parser, ValueEnum};
use sous::{Cookbook, MarkdownRenderer, Recipe, Renderer, SousError, TemplateRenderer};

#[derive(Clone, Debug, Default, ValueEnum)]
enum RenderMode {
    /// Use the built-in Markdown renderer.
    #[default]
    Markdown,
    /// Use the Tera template renderer.
    Template,
}

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

    /// Render mode selection.
    ///
    /// Markdown mode uses the built-in renderer to generate markdown files.
    /// Template mode uses the provided Tera template to render recipes.
    #[arg(short, long, value_enum, default_value_t = RenderMode::Markdown)]
    mode: RenderMode,

    /// File to use for the template renderer.
    ///
    /// If a template file is not specified, sous will read a template from standard input.
    #[arg(short, long)]
    template: Option<PathBuf>,

    /// Override number of servings (Only applies to the Markdown renderer).
    ///
    /// Render recipes with a specific number of servings. Ingredient amounts will be adjusted
    /// accordingly.
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..))]
    servings: Option<u32>,

    /// Use front matter instead of pure Markdown (Only applies to Markdown renderer).
    ///
    /// This option enables outputting some metadata content to YAML front matter instead of using
    /// Markdown headers. Useful for static site generators.
    #[arg(short, long)]
    front_matter: bool,
}

fn create_renderer(args: &Args) -> Result<Box<dyn Renderer>, SousError> {
    let renderer: Box<dyn Renderer> = match args.mode {
        RenderMode::Markdown => Box::new(MarkdownRenderer {
            servings: args.servings,
            front_matter: args.front_matter,
            ..Default::default()
        }),
        RenderMode::Template => match &args.template {
            Some(path) => Box::new(TemplateRenderer::from_path(&path)?),
            None => {
                let mut template = String::new();
                stdin().read_to_string(&mut template)?;
                Box::new(TemplateRenderer::from_str(&template)?)
            }
        },
    };

    Ok(renderer)
}

fn main() {
    let args = Args::parse();

    let renderer = create_renderer(&args).unwrap_or_else(|e| {
        eprintln!("failed to initialize renderer: {e}");
        process::exit(1);
    });

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
