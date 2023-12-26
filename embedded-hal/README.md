[![crates.io](https://img.shields.io/crates/d/embedded-hal.svg)](https://crates.io/crates/embedded-hal)
[![crates.io](https://img.shields.io/crates/v/embedded-hal.svg)](https://crates.io/crates/embedded-hal)
[![Documentation](https://docs.rs/embedded-hal/badge.svg)](https://docs.rs/embedded-hal)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.60+-blue.svg)

# `embedded-hal`

>  A Hardware Abstraction Layer (HAL) for embedded systems

This project is developed and maintained by the [HAL team](https://github.com/rust-embedded/wg#the-hal-team).

## Companion crates

The main `embedded-hal` crate contains only blocking traits, where the operation is done
synchronously before returning. Check out the following crates, which contain versions
of the traits for other execution models:

- [`embedded-hal-async`](https://docs.rs/embedded-hal-async): async/await-based.
- [`embedded-hal-nb`](https://docs.rs/embedded-hal-nb): polling-based, using the `nb` crate.

The [`embedded-hal-bus`](https://docs.rs/embedded-hal-bus) crate provides utilities for sharing
SPI and I2C buses.

Additionally, more domain-specific traits are available in separate crates:
- [`embedded-can`](https://docs.rs/embedded-can): Controller Area Network (CAN)
- [`embedded-io`](https://docs.rs/embedded-io): I/O byte streams (like `std::io`, but `no-std`-compatible).

## Serial/UART traits

There is no serial traits in `embedded-hal`. Instead, use [`embedded-io`](https://crates.io/crates/embedded-io).
A serial port is essentially a byte-oriented stream, and that's what `embedded-io` models. Sharing the traits
with all byte streams has some advantages. For example, it allows generic code providing a command-line interface
or a console to operate either on hardware serial ports or on virtual ones like Telnet or USB CDC-ACM.

## Design goals

The HAL

- Must *erase* device specific details. Neither register, register blocks, nor magic values should
appear in the API.

- Must be generic *within* a device and *across* devices. The API to use a serial interface must
be the same regardless of whether the implementation uses the USART1 or UART4 peripheral of a
device or the UART0 peripheral of another device.

- Where possible must *not* be tied to a specific asynchronous model. The API should be usable
in blocking mode, with the `futures` model, with an async/await model or with a callback model.
(cf. the [`nb`](https://docs.rs/nb) crate)

- Must be minimal, and thus easy to implement and zero cost, yet highly composable. People that
want higher level abstraction should *prefer to use this HAL* rather than *re-implement*
register manipulation code.

- Serve as a foundation for building an ecosystem of platform-agnostic drivers. Here driver
means a library crate that lets a target platform interface an external device like a digital
sensor or a wireless transceiver. The advantage of this system is that by writing the driver as
a generic library on top of `embedded-hal` driver authors can support any number of target
platforms (e.g. Cortex-M microcontrollers, AVR microcontrollers, embedded Linux, etc.). The
advantage for application developers is that by adopting `embedded-hal` they can unlock all
these drivers for their platform.

- Trait methods must be fallible so that they can be used in any possible situation.
Nevertheless, HAL implementations can additionally provide infallible versions of the same methods
if they can never fail in their platform. This way, generic code can use the fallible abstractions
provided here but platform-specific code can avoid fallibility-related boilerplate if possible.

## Out of scope

- Initialization and configuration stuff like "ensure this serial interface and that SPI
interface are not using the same pins". The HAL will focus on *doing I/O*.

## Platform agnostic drivers

You can find platform-agnostic drivers built on top of `embedded-hal` on crates.io by [searching
for the *embedded-hal* keyword](https://crates.io/keywords/embedded-hal).

If you are writing a platform-agnostic driver yourself you are highly encouraged to [add the
embedded-hal keyword](https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata)
to your crate before publishing it!

## Optional Cargo features

- **`defmt-03`**: Derive `defmt::Format` from `defmt` 0.3 for enums and structs.

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.60 and up. It *might*
compile with older versions but that may change in any new patch release.

See [here](../docs/msrv.md) for details on how the MSRV may be upgraded.

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
