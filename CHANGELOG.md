# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v1.0.0-alpha.8] - 2022-04-15

*** This is (also) an alpha release with breaking changes (sorry) ***

### Changed
- The Minimum Supported Rust Version (MSRV) is now 1.54.0
- `spi`: unify all traits into `SpiReadBus`, `SpiWriteBus` and `SpiBus` (read-write).
- `spi`: Add `SpiDevice` trait to represent a single device in a (possibly shared) bus, with managed chip-select (CS) pin.
- `spi`: Clarify that implementations are allowed to return before operations are finished, add `flush` to wait until finished.

### Removed
- ADC traits: `adc::nb::OneShot` and `adc::nb::Channel`.

## [v1.0.0-alpha.7] - 2022-02-09

*** This is (also) an alpha release with breaking changes (sorry) ***

### Added
- `Error` traits for CAN, SPI, I2C and Serial are implemented for `Infallible`.

### Fixed
- Fixed blanket impl of `DelayUs` not covering the `delay_ms` method.

### Changed
- `spi`: traits now enforce all impls on the same struct (eg `Transfer` and `Write`) have the same `Error` type.
- `digital`: traits now enforce all impls on the same struct have the same `Error` type.
- `serial`: traits now enforce all impls on the same struct have the same `Error` type.
- `i2c`: traits now enforce all impls on the same struct have the same `Error` type.
- `i2c`: unify all traits into a single `I2c` trait.

### Removed
- Traits with unconstrained associated types and their modules (See: [#324], [#354]):
    - `capture::Capture`
    - `pwm::Pwm`
    - `pwm::PwmPin`
    - `qei::Qei`
    - `timer::Cancel`
    - `timer::CountDown`
    - `timer::Periodic`
    - `watchdog::Disable`
    - `watchdog::Enable`
    - `watchdog::Watchdog`


[#324]: https://github.com/rust-embedded/embedded-hal/pull/324/
[#354]: https://github.com/rust-embedded/embedded-hal/pull/354

## [v1.0.0-alpha.6] - 2021-11-19

*** This is (also) an alpha release with breaking changes (sorry) ***

### Changed
- Use `u8` as default SPI as Serial Word type
- The Minimum Supported Rust Version (MSRV) is now 1.46.0
- Require all SPI and Serial word types to be `Copy`.

### Added
- Added `Can` Controller Area Network traits.
- `Error` traits for SPI, I2C and Serial traits. The error types used in those must
  implement these `Error` traits, which implies providing a conversion to a common
  set of error kinds. Generic drivers using these interfaces can then convert the errors
  to this common set to act upon them.

### Removed
- Removed `DelayMs` in favor of `DelayUs` with `u32` as type for clarity.

## [v1.0.0-alpha.5] - 2021-09-11

*** This is (also) an alpha release with breaking changes (sorry) ***

### Added
- Added `IoPin` trait for pins that can change between being inputs or outputs
  dynamically.
- Added `Debug` to all spi mode types.
- Add impls of all traits for references (`&T` or `&mut T` depending on the trait) when `T` implements the trait.
- SPI: Added blocking `Read` trait and `Read` transactional operation
- SPI: Added blocking `Transfer` trait with separate buffers (single-buffer `Transfer` has been renamed `TransferInplace`)

### Changed
- Swap PWM channel arguments to references
- All trait methods have been renamed to remove the `try_` prefix (i.e. `try_send` -> `send`) for consistency.
- Moved all traits into two sub modules for each feature depending on the execution model: `blocking` and `nb` (non-blocking). For example, the spi traits can now be found under `embedded_hal::spi::blocking` or `embedded_hal::spi::nb`.
- Execution-model-independent definitions have been moved into the feature module. For example, SPI `Phase` is now defined in `embedded_hal::spi::Phase`. For convenience, these definitions are reexported in both of its blocking and non-blocking submodules.
- Re-export `nb::{block!, Error, Result}` to avoid version mismatches. These should be used instead of
  importing the `nb` crate directly in dependent crates.
- `blocking::Serial`: renamed `bwrite_all` to `write`, `bflush` to `flush.
- Removed `prelude` to avoid method name conflicts between different flavors (blocking, nb) of the same trait. Traits must now be manually imported.
- Removed the various `Default` marker traits.
- Removed `&[W]` returned slice in `spi::blocking::Transfer`.
- Require all associated error types to implement `core::fmt::Debug`.

### Removed
- Removed random number generation (`rng`) traits in favor of [rand_core](https://crates.io/crates/rand_core).

## [v1.0.0-alpha.4] - 2020-11-11

### Fixed
- Support for I2C addressing modes in `Transactional` I2C traits.

## [v1.0.0-alpha.3] - 2020-11-04

### Added
- `Transactional` SPI interface for executing groups of SPI transactions.
- `Transactional` I2C interface for executing groups of I2C transactions.


## [v1.0.0-alpha.2] - 2020-10-16

*** This is (also) an alpha release with breaking changes (sorry) ***

### Added
- 10-bit addressing mode for I2C traits.
- `try_set_state` method for `OutputPin` using an input `PinState` value.

### Changed

- I2C addressing modes are now selected via an `AddressMode` type parameter.
  The trait features implementations for marker types `SevenBitAddress` and
  `TenBitAddress`. `SevenBitAddress` is the default mode so this is not a
  breaking change.
- The method `try_write` from the trait `blocking::i2c::WriteIter` trait
  has been renamed `try_write_iter` for consistency.
- Updated `nb` dependency to version `1`.
- The watchdog API now uses move semantics. See [PR](https://github.com/rust-embedded/embedded-hal/pull/222).
- The ADC `Channel` trait now uses a stateful method to get the IDs.

## [v1.0.0-alpha.1] - 2020-06-16

*** This is an alpha release with breaking changes (sorry) ***

### Added
- A nonblocking trait for interfacing with random number generation hardware.

### Changed
- All traits have been marked as proven (`unproven` feature has been removed).
- All trait methods have been made fallible.
- All trait methods have been renamed `try_*` (i.e. `try_send`) for consistency.
- The `Capture`, `Pwm`, `PwmPin` and `Qei` traits have been moved into their own
  `capture`, `pwm` and `qei` modules for consistency.
- Void has been replaced with `core::convert::Infallible` which should be used
  in trait implementations where methods cannot fail.
- A new [process](https://github.com/rust-embedded/embedded-hal#how-to-add-a-new-trait)
  has been adopted for the addition of traits to the embedded-hal.
- The ADC `Channel` trait now uses a constant to represent the IDs.
- The minimum supported Rust version is 1.35 due to [this issue](https://github.com/rust-lang/rust/issues/54973).

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

[Unreleased]: https://github.com/rust-embedded/embedded-hal/compare/v1.0.0-alpha.7...HEAD
[v1.0.0-alpha.7]: https://github.com/rust-embedded/embedded-hal/compare/v1.0.0-alpha.6...v1.0.0-alpha.7
[v1.0.0-alpha.6]: https://github.com/rust-embedded/embedded-hal/compare/v1.0.0-alpha.5...v1.0.0-alpha.6
[v1.0.0-alpha.5]: https://github.com/rust-embedded/embedded-hal/compare/v1.0.0-alpha.4...v1.0.0-alpha.5
[v1.0.0-alpha.4]: https://github.com/rust-embedded/embedded-hal/compare/v1.0.0-alpha.3...v1.0.0-alpha.4
[v1.0.0-alpha.3]: https://github.com/rust-embedded/embedded-hal/compare/v1.0.0-alpha.2...v1.0.0-alpha.3
[v1.0.0-alpha.2]: https://github.com/rust-embedded/embedded-hal/compare/v1.0.0-alpha.1...v1.0.0-alpha.2
[v1.0.0-alpha.1]: https://github.com/rust-embedded/embedded-hal/compare/v0.2.3...v1.0.0-alpha.1
[v0.2.3]: https://github.com/rust-embedded/embedded-hal/compare/v0.2.2...v0.2.3
[v0.2.2]: https://github.com/rust-embedded/embedded-hal/compare/v0.2.1...v0.2.2
[v0.2.1]: https://github.com/rust-embedded/embedded-hal/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/rust-embedded/embedded-hal/compare/v0.1.2...v0.2.0
[v0.1.2]: https://github.com/rust-embedded/embedded-hal/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/rust-embedded/embedded-hal/compare/v0.1.0...v0.1.1
