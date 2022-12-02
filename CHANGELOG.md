# Changelog

Format based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

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

[unreleased]: https://github.com/emar10/sous/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/emar10/sous/releases/tag/v0.2.0

