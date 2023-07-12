[![crates.io](https://img.shields.io/crates/d/embedded-io.svg)](https://crates.io/crates/embedded-io)
[![crates.io](https://img.shields.io/crates/v/embedded-io.svg)](https://crates.io/crates/embedded-io)
[![Documentation](https://docs.rs/embedded-io/badge.svg)](https://docs.rs/embedded-io)

# `embedded-io-adapters`

This project is developed and maintained by the [HAL team](https://github.com/rust-embedded/wg#the-hal-team).

Adapters between the [`embedded-io`](https://crates.io/crates/embedded-io) and [`embedded-io-async`](https://crates.io/crates/embedded-io-async) traits and other IO traits.

The adapters are structs that wrap an I/O stream and implement another family of I/O traits
based on the wrapped streams. This allows "converting" from an `embedded_io::Read`
to a `std::io::Read` or vice versa, for example.

There are no separate adapters for `Read`/`ReadBuf`/`Write` traits. Instead, a single
adapter implements the right traits based on what the inner type implements.
This allows using these adapters when using combinations of traits, like `Read+Write`.

## Supported traits

For `embedded-io`:

- [`std::io`](https://doc.rust-lang.org/stable/std/io/index.html) traits. Needs the `std` feature.

For `embedded-io-async`:

- [`futures` 0.3](https://crates.io/crates/futures) traits. Needs the `futures-03` feature.
- [`tokio` 1.x](https://crates.io/crates/tokio) traits. Needs the `tokio-1` feature.

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.59 and up. It *might*
compile with older versions but that may change in any new patch release.

See [here](../docs/msrv.md) for details on how the MSRV may be upgraded.

Enabling any of the `tokio-*` or `futures-*` Cargo features requires Rust nightly newer than
`nightly-2022-11-22`, due to requiring support for `async fn` in traits (AFIT),
which is not stable yet. Keep in mind Rust nightlies can make backwards-incompatible
changes to unstable features at any time.

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
