# Changelog

Format based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased] (Expected version [0.2.0])

### Added

- An initial GitHub Actions workflow to ensure the project compiles and is
  formatted reasonably. (#1)

### Changed

- Recipe data structs have been split into their own modules and implement
  common standard library traits. (#18)
- Fields in `Recipe` are now public. (#18)

### Fixed

- `cargo fmt` was not run before version 0.1.0. (#10)

[unreleased]: https://github.com/emar10/sous/compare/v0.1.0...HEAD
[0.2.0]: https://github.com/emar10/sous/milestone/1

