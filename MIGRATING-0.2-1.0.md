# Migrating from embedded-hal 0.2.x to 1.0.0

## Table of contents

- [Migrating from embedded-hal 0.2.x to 1.0.0](#migrating-from-embedded-hal-02x-to-100)
  - [Table of contents](#table-of-contents)
  - [Trait organization](#trait-organization)
  - [Fallibility](#fallibility)
  - [Method renaming](#method-renaming)
  - [`nb` dependency](#nb-dependency)
  - [Prelude](#prelude)
  - [`rng` module](#rng-module)
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

For example, an implementation similar to the one below would allow to use the GPIO pins as `OutputPin`s
in any generic driver or implementation-agnostic code (by importing the `OutputPin` trait),
as well as using the infallible methods in non-generic code, thus avoiding the need to use `unwrap()`
the results in many cases and resulting in more succinct code.

It should be noted that given this implementation, importing the `OutputPin` trait can result in
ambiguous calls, so please remove the trait imports if you do not need them.

```rust
use core::convert::Infallible;
use embedded_hal::blocking::digital::OutputPin;

struct GpioPin;

impl OutputPin for GpioPin {
  type Error = Infallible;

  fn set_high(&mut self) -> Result<(), Self::Error> {
    // ...
    Ok(())
  }

  fn set_low(&mut self) -> Result<(), Self::Error> {
    // ...
    Ok(())
  }
}

impl GpioPin {
  fn set_high(&mut self) {
    // ...
  }

  fn set_low(&mut self) {
    // ...
  }
}
```

## Method renaming

The methods in `SPI`, `I2C` and `Serial` traits for both `blocking` and `nb` execution models have been renamed
to `write()`, `read()` and `flush()` for consistency.

In order to avoid method call ambiguity, only the traits from the corresponding execution model should be imported
into the relevant scope. This is the reason why we have removed the prelude.

For more on this, see [Prelude](#prelude).

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
To overcome this, please import the traits you wish to use individually.
If you run into ambiguous method calls, you can disambiguate using fully-qualified syntax (the error message
from the compiler should already tell you how it should look like in your case) or tweak your trait imports or code
to limit the scope of the trait imports and thus avoid ambiguity.
Please note that it is also possible to import traits *inside a function*.

## `rng` module

The `rng` module and its traits have been removed in favor of the [`rand_core`] traits.

[`rand_core`]: https://crates.io/crates/rand_core

## Features

The `unproven` feature has been removed and the traits have been marked as proven.
In the past, managing unproven features, and having "sort of breaking" changes has been a struggling point.
Also, people tended to adopt `unproven` features quickly, but the features would take a very
long time to stabilize.

Instead, we would like to push experimentation OUT of the `embedded-hal` crate, allowing people to
experiment externally, and merge when some kind of feasibility had been proven.

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
