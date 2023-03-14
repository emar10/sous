# Changelog

Format based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased] (Expected version [0.4.0])

...

## Version [0.3.0]

### Added

- New dependency: [Tera](https://crates.io/crates/tera/).
- `SousError::TemplateError` wraps the error type from Tera.
- `TemplateRenderer` allows recipes to be rendered using Tera templates,
  provided either as paths or raw strings.
- New CLI options: `--mode` and `--template` for templated output.

### Changed

- Use only major versions for dependencies where possible in `Cargo.toml` (e.g.
  `thiserror = "1.0.38"` becomes `1`).
- Flattened public API in `lib.rs` (e.g. clients can use `sous::Renderer` instead
  of `sous::render::Renderer`).
- `Renderer::render` now returns a Result, allowing implementations to notify
  clients of failures during rendering.
- `Markdown` has been renamed to `MarkdownRenderer` for clarity.

## Version [0.2.0]

### Added

- An initial GitHub Actions workflow to ensure the project compiles and is
  formatted reasonably.
- This changelog!
- Documentation comments for the public API.
- New `Renderer` trait for types that can construct string representations of
  recipes.
- A basic test suite for the `recipe` and `render` modules.
- A GitHub Actions workflow to automagically publish to crates.io on release.

### Changed

- Recipe data structs have been split into their own modules and implement
  common standard library traits.
- Fields in `Recipe` are now public.
- Refined help text for the CLI.
- Populated `README.md`.
- `RenderSettings` has been renamed to `Markdown`, and implements `Renderer`.

### Removed

- `Recipe::to_markdown()` and `Recipe::to_file()` (see the `Markdown`
  renderer).

### Fixed

- `cargo fmt` was not run before version 0.1.0.

[unreleased]: https://github.com/emar10/sous/compare/v0.3.0...HEAD
[0.2.0]: https://github.com/emar10/sous/releases/tag/v0.2.0
[0.3.0]: https://github.com/emar10/sous/releases/tag/v0.3.0
[0.4.0]: https://github.com/emar10/sous/milestone/3

