# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

No unreleased changes

## [v1.0.0-rc.2] - 2023-11-28

- Updated `embedded-hal` to version `1.0.0-rc.2`.
- Minor document fixes.
- Add #[inline] hints to most of `embedded-hal-async` functions.
- delay: Rename `DelayUs` to `DelayNs`
- delay: Add `DelayNs::delay_ns()`
- delay: Add default impls of `delay_ms` and `delay_us` based on `delay_ns`.
- spi: Rename `Operation::DelayUs` to `Operation::DelayNs`, with nanosecond precision.
- Use `feature()` on nightly toolchains only. This adds support for 1.75 beta and stable.

## [v1.0.0-rc.1] - 2023-08-15

- Updated `embedded-hal` to version `1.0.0-rc.1`.
- Add optional `defmt` 0.3 support.
- Remove serial traits, the replacement is the `embedded-io` crate.
- Added `+ ?Sized` to all blanket impls.
- Moved `ExclusiveDevice` to `embedded-hal-bus`.

## [v0.2.0-alpha.2] - 2023-07-04

### Added
- spi: added `Operation::DelayUs(u32)`.

### Changed
- Updated `embedded-hal` to version `1.0.0-alpha.11`.
- spi: removed redundant lifetime annotations. Note that recent nightlies care about them and require impls to match, so you might have to adjust them.

### Removed
- spi: removed read-only and write-only traits.

## [v0.2.0-alpha.1] - 2023-04-04

### Added
- Added a `serial::Write` trait.

### Changed
- Updated `embedded-hal` to version `1.0.0-alpha.10`.
- delay: make infallible.
- i2c: remove `_iter()` methods.
- i2c: add default implementations for all methods based on `transaction()`.
- spi: SpiDevice transaction now takes an operation slice instead of a closure

## [v0.2.0-alpha.0] - 2022-11-23

- Switch all traits to use [`async_fn_in_trait`](https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html) (AFIT). Requires `nightly-2022-11-22` or newer.

## [v0.1.0-alpha.3] - 2022-10-26

- Fix build on newer Rust nightlies.

## [v0.1.0-alpha.2] - 2022-09-28

### Added
- spi: added a transaction helper macro as a workaround for the raw pointer workaround.

### Changed
- Updated `embedded-hal` to version `1.0.0-alpha.9`.

## [v0.1.0-alpha.1] - 2022-05-24

### Changed

- spi: device helper methods (`read`, `write`, `transfer`...) are now default methods in `SpiDevice` instead of an `SpiDeviceExt` extension trait.
- spi: the `SpiDevice::transaction` closure now gets a raw pointer to the `SpiBus` to work around Rust borrow checker limitations.
- spi: the `SpiDevice` trait is now unsafe to implement due to the raw pointer workaround.


## [v0.1.0-alpha.0] - 2022-04-17

First release to crates.io


[Unreleased]: https://github.com/rust-embedded/embedded-hal/compare/embedded-hal-async-v1.0.0-rc.2...HEAD
[v1.0.0-rc.2]: https://github.com/rust-embedded/embedded-hal/compare/embedded-hal-async-v1.0.0-rc.1...embedded-hal-async-v1.0.0-rc.2
[v1.0.0-rc.1]: https://github.com/rust-embedded/embedded-hal/compare/embedded-hal-async-v0.2.0-alpha.2...embedded-hal-async-v1.0.0-rc.1
[v0.2.0-alpha.2]: https://github.com/rust-embedded/embedded-hal/compare/embedded-hal-async-v0.2.0-alpha.1...embedded-hal-async-v0.2.0-alpha.2
[v0.2.0-alpha.1]: https://github.com/rust-embedded/embedded-hal/compare/embedded-hal-async-v0.2.0-alpha.0...embedded-hal-async-v0.2.0-alpha.1
[v0.2.0-alpha.0]: https://github.com/rust-embedded/embedded-hal/compare/embedded-hal-async-v0.1.0-alpha.3...embedded-hal-async-v0.2.0-alpha.0
[v0.1.0-alpha.3]: https://github.com/rust-embedded/embedded-hal/compare/embedded-hal-async-v0.1.0-alpha.2...embedded-hal-async-v0.1.0-alpha.3
[v0.1.0-alpha.2]: https://github.com/rust-embedded/embedded-hal/compare/embedded-hal-async-v0.1.0-alpha.1...embedded-hal-async-v0.1.0-alpha.2
[v0.1.0-alpha.1]: https://github.com/rust-embedded/embedded-hal/compare/embedded-hal-async-v0.1.0-alpha.0...embedded-hal-async-v0.1.0-alpha.1
[v0.1.0-alpha.0]: https://github.com/rust-embedded/embedded-hal/tree/embedded-hal-async-v0.1.0-alpha.0
