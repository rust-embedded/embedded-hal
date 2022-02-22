# How-to: add a new trait

This is the suggested approach to adding a new trait to `embedded-hal`

## Research / Discussion

Ideally, before proposing a new trait, or set of traits, you should check for an existing issue
suggesting the need for the trait, as well as any related works / use cases / requirements that
are useful to consider in the design of the trait.

These issues will be labeled as `discussion` in the issue tracker.

## Implementation / Demonstration

Proposed traits should then be implemented and demonstrated, either by forking `embedded-hal` or by creating a new crate with the intent of integrating this into `embedded-hal` once the traits have stabilized. You may find [cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) and [patch](https://doc.rust-lang.org/edition-guide/rust-2018/cargo-and-crates-io/replacing-dependencies-with-patch.html) useful for the forking approach.

Traits should be demonstrated with at least *two* implementations on different platforms and *one* generic driver built on the trait. Where it is possible we suggest an implementation on a microcontroller, and implementation for [linux](https://github.com/rust-embedded/linux-embedded-hal), and a driver (or drivers where requirements are more complex) with bounds using the trait.

## Proposing a trait

Once the trait has been demonstrated a PR should be opened to merge the new trait(s) into `embedded-hal`. This should include a link to the previous discussion issue.

If there is determined to be more than one alternative then there should be further discussion to
try to single out the best option. Once there is consensus this will be merged into the `embedded-hal` repository.

These issues / PRs will be labeled as `proposal`s in the issue tracker.
