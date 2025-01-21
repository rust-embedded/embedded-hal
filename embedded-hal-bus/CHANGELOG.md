# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

- Your change here!

## [v0.3.0] - 2025-01-21

- Added the `alloc` feature.
- Added a new `RcDevice` for I2C and SPI, a reference-counting equivalent to `RefCellDevice`.
- Migrated `std` feature-gated `std::error::Error` implementations to `core::error::Error`
- Increased MSRV to 1.81 due to `core::error::Error`

## [v0.2.0] - 2024-04-23

- Added a new `AtomicDevice` for I2C and SPI to enable bus sharing across multiple contexts.
- SPI shared bus constructors now set `CS` high, to prevent sharing issues if it was low.

## [v0.1.0] - 2023-12-28

- Updated `embedded-hal` to version `1.0.0`.

## [v0.1.0-rc.3] - 2023-12-14

- Updated `embedded-hal` to version `1.0.0-rc.3`.

## [v0.1.0-rc.2] - 2023-11-28

- Updated `embedded-hal(-async)` to version `1.0.0-rc.2`.
- Minor document fixes.
- Add #[inline] hints to most of `embedded-hal-bus` functions.
- Use `feature()` on nightly toolchains only. This adds async support for 1.75 beta and stable.

## [v0.1.0-rc.1] - 2023-08-15

- Updated `embedded-hal`, `embedded-hal-async` to version `1.0.0-rc.1`.
- The Minimum Supported Rust Version (MSRV) is now 1.60.0
- Added `embedded-hal-async` support to SPI `ExclusiveDevice`.
- Added methods to access the inner bus to SPI `ExclusiveDevice`.
- Add optional `defmt` 0.3 support.

## [v0.1.0-alpha.3] - 2023-07-04

### Changed
- Updated `embedded-hal` to version `1.0.0-alpha.11`.


## [v0.1.0-alpha.2] - 2023-04-04

### Changed
- Updated `embedded-hal` to version `1.0.0-alpha.10`.

### Added
- i2c: add bus sharing implementations.
- spi: add bus sharing implementations.

## [v0.1.0-alpha.1] - 2022-09-28

### Changed
- Updated `embedded-hal` to version `1.0.0-alpha.9`.

## [v0.1.0-alpha.0] - 2022-08-17

First release to crates.io

[Unreleased]: https://github.com/rust-embedded/embedded-hal/compare/embedded-hal-bus-v0.3.0...HEAD
[v0.3.0]: https://github.com/rust-embedded/embedded-hal/compare/embedded-hal-bus-v0.2.0...embedded-hal-bus-v0.3.0
[v0.2.0]: https://github.com/rust-embedded/embedded-hal/compare/embedded-hal-bus-v0.1.0...embedded-hal-bus-v0.2.0
[v0.1.0]: https://github.com/rust-embedded/embedded-hal/compare/embedded-hal-bus-v0.1.0-rc.3...embedded-hal-bus-v0.1.0
[v0.1.0-rc.3]: https://github.com/rust-embedded/embedded-hal/compare/embedded-hal-bus-v0.1.0-rc.2...embedded-hal-bus-v0.1.0-rc.3
[v0.1.0-rc.2]: https://github.com/rust-embedded/embedded-hal/compare/embedded-hal-bus-v0.1.0-rc.1...embedded-hal-bus-v0.1.0-rc.2
[v0.1.0-rc.1]: https://github.com/rust-embedded/embedded-hal/compare/embedded-hal-bus-v0.1.0-alpha.3...embedded-hal-bus-v0.1.0-rc.1
[v0.1.0-alpha.3]: https://github.com/rust-embedded/embedded-hal/compare/embedded-hal-bus-v0.1.0-alpha.2...embedded-hal-bus-v0.1.0-alpha.3
[v0.1.0-alpha.2]: https://github.com/rust-embedded/embedded-hal/compare/embedded-hal-bus-v0.1.0-alpha.1...embedded-hal-bus-v0.1.0-alpha.2
[v0.1.0-alpha.1]: https://github.com/rust-embedded/embedded-hal/compare/embedded-hal-bus-v0.1.0-alpha.0...embedded-hal-bus-v0.1.0-alpha.1
[v0.1.0-alpha.0]: https://github.com/rust-embedded/embedded-hal/tree/embedded-hal-bus-v0.1.0-alpha.0
