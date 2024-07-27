[![crates.io](https://img.shields.io/crates/d/embedded-hal-bus.svg)](https://crates.io/crates/embedded-hal-bus)
[![crates.io](https://img.shields.io/crates/v/embedded-hal-bus.svg)](https://crates.io/crates/embedded-hal-bus)
[![Documentation](https://docs.rs/embedded-hal-bus/badge.svg)](https://docs.rs/embedded-hal-bus)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.60+-blue.svg)

# `embedded-hal-bus`

Bus sharing utilities for [`embedded-hal`](https://crates.io/crates/embedded-hal), a Hardware Abstraction Layer (HAL) for embedded systems.

`embedded-hal` provides traits for SPI and I2C buses and devices. This crate provides hardware-independent adapters for sharing a single bus between multiple devices, compatible with the traits.

This project is developed and maintained by the [HAL team](https://github.com/rust-embedded/wg#the-hal-team).

## SPI

To support bus sharing, `embedded-hal` provides the `SpiBus` and `SpiDevice` traits. `SpiBus` represents an entire bus,
while `SpiDevice` represents a device on that bus. For further details on these traits, please consult the
[`embedded-hal` documentation](https://docs.rs/embedded-hal/latest/embedded_hal/spi/index.html).

`embedded-hal` trait implementations for microcontrollers should implement the `SpiBus` trait.
However, device drivers should use the `SpiDevice` traits, _not the `SpiBus` traits_ if at all possible
in order to allow for sharing of the bus they are connected to.

This crate provides mechanisms to connect a `SpiBus` and a `SpiDevice`.

## I2C

In the case of I2C, the same `I2c` `embedded-hal` trait represents either an entire bus, or a device on a bus. This crate
provides mechanisms to obtain multiple `I2c` instances out of a single `I2c` instance, sharing the bus.

## Optional Cargo features

- **`async`**: enable `embedded-hal-async` support.
- **`defmt-03`**: Derive `defmt::Format` from `defmt` 0.3 for enums and structs.
- **`alloc`**: enable implementations using `alloc` (for instance, `spi::RcDevice`, which makes use of `alloc::rc::Rc`)
- **`portable-atomic`**: Use `portable-atomic` to enable `atomic-device` on devices without native atomic CAS

  `portable-atomic` emulates atomic CAS functionality, allowing `embedded-hal-bus` to use `atomic-device` on hardware
  that does not natively support atomic CAS. If you enable this, you must also add `portable-atomic` to your crate with
  a feature flag such as `unsafe-assume-single-core` or `critical-section` to choose how atomic CAS is implemented.
  See https://docs.rs/portable-atomic/1.7.0/portable_atomic/#optional-features for more info.
- **`std`**: enable shared bus implementations using `std::sync::Mutex`, and implement
  `std::error::Error` for `DeviceError`.

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.60 and up. It *might*
compile with older versions but that may change in any new patch release.

See [here](../docs/msrv.md) for details on how the MSRV may be upgraded.

Enabling the `async` Cargo features requires Rust 1.75 or higher.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
