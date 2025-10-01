# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

- Added `core::error::Error` implementations for every custom `impl Error`
- Increased MSRV to 1.81 due to `core::error::Error`
- Bumped `defmt` to v1
- `defmt-03` feature is now named `defmt`

## [v0.4.1] - 2022-09-28

### Removed
- Unnecessary `embedded-hal` dependency.

## [v0.4.0] - 2022-09-28

Release of version of the traits extracted from embedded-hal.

[Unreleased]: https://github.com/rust-embedded/embedded-hal/compare/embedded-can-v0.4.1...HEAD
[v0.4.1]: https://github.com/rust-embedded/embedded-hal/compare/embedded-can-v0.4.0...embedded-can-v0.4.1
[v0.4.0]: https://github.com/rust-embedded/embedded-hal/tree/embedded-can-v0.4.0
