[![crates.io](https://img.shields.io/crates/d/embedded-io-async.svg)](https://crates.io/crates/embedded-io-async)
[![crates.io](https://img.shields.io/crates/v/embedded-io-async.svg)](https://crates.io/crates/embedded-io-async)
[![Documentation](https://docs.rs/embedded-io-async/badge.svg)](https://docs.rs/embedded-io-async)

# `embedded-io-async`

Async IO traits for embedded systems.

This crate contains asynchronous versions of the [`embedded-io`](https://crates.io/crates/embedded-io) traits and shares its scope and design goals.

This project is developed and maintained by the [HAL team](https://github.com/rust-embedded/wg#the-hal-team).

## Optional Cargo features

- **`std`**: Adds `From` impls to convert to/from `std::io` structs.
- **`alloc`**: Adds blanket impls for `Box`, adds `Write` impl to `Vec`.
- **`defmt-03`**: Derive `defmt::Format` from `defmt` 0.3 for enums and structs.

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.75 and up. It *might*
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
