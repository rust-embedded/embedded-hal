# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Added

- A fallible version of the `OutputPin` trait has been added. It is a version
  where the methods return a `Result` type as setting an output pin could potentially fail.
  The current `OutputPin` trait version is now marked as deprecated. The fallible version
  is available throught the `use-fallible-digital-traits` feature. After this release,
  the fallible version of `OutputPin` will be made the default and the infallible one
  will only be available if activating the `use-infallible-digital-traits` feature.
  The infallible version of the trait will be removed in the relase after that one.
  See [here](https://github.com/rust-embedded/embedded-hal/issues/95) for more info.

### Changed

- [breaking-change] The unproven `InputPin` trait methods are now fallible, as reading an input
  could potentially fail. Similarly, the methods in the unproven `ToggleableOutputPin` trait
  are now also fallible.
  See [here](https://github.com/rust-embedded/embedded-hal/issues/95) for more info.

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

[Unreleased]: https://github.com/japaric/embedded-hal/compare/v0.2.1...HEAD
[v0.2.1]: https://github.com/japaric/embedded-hal/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/japaric/embedded-hal/compare/v0.1.2...v0.2.0
[v0.1.2]: https://github.com/japaric/embedded-hal/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/japaric/embedded-hal/compare/v0.1.0...v0.1.1
