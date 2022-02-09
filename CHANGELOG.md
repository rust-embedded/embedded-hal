# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.2.7] - 2022-02-09

### Added

- Backport CAN interface from the upcoming 1.0 release.

## [v0.2.6] - 2021-08-03

### Added

Backported non-breaking changes from the upcoming 1.0 release:

- `Transactional` SPI interface for executing groups of SPI transactions.
- `Transactional` I2C interface for executing groups of I2C transactions.
- 10-bit addressing mode for I2C traits.
- `set_state` method for `OutputPin` using an input `PinState` value.
- `IoPin` trait for pins that can change between being inputs or outputs
  dynamically.


## [v0.2.5] - 2021-04-28

### Changed

- Updated `nb` dependency to version `0.1.3` to ensure compatibility with `nb` version `1.0`.


## [v0.2.4] - 2020-06-17

### Changed

- Fix for `dyn` traits in fmt.rs
- Remove `#![deny(warnings)]`, now imposed y CI
- Updates stm32f30x from 0.6.0 to 0.8.0
- Fix the input pin v2->v1 compatibility shim constructor, where `OldInputPin::new`
  was incorrectly implemented for `v1::OutputPin` values.


## [v0.2.3] - 2019-05-09

### Added
- A new version of the digital `OutputPin`, `StatefulOutputPin`, `ToggleableOutputPin`
  and `InputPin` traits has been added under `digital::v2`. These traits are now
  fallible and their methods now return a `Result` type as setting an output pin
  and reading an input pin could potentially fail.
  See [here](https://github.com/rust-embedded/embedded-hal/issues/95) for more info.
- Compatibility shims between `digital::v1` and `digital::v2` traits allowing v1 traits
  to be implicitly promoted to v2, and for v2 traits to be explicitly cast to v1 wrappers.

### Changed
- The current versions of the `OutputPin`, `StatefulOutputPin`, `ToggleableOutputPin`
  and `InputPin` traits have been marked as deprecated. Please use the new versions
  included in `digital::v2`.
  See [here](https://github.com/rust-embedded/embedded-hal/issues/95) for more info.


## [v0.2.2] - 2018-11-03

### Added

- Added the Rust Code of Conduct to this repository
- The first ADC-related trait. This is a simple trait for one-shot conversions.
- Iterator-based blocking write and write+read traits have been added to I2C and SPI.
- New helper constants for SPI modes.
- A new trait for a cancellable countdown.
- New traits for watchdog timer management, including startup, feeding, and stopping.

### Changed
- Updated docs to clarify I2C address bit widths and expectations.


## [v0.2.1] - 2018-05-14

### Changed

- Auto-generated documentation (docs.rs) now includes the unproven traits.

## [v0.2.0] - 2018-05-12

### Added

- A `ToggeableOutputPin` trait has been added. This trait contains a single method: `toggle` that
  can be used to toggle the state of a push-pull pin.

### Changed

- [breaking-change] The signature of `CountDown.wait` changed; it now returns `nb::Result<(),
  Void>`. Where [`Void`] is the stable alternative to the never type, `!`, provided by the stable
  [`void`] crate. Implementations of the `CountDown` trait will have to be updated to use the new
  signature. With this change this crate compiles on the stable and beta channels.

[`Void`]: https://docs.rs/void/1.0.2/void/enum.Void.html
[`void`]: https://crates.io/crates/void

- [breaking-change] the `OutputPin.is_{low,high}` methods have been moved into its own trait
  `StatefulOutputPin` and renamed to `is_set_{low,high}`.

- It has been clarified in the documentation that `OutputPin` must be implemented for push-pull
  output pins (and e.g. not for open drain output pins).

## [v0.1.3] - 2018-05-14

### Changed

- Re-export most / unchanged traits from embedded-hal v0.2.x to allow inter-operation between HAL
  implementations and drivers that are using different minor versions.

## [v0.1.2] - 2018-02-14

### Added

- Unproven `blocking::serial::*` traits

## [v0.1.1] - 2018-02-06

### Added

- Unproven `digital::InputPin` trait

## v0.1.0 - 2018-01-16

Initial release

[Unreleased]: https://github.com/rust-embedded/embedded-hal/compare/v0.2.7...v0.2.x
[v0.2.7]: https://github.com/rust-embedded/embedded-hal/compare/v0.2.6...v0.2.7
[v0.2.6]: https://github.com/rust-embedded/embedded-hal/compare/v0.2.5...v0.2.6
[v0.2.5]: https://github.com/rust-embedded/embedded-hal/compare/v0.2.4...v0.2.5
[v0.2.4]: https://github.com/rust-embedded/embedded-hal/compare/v0.2.3...v0.2.4
[v0.2.3]: https://github.com/rust-embedded/embedded-hal/compare/v0.2.2...v0.2.3
[v0.2.2]: https://github.com/rust-embedded/embedded-hal/compare/v0.2.1...v0.2.2
[v0.2.1]: https://github.com/rust-embedded/embedded-hal/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/rust-embedded/embedded-hal/compare/v0.1.2...v0.2.0
[v0.1.2]: https://github.com/rust-embedded/embedded-hal/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/rust-embedded/embedded-hal/compare/v0.1.0...v0.1.1
