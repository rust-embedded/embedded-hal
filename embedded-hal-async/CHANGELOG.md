# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]


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


[Unreleased]: https://github.com/rust-embedded/embedded-hal/compare/embedded-hal-async-v0.1.0-alpha.2...HEAD
[v0.1.0-alpha.2]: https://github.com/rust-embedded/embedded-hal/compare/embedded-hal-async-v0.1.0-alpha.1...embedded-hal-async-v0.1.0-alpha.2
[v0.1.0-alpha.1]: https://github.com/rust-embedded/embedded-hal/compare/embedded-hal-async-v0.1.0-alpha.0...embedded-hal-async-v0.1.0-alpha.1
[v0.1.0-alpha.0]: https://github.com/rust-embedded/embedded-hal/tree/embedded-hal-async-v0.1.0-alpha.0
