# `embedded-hal`

>  A Hardware Abstraction Layer (HAL) for embedded systems

This project is developed and maintained by the [HAL team][team].

## [API reference]

[API reference]: https://docs.rs/embedded-hal

## How-to: add a new trait

This is the suggested approach to adding a new trait to `embedded-hal`

### Discussion

Ideally, before proposing a new trait, or set of traits, you should create an issue where the use
cases and requirements of the trait(s) are discussed.

These issues will be labeled as `discussion`s in the issue tracker.

### Proposing a trait

Once there's consensus on the requirements of the trait(s) a new issue, or a PR, with a proposal
should be opened. The proposal should include the actual trait definitions as well as a link to the
issue with previous discussion, if there was one.

If the proposal includes more than one alternative then there should be further discussion to try to
single out the best alternative.

These issues / PRs will be labeled as `proposal`s in the issue tracker.

### Testing period

If there are no objections to the proposal the new trait(s) will land behind the "unproven" Cargo
feature and an issue about the new trait(s) will be created. If the proposal includes several
alternatives and a single one couldn't be chosen as the best then each alternative will land behind
a different Cargo feature, e.g. "alt1" or "alt2".

The traits will undergo a testing period before they move into the set of proven traits. During
this period users are encouraged to try to implement the unproven traits for their platforms and to
build drivers on top of them. Problems implementing the trait(s) as well as successful
implementations should be reported on the corresponding issue.

To leave the unproven state at least *two* implementations of the trait(s) for different platforms
(ideally, the two platforms should be from different vendors) and *one* generic driver built on top
of the trait(s), or alternatively one demo program that exercises the trait (via generic function /
trait object), *should* be demonstrated. If, instead, reports indicate that the proposed trait(s)
can't be implemented for a certain platform then the trait(s) will be removed and we'll go back to
the drawing board.

Issues used to track unproven APIs will be labeled as `unproven-api`s in the issue tracker and they
may also include the labels `needs-impl` and `needs-driver` to signal what's required for them to
move to the set of proven traits.

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
