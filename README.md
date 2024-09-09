# `embedded-hal`

>  A Hardware Abstraction Layer (HAL) for embedded systems

This project is developed and maintained by the [HAL team](https://github.com/rust-embedded/wg#the-hal-team).

> [!IMPORTANT]
> 📣 `embedded-hal` v1.0 is now released! Check out the [announcement blog post](https://blog.rust-embedded.org/embedded-hal-v1/), the [API documentation](https://docs.rs/embedded-hal) and the [migration guide](docs/migrating-from-0.2-to-1.0.md).

## Scope

`embedded-hal` serves as a foundation for building an ecosystem of platform-agnostic drivers.
(driver meaning library crates that let a target platform interface an external device like a digital
sensor or a wireless transceiver).

The advantage of this system is that by writing the driver as a generic library on top
of `embedded-hal` driver authors can support any number of target
platforms (e.g. Cortex-M microcontrollers, AVR microcontrollers, embedded Linux, etc.).

The advantage for application developers is that by adopting `embedded-hal` they can unlock all
these drivers for their platform.

For functionality that goes beyond what is provided by `embedded-hal`, users are encouraged
to use the target platform directly. Abstractions of common functionality can be proposed to be
included into `embedded-hal` as described [in this guide](docs/how-to-add-a-new-trait.md), though.

See more about the design goals in [this documentation section](https://docs.rs/embedded-hal/latest/embedded_hal/#design-goals).

## Crates

The main  `embedded-hal` project is not tied to a specific execution model like blocking or non-blocking.

| Crate | crates.io | Docs | |
|-|-|-|-|
| [embedded-hal](./embedded-hal)       | [![crates.io](https://img.shields.io/crates/v/embedded-hal.svg)](https://crates.io/crates/embedded-hal) | [![Documentation](https://docs.rs/embedded-hal/badge.svg)](https://docs.rs/embedded-hal) | Core traits, blocking version |
| [embedded-hal-async](./embedded-hal-async) | [![crates.io](https://img.shields.io/crates/v/embedded-hal-async.svg)](https://crates.io/crates/embedded-hal-async) | [![Documentation](https://docs.rs/embedded-hal-async/badge.svg)](https://docs.rs/embedded-hal-async) | Core traits, async version |
| [embedded-hal-nb](./embedded-hal-nb)    | [![crates.io](https://img.shields.io/crates/v/embedded-hal-nb.svg)](https://crates.io/crates/embedded-hal-nb) | [![Documentation](https://docs.rs/embedded-hal-nb/badge.svg)](https://docs.rs/embedded-hal-nb) | Core traits, polling version using the `nb` crate |
| [embedded-hal-bus](./embedded-hal-bus)   | [![crates.io](https://img.shields.io/crates/v/embedded-hal-bus.svg)](https://crates.io/crates/embedded-hal-bus) | [![Documentation](https://docs.rs/embedded-hal-bus/badge.svg)](https://docs.rs/embedded-hal-bus) | Utilities for sharing SPI and I2C buses |
| [embedded-can](./embedded-can)       | [![crates.io](https://img.shields.io/crates/v/embedded-can.svg)](https://crates.io/crates/embedded-can) | [![Documentation](https://docs.rs/embedded-can/badge.svg)](https://docs.rs/embedded-can) | Controller Area Network (CAN) traits |
| [embedded-io](./embedded-io)       | [![crates.io](https://img.shields.io/crates/v/embedded-io.svg)](https://crates.io/crates/embedded-io) | [![Documentation](https://docs.rs/embedded-io/badge.svg)](https://docs.rs/embedded-io) | I/O traits (read, write, seek, etc.), blocking and nonblocking version. |
| [embedded-io-async](./embedded-io-async)       | [![crates.io](https://img.shields.io/crates/v/embedded-io-async.svg)](https://crates.io/crates/embedded-io-async) | [![Documentation](https://docs.rs/embedded-io-async/badge.svg)](https://docs.rs/embedded-io-async) | I/O traits, async version  |
| [embedded-io-adapters](./embedded-io-adapters)       | [![crates.io](https://img.shields.io/crates/v/embedded-io-adapters.svg)](https://crates.io/crates/embedded-io-adapters) | [![Documentation](https://docs.rs/embedded-io-adapters/badge.svg)](https://docs.rs/embedded-io-adapters) | Adapters between the [`embedded-io`](https://crates.io/crates/embedded-io) and [`embedded-io-async`](https://crates.io/crates/embedded-io-async) traits and other IO traits (`std`, `tokio`, `futures`...)  |

## Documents

- [Migrating from v0.2 to v1.0](docs/migrating-from-0.2-to-1.0.md).
- [How-to: add a new trait](docs/how-to-add-a-new-trait.md)
- [MSRV](docs/msrv.md)

## Implementations and drivers

For a non-exhaustive list of `embedded-hal` implementations and driver crates check the
[awesome-embedded-rust] list.

You may be able to find even more HAL implementation crates and driver crates by searching for the
[`embedded-hal-impl`], [`embedded-hal-driver`] and [`embedded-hal`][embedded-hal-kw] keywords
on crates.io.

[`embedded-hal-impl`]: https://crates.io/keywords/embedded-hal-impl
[`embedded-hal-driver`]: https://crates.io/keywords/embedded-hal-driver
[embedded-hal-kw]: https://crates.io/keywords/embedded-hal

[awesome-embedded-rust]: https://github.com/rust-embedded/awesome-embedded-rust#driver-crates

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.81 and up. It *might*
compile with older versions but that may change in any new patch release.

See [here](docs/msrv.md) for details on how the MSRV may be upgraded.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Code of Conduct

Contribution to this repository is organized under the terms of the [Rust Code of
Conduct](CODE_OF_CONDUCT.md), the maintainers of this repository, the [HAL team](https://github.com/rust-embedded/wg#the-hal-team), promise
to intervene to uphold that code of conduct.
