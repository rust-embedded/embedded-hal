[![crates.io](https://img.shields.io/crates/d/embedded-hal-async.svg)](https://crates.io/crates/embedded-hal-async)
[![crates.io](https://img.shields.io/crates/v/embedded-hal-async.svg)](https://crates.io/crates/embedded-hal-async)
[![Documentation](https://docs.rs/embedded-hal-async/badge.svg)](https://docs.rs/embedded-hal-async)
<!--
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.46+-blue.svg)
-->

# `embedded-hal-async`

An asynchronous Hardware Abstraction Layer (HAL) for embedded systems.

This crate contains asynchronous versions of the [`embedded-hal`] traits and shares its scope and [design goals].
The purpose of this crate is to iterate over these trait versions before integrating them into [`embedded-hal`].

**NOTE** These traits are still experimental. At least one breaking change to this crate is expected in the future (changing from GATs to `async fn`), but there might be more.

This project is developed and maintained by the [HAL team][team].

## [API reference]

[API reference]: https://docs.rs/embedded-hal-async

<!--
## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.46 and up. It *might*
compile with older versions but that may change in any new patch release.
-->

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
[design goals]: https://docs.rs/embedded-hal/latest/embedded_hal/#design-goals