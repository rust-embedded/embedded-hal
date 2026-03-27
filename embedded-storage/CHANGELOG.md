# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## Unreleased

- Add `start()` and `end()` method to the `Region` trait.
- Much faster `OverlapIterator`.
- Let `&mut` `MultiwriteNorFlash` implement `MultiwriteNorFlash`.

## [0.3.1] - 2023-12-04

- Let `&mut` `NorFlash` implement `NorFlash`.

## [0.3.0] - 2022-02-07

- Require Nor flashes error types to be convertible to generic errors.
- Provide helper functions for Nor flashes to check for generic errors.
- Add embedded-storage-async crate with async version of nor flash (requires nightly).
- Add `ErrorType` trait to avoid needing to specify error type multiple times.

## [0.2.0] - 2021-09-15

- Removed `try_` prefix from all trait methods.
- Add RMW helpers for Nor flashes, implementing `Storage` trait.

## [0.1.0] - 2021-05-18

Initial release to crates.io.

[Unreleased]: https://github.com/rust-embedded-community/embedded-storage/compare/v0.3.1...HEAD
[0.3.1]: https://github.com/rust-embedded-community/embedded-storage/releases/tag/v0.3.1
[0.3.0]: https://github.com/rust-embedded-community/embedded-storage/releases/tag/v0.3.0
[0.2.0]: https://github.com/rust-embedded-community/embedded-storage/releases/tag/v0.2.0
[0.1.0]: https://github.com/rust-embedded-community/embedded-storage/releases/tag/v0.1.0
