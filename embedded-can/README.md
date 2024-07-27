[![crates.io](https://img.shields.io/crates/d/embedded-can.svg)](https://crates.io/crates/embedded-can)
[![crates.io](https://img.shields.io/crates/v/embedded-can.svg)](https://crates.io/crates/embedded-can)
[![Documentation](https://docs.rs/embedded-can/badge.svg)](https://docs.rs/embedded-can)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.60+-blue.svg)

# `embedded-can`

An embedded Controller Area Network (CAN) abstraction layer. This crate defines generic traits to be implemented by CAN driver and MCU HAL crates.

This project is developed and maintained by the [HAL team](https://github.com/rust-embedded/wg#the-hal-team).

## [API reference]

[API reference]: https://docs.rs/embedded-can

## Optional features

- **`defmt-03`**: Derive `defmt::Format` from `defmt` 0.3 for enums and structs.

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.60 and up. It *might*
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
