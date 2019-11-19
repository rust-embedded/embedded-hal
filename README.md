# `embedded-hal`

>  A Hardware Abstraction Layer (HAL) for embedded systems

This project is developed and maintained by the [HAL team][team].

## Repository Layout

This repository is a workspace containing multiple crates. The top-level crate is [`embedded-hal`](./embedded-hal), which pulls in the various other HAL crates at stable revisions. If you want the bleeding-edge, you can depend on one of the other HAL crates separately.

## Implementations and drivers

For a list of `embedded-hal` implementations and driver crates check the [awesome-embedded-rust]
list.

[awesome-embedded-rust]: https://github.com/rust-embedded/awesome-embedded-rust#driver-crates

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
