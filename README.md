# `embedded-hal`

>  A Hardware Abstraction Layer (HAL) for embedded systems

This project is developed and maintained by the [HAL team][team].

## [API reference]

[API reference]: https://docs.rs/embedded-hal

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

## How-to: add a new trait

This is the suggested approach to adding a new trait to `embedded-hal`

### Research / Discussion

Ideally, before proposing a new trait, or set of traits, you should check for an existing issue
suggesting the need for the trait, as well as any related works / use cases / requirements that
are useful to consider in the design of the trait.

These issues will be labeled as `discussion` in the issue tracker.

### Implementation / Demonstration

Proposed traits should then be implemented and demonstrated, either by forking `embedded-hal` or by creating a new crate with the intent of integrating this into `embedded-hal` once the traits have stabilized. You may find [cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) and [patch](https://doc.rust-lang.org/edition-guide/rust-2018/cargo-and-crates-io/replacing-dependencies-with-patch.html) useful for the forking approach.

Traits should be demonstrated with at least *two* implementations on different platforms and *one* generic driver built on the trait. Where it is possible we suggest an implementation on a microcontroller, and implementation for [linux](https://github.com/rust-embedded/linux-embedded-hal), and a driver (or drivers where requirements are more complex) with bounds using the trait.

### Proposing a trait

Once the trait has been demonstrated a PR should be opened to merge the new trait(s) into `embedded-hal`. This should include a link to the previous discussion issue.

If there is determined to be more than one alternative then there should be further discussion to
try to single out the best option. Once there is consensus this will be merged into the `embedded-hal` repository.

These issues / PRs will be labeled as `proposal`s in the issue tracker.


## Implementations and drivers

For a list of `embedded-hal` implementations and driver crates check the [awesome-embedded-rust]
list.

[awesome-embedded-rust]: https://github.com/rust-embedded/awesome-embedded-rust#driver-crates

### Adding support for an `embedded-hal` `-alpha` version in a HAL implementation

It is possible for HAL implementations to support both the latest `0.2.x` version of `embedded-hal`
as well as the latest `1.0.0-alpha` version side by side. This has several big advantadges:
- Allows for a more gradual upgrade process within the community.
- Allows for a faster upgrade to `1.0` once it comes out.
- Provides more oportunities to test the new `embedded-hal` version.

This approach has been implemented in [LPC8xx HAL](https://github.com/lpc-rs/lpc8xx-hal). Here are the steps:

1. Add a dependency to the latest `embedded-hal` version to `Cargo.toml`.
   Use the `package` attribute to refer to it by another name, to prevent name collision
   ([example](https://github.com/lpc-rs/lpc8xx-hal/blob/a2b774e8a9ef025fb5119ddfb09e1b190e510896/Cargo.toml#L44-L46)).
2. Import the traits into the module where they should be implemented.
   Change their name using `as` to prevent name collisions
   ([example](https://github.com/lpc-rs/lpc8xx-hal/blob/a2b774e8a9ef025fb5119ddfb09e1b190e510896/src/gpio.rs#L49-L53)).
3. Implement the traits next to their non-alpha versions
   ([example](https://github.com/lpc-rs/lpc8xx-hal/blob/a2b774e8a9ef025fb5119ddfb09e1b190e510896/src/gpio.rs#L767-L782)).

While none of this is hard, some HAL maintainers might prefer not to add a dependency on an alpha version.
The main drawback of this approach is that it requires ongoing updates, as new `embedded-hal` alpha versions come out.
As stated before, `embedded-hal` `-alpha` versions are _not guaranteed_ to be compatible with each other.

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.35 and up. It *might*
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
