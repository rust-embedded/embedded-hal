[![crates.io](https://img.shields.io/crates/d/embedded-hal.svg)](https://crates.io/crates/embedded-hal)
[![crates.io](https://img.shields.io/crates/v/embedded-hal.svg)](https://crates.io/crates/embedded-hal)
[![Documentation](https://docs.rs/embedded-hal/badge.svg)](https://docs.rs/embedded-hal)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.46+-blue.svg)

# `embedded-hal`

>  A Hardware Abstraction Layer (HAL) for embedded systems

This project is developed and maintained by the [HAL team][team].

## [API reference]

[API reference]: https://docs.rs/embedded-hal

## Scope

`embedded-hal` serves as a foundation for building an ecosystem of platform agnostic drivers.
(driver meaning library crates that let a target platform interface an external device like a digital
sensor or a wireless transceiver).

The advantage of this system is that by writing the driver as a generic library on top
of `embedded-hal` driver authors can support any number of target
platforms (e.g. Cortex-M microcontrollers, AVR microcontrollers, embedded Linux, etc.).

The advantage for application developers is that by adopting `embedded-hal` they can unlock all
these drivers for their platform.

`embedded-hal` is not tied to a specific execution model like blocking or non-blocking.

For functionality that goes beyond what is provided by `embedded-hal`, users are encouraged
to use the target platform directly. Abstractions of common functionality can be proposed to be
included into `embedded-hal` as described [in this guide](docs/how-to-add-a-new-trait.md), though.

See more about the design goals in [this documentation section](https://docs.rs/embedded-hal/latest/embedded_hal/#design-goals).

## Releases

At the moment we are working towards a `1.0.0` release (see [#177]). During this process we will
release alpha versions like `1.0.0-alpha.1` and `1.0.0-alpha.2`.
Alpha releases are **not guaranteed** to be compatible with each other.
They are provided as early previews for community testing and preparation for the final release.
If you use an alpha release, we recommend you choose an exact version specification in your
`Cargo.toml` like: `embedded-hal = "=1.0.0-alpha.2"`

See below for a way to implement both an `embedded-hal` `0.2.x` version and an `-alpha` version
side by side in a HAL.

[#177]: https://github.com/rust-embedded/embedded-hal/issues/177

## Documents

- [How-to: add a new trait](docs/how-to-add-a-new-trait.md)

## Implementations and drivers
For a list of `embedded-hal` implementations and driver crates check the [awesome-embedded-rust]
list.

[awesome-embedded-rust]: https://github.com/rust-embedded/awesome-embedded-rust#driver-crates

### Supporting different (alpha and non-alpha) HALs

[embedded-hal-compat](https://github.com/ryankurte/embedded-hal-compat) provides shims
to support interoperability between the latest `0.2.x` and `1.0.0-alpha.N` HALs, allowing one to use
incompatible HAL components (generally) without alteration.
See the [docs](https://docs.rs/embedded-hal-compat/) for examples.

It is also possible for HAL implementations to support both the latest `0.2.x` and `1.0.0-alpha.N` versions
side by side, for an example see [LPC8xx HAL](https://github.com/lpc-rs/lpc8xx-hal).

Note that `embedded-hal` `-alpha` versions are a moving target and _not guaranteed_ to be compatible.
Because of this we only aim to support the latest `-alpha`.

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.46 and up. It *might*
compile with older versions but that may change in any new patch release.

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

[CoC]: CODE_OF_CONDUCT.md
[team]: https://github.com/rust-embedded/wg#the-hal-team
