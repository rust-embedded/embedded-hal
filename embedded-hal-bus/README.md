[![crates.io](https://img.shields.io/crates/d/embedded-hal-bus.svg)](https://crates.io/crates/embedded-hal-bus)
[![crates.io](https://img.shields.io/crates/v/embedded-hal-bus.svg)](https://crates.io/crates/embedded-hal-bus)
[![Documentation](https://docs.rs/embedded-hal-bus/badge.svg)](https://docs.rs/embedded-hal-bus)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.54+-blue.svg)

# `embedded-hal-bus`

Bus/Device connection mechanisms for [`embedded-hal`], a Hardware Abstraction Layer (HAL) for embedded systems.

It is possible to connect several peripherals to a bus like SPI or I2C.
To support this, `embedded-hal` provides the `SpiBus` and `SpiDevice` traits in the case of SPI, for example.

`embedded-hal` trait implementations for microcontrollers should implement the `...Bus` traits.
However, device drivers should use the `...Device` traits, _not the `...Bus` traits_ if at all possible
in order to allow for sharing of the bus they are connected to.

This crate provides mechanisms to connect a `...Bus` and a `...Device`.

For further details on these traits, please consult the [`embedded-hal` documentation](https://docs.rs/embedded-hal).

This project is developed and maintained by the [HAL team][team].

## [API reference]

[API reference]: https://docs.rs/embedded-hal-bus

## Minimum Supported Rust Version (MSRV)


This crate is guaranteed to compile on stable Rust 1.54 and up. It *might*
compile with older versions but that may change in any new patch release.

See [here](../docs/msrv.md) for details on how the MSRV may be upgraded.


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

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, the [HAL team][team], promises
to intervene to uphold that code of conduct.

[CoC]: ../CODE_OF_CONDUCT.md
[team]: https://github.com/rust-embedded/wg#the-hal-team
[`embedded-hal`]: https://crates.io/crates/embedded-hal