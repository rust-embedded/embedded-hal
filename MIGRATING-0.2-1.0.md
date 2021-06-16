# Migrating from embedded-hal 0.2.x to 1.0.0

## Table of contents

- [Migrating from embedded-hal 0.2.x to 1.0.0](#migrating-from-embedded-hal-02x-to-100)
  - [Table of contents](#table-of-contents)
  - [Trait organization](#trait-organization)
  - [Fallibility](#fallibility)
  - [Method renaming](#method-renaming)
  - [`nb` dependency](#nb-dependency)
  - [Prelude](#prelude)
  - [Features](#features)
  - [Use-case-specific help](#use-case-specific-help)
    - [For driver authors](#for-driver-authors)
    - [I2C traits](#i2c-traits)
      - [SPI traits](#spi-traits)
    - [For HAL authors](#for-hal-authors)

## Trait organization

All traits have been organized in modules depending on their execution model. That includes `blocking` and `nb` for
non-blocking. In the future when we add asynchronous traits, we envision adding a `futures` (or similarly-named) module.

## Fallibility

All trait methods are now fallible so that they can be used in any possible situation.
However, HAL implementations can also provide infallible versions of the methods.

## Method renaming

The methods in `SPI`, `I2C` and `Serial` traits for both `blocking` and `nb` execution models have been renamed
to `write()`, `read()` and `flush()`.

## `nb` dependency

The `Result` type and `block!` macro from the [`nb`] crate are now reexported in `embeddeh_hal::nb`.
This ensures there are no version mismatches.
You should remove the `nb` crate dependency in your `Cargo.toml` in any version and use the reexported items.

In your `Cargo.toml`:
```diff
- nb = "1"
```

In your code:
```diff
- use nb;
+ use embedded_hal::nb;
```
You can also drop `#[macro_use]` if you are using Rust edition 2018.

Alternatively (needs Rust edition 2018):
```diff
- use nb::{Result, block};
+ use embedded_hal::nb::{Result, block};
```

## Prelude

The prelude has been removed because it could make method calls ambiguous, since the method names are now
the same across execution models.
To overcome this, simply import the traits you wish to use individually.
If you run into ambiguous method calls, you can disambiguate using fully-qualified syntax (the error message
from the compiler should already tell you how it should look like in your case) or tweak your trait imports or code
to limit the scope of the trait imports and thus avoid ambiguity.
Please note that it is also possible to import traits *inside a function*.

## Features

The `unproven` feature has been removed and the traits have been marked as proven.
In the past, managing unproven features, and having "sort of breaking" changes have been a struggling point.
Also, people tended to adopt `unproven` features quickly, but the features would take a very
long time to stabilize.

Instead, we would like to push experimentation OUT of the `embedded-hal` crate, allowing people to
experiment externally, and merge when some kind of feasability had been proven.

## Use-case-specific help

### For driver authors

### I2C traits

Nothing changed.

#### SPI traits

For the blocking traits nothing changed.
For the non-blocking traits, TODO

### For HAL authors

TODO


[MeetingSummary]: https://hackmd.io/ck-xRXtMTmKYXdK5bEh82A
