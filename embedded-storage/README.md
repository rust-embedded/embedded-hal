# embedded-storage

> An Embedded Storage Abstraction Layer

This crate defines a set of traits that can be implemented to provide
non-volatile storage of different kinds, including but not limited to EEPROM,
NOR-flash & NAND-flash, both external and internal.

## [API reference]

## How-to: add a new trait

This is the suggested approach to adding a new trait to `embedded-storage`

### Research / Discussion

Ideally, before proposing a new trait, or set of traits, you should check for an existing issue
suggesting the need for the trait, as well as any related works / use cases / requirements that
are useful to consider in the design of the trait.

These issues will be labeled as `discussion` in the issue tracker.

### Implementation / Demonstration

Proposed traits should then be implemented and demonstrated, either by forking `embedded-storage` or by creating a new crate with the intent of integrating this into `embedded-storage` once the traits have stabilized. You may find [cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) and [patch](https://doc.rust-lang.org/edition-guide/rust-2018/cargo-and-crates-io/replacing-dependencies-with-patch.html) useful for the forking approach.

### Proposing a trait

Once the trait has been demonstrated a PR should be opened to merge the new trait(s) into `embedded-storage`. This should include a link to the previous discussion issue.

If there is determined to be more than one alternative then there should be further discussion to
try to single out the best option. Once there is consensus this will be merged into the `embedded-storage` repository.

These issues / PRs will be labeled as `proposal`s in the issue tracker.

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.50.0 and up. It *might*
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

[API reference]: https://docs.rs/embedded-storage
