[![crates.io](https://img.shields.io/crates/d/embedded-hal-nb.svg)](https://crates.io/crates/embedded-hal-nb)
[![crates.io](https://img.shields.io/crates/v/embedded-hal-nb.svg)](https://crates.io/crates/embedded-hal-nb)
[![Documentation](https://docs.rs/embedded-hal-nb/badge.svg)](https://docs.rs/embedded-hal-nb)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.54+-blue.svg)

# `embedded-hal-nb`

A non-blocking Hardware Abstraction Layer (HAL) for embedded systems, using the `nb` crate.

This crate contains versions of some [`embedded-hal`](https://crates.io/crates/embedded-hal) traits using `nb`, and shares its scope and [design goals].

This project is developed and maintained by the [HAL team][https://github.com/rust-embedded/wg#the-hal-team].

## [API reference]

[API reference]: https://docs.rs/embedded-hal-nb

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.54 and up. It *might*
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
