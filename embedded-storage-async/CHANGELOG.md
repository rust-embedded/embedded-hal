# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## Unreleased

- Add RMW helpers for Nor flashes, implementing `Storage` trait.
- Let `&mut` `MultiwriteNorFlash` implement `MultiwriteNorFlash`.

## [0.4.1] - 2023-11-28

- Let `&mut` `NorFlash` implement `NorFlash`.
- Reexport NOR flash errors from `embedded-storage`.
- Use now stabilized `async_fn_in_trait` and `impl_trait_projections`.

## [0.4.0] - 2022-12-01

### Changes
- Switch all traits to use [`async_fn_in_trait`](https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html) (AFIT). Requires `nightly-2022-11-22` or newer.
- Remove `Async` prefix in trait names.

## [0.3.0] - 2022-02-07

Initial release to crates.io.

[Unreleased]: https://github.com/rust-embedded-community/embedded-storage/compare/embedded-storage-async-v0.4.1...HEAD
[0.4.1]: https://github.com/rust-embedded-community/embedded-storage/compare/embedded-storage-async-v0.4.0...embedded-storage-async-v0.4.1
[0.4.0]: https://github.com/rust-embedded-community/embedded-storage/compare/embedded-storage-async-v0.3.0...embedded-storage-async-v0.4.0
[0.3.0]: https://github.com/rust-embedded-community/embedded-storage/releases/tag/embedded-storage-async-v0.3.0
