# Migrating from embedded-hal 0.2.x to 1.0.0

## Table of contents

- [Migrating from embedded-hal 0.2.x to 1.0.0](#migrating-from-embedded-hal-02x-to-100)
  - [Table of contents](#table-of-contents)
  - [Overview and reasoning](#overview-and-reasoning)
  - [Trait organization](#trait-organization)
    - [Trait unification](#trait-unification)
  - [Removed traits](#removed-traits)
    - [Unconstrained associated types](#unconstrained-associated-types)
    - [Impractical traits](#impractical-traits)
    - [Delay traits](#delay-traits)
  - [Bus/device separation](#busdevice-separation)
  - [Fallibility](#fallibility)
  - [SPI transfer return type](#spi-transfer-return-type)
  - [Error type bounds](#error-type-bounds)
  - [Prelude](#prelude)
  - [`rng` module](#rng-module)
  - [Removed blanket implementations](#removed-blanket-implementations)
  - [Features](#features)
  - [Companion crates](#companion-crates)
  - [Use-case-specific help](#use-case-specific-help)
    - [For driver authors](#for-driver-authors)
    - [I2C traits](#i2c-traits)
      - [SPI traits](#spi-traits)
    - [For HAL authors](#for-hal-authors)

## Overview and reasoning

There have been _a lot_ of changes in `embedded_hal` between versions 0.2.x and 1.0.0.
We understand the significance of `embedded-hal` in the Rust embedded
ecosystem and thus intend to release a version that stays compatible for a long time.

In this version, among many other changes, we have addressed several big topics that have emerged over the years:
- [Associated type compatibiilty](#removed-traits)
- [Trait fragmentation](#trait-organization)
- [Bus/device separation](#bus-device-separation)
- [Fallibility](#fallibility)
- [Execution model support](#trait-organization)

## Trait organization

All traits have been organized in modules for each feature. For example `embedded_hal::spi` and `embedded_hal::i2c`.
We only foresee having blocking traits in `embedded-hal`. We have put the traits for different execution models
into separate crates. Notably `embedded-hal-async` and `embedded-hal-nb`. See [companion crates](#companion-crates).
This allows for a separate and more tailored evolution.

<!-- TODO assumes nb separation merged -->

Execution-model-independent definitions have been moved into the feature module. For example, SPI `Phase` is now defined in `embedded_hal::spi::Phase`.

### Trait unification

Previously, there were multiple traits for the same feature. In order to avoid fragmentation and ensure
interoperability for generic code, these have now been united.

For example, most generic code should simply use the `SpiDevice` trait instead of
choosing from `Transactional`, `Transfer`, `Write` and `WriteIter`.

For HAL implementations and some specialized use-cases there are still a few traits to implement for SPI
but the number has been reduced.

Please see more about this separation [below](#bus-device-separation).

## Removed traits

These traits have been removed in the 1.0.0 release:

- [`adc::OneShot`][adc]
- [`adc::Channel`][adc]
- [`capture::Capture`][capture]
- `delay::DelayMs` (replaced by `DelayUs`)
- [`digital::IoPin`][iopin]
- [`pwm::Pwm`][pwm]
- [`pwm::PwmPin`][pwm]
- [`qei::Qei`][qei]
- [`timer::Cancel`][timer]
- [`timer::CountDown`][timer]
- [`timer::Periodic`][timer]
- [`watchdog::Disable`][watchdog]
- [`watchdog::Enable`][watchdog]
- [`watchdog::Watchdog`][watchdog]

Please find a general [roadmap with further guidance here][roadmap-rm-traits] about
how to get these traits back in a future release.
If you need them, we would like to hear from you. Please add your use case to the appropriate issue for the trait affected.

[roadmap-rm-traits]: https://github.com/rust-embedded/embedded-hal/issues/357
[adc]: https://github.com/rust-embedded/embedded-hal/issues/377
[iopin]: https://github.com/rust-embedded/embedded-hal/issues/397
[capture]: https://github.com/rust-embedded/embedded-hal/issues/361
[pwm]: https://github.com/rust-embedded/embedded-hal/issues/358
[qei]: https://github.com/rust-embedded/embedded-hal/issues/362
[timer]: https://github.com/rust-embedded/embedded-hal/issues/359
[watchdog]: https://github.com/rust-embedded/embedded-hal/issues/360

### Unconstrained associated types

Traits defined in `embedded-hal` pursue creating an interface for interoperability between generic code (be it generic user code, generic application code, generic device drivers, etc.).
When a trait has an unconstrained associated type, it is not possible to write generic code around it. Each side (implementer and user) need to specify which type the associated type will be. If the types match, the both parts can work together, however, this is not truly generic code.

For example, if somebody creates a device driver that receives a `CountDown` struct, it needs to specify what its `Time` type should be. If they choose a type coming from `fugit`, somebody else cannot use this driver if the HAL implementation for the MCU they are using only provides `CountDown` with `Time` types defined in `embedded-time`. It is also not possible for the user to implement `CountDown` for `Time` types defined by `fugit` in a straight-forward way due to the orphan rule.
In summary, it is not possible for anybody to start a countdown for a certain duration in a generic way, without it being tied to a particular time implementation and thus forcing everybody to use that one.

At the moment no solution for this has been found so we have decided to remove such traits hoping that a solution may be found
and we can add them back in a future 1.x release.

### Impractical traits

The [`digital::IoPin` trait][iopin] and the [`adc` traits][adc] have been deemed impractical for use and have thus been removed.
Please feel free to comment on the appropriate issue if you need any of these crates and propose a solution.

### Delay traits

The `DelayMs` trait has been removed. The functionality provided by this trait should now be provided by the `DelayUs` trait,
which also features a convenience `delay_ms()` method, so changes should be minimal.

This allowed us to reduce the API surface while still keeping the main functionality. We intend to add a generic `Delay` trait
in the future, once the time representation issue has been resolved.

## Bus/device separation
<!-- TODO assumes I2C bus/device merged -->

## Fallibility

All trait methods are now fallible so that they can be used in any possible situation.
However, HAL implementations can also provide infallible versions of the methods.

For example, an implementation similar to the one below would allow to use the GPIO pins as `OutputPin`s
in any generic driver or implementation-agnostic code (by importing the `OutputPin` trait),
as well as using the infallible methods in non-generic code.
This avoids the need to use `unwrap()` the results in many cases and results in more succinct code.

It should be noted that given this implementation, importing the `OutputPin` trait can result in
ambiguous calls, so please remove the trait imports if you do not need them.

```rust
use core::convert::Infallible;
use embedded_hal::digital::blocking::OutputPin;

struct HalImplGpioPin;

impl OutputPin for HalImplGpioPin {
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

impl HalImplGpioPin {
  fn set_high(&mut self) {
    // ...
  }

  fn set_low(&mut self) {
    // ...
  }
}
```

## SPI transfer return type

Previously the `transfer()` method in SPI returned a slice of the output data.
This slice is the same as the output buffer which is passed to the method, though, thus redundant and potentially confusing.
The `transfer()` method now returns `Result<(), Self::Error>`.
If you were using this return value, adapting the code should be straight forward by simply using the reception buffer which is passed.

See an example:

```rust
let tx_data = [1, 2, 3, 4];
let mut rx_data = [0; 4];
let data = spi.transfer(&tx_data, &mut rx_data)?;
println!("{:?}", data);
// There is no need to do `let data = `, since we already have the data in `rx_data`.
// Do this instead:
spi.transfer(&tx_data, &mut rx_data)?;
println!("{:?}", rx_data);
```

## Error type bounds

All associated error types are now required to implement `core::fmt::Debug`.
Usually it is enough to add a `#[derive(Debug)]` clause to your error types. For example:

```diff
+ #[derive(Debug)]
pub enum MyError {
  InvalidInputData,
  // ...
}
```

Additionally, for the I2C, SPI and Serial communication interfaces we have added a dedicated mechanism
which allows for two crucial requirements:
1. Generic code like drivers can interpret and act on errors if they want to.
2. HAL implementations can have arbitrarily-precise error types.

This works in the following way:

For each interface, `embedded-hal` defines an `ErrorKind` `enum` type with all sensible error variants as well
as an `Error` trait featuring a method that converts the type into that `ErrorKind`.

`embedded-hal` still allows for implementation-defined error types associated to each trait, but requires these to
implement the appropriate `Error` trait, thus providing a mapping to a defined set of error variants.

With this mechanism, HAL implementations can continue to define their own error types which can carry as much
information as they want. On the other hand it is now possible for generic code to inspect those errors
and act on common errors like I2Cs NACK.

Furthermore, implementation-specific code can access the original error type and retrieve any information contained.

An example of a driver which looks for I2C NACK errors and returns its own `DeviceBusy` or `Comm` error
wrapping the original one could be as follows:

```rust
const address = 0x1D;

fn set_some_parameter(&mut self) -> Result<(), Self::Error> {
  const data = [0, 1];
  match self.i2c.write(address, &data) {
    Err(e) => match e.kind() {
      ErrorKind::NoAcknowledge(_) => Err(Self::Error::DeviceBusy(e)),
      _ => Err(Self::Error::Comm(e)) // wrap and return any other error
    },
    Ok(_) => Ok(())
  }
}
```

## Prelude

The prelude has been removed because it could make method calls ambiguous, since the method names are now
the same across traits.
To overcome this, please import the traits you wish to use individually.

If you run into ambiguous method calls, you can disambiguate using the fully-qualified syntax (the error message
from the compiler should already tell you how it should look like in your case) or tweak your trait imports or code
to limit the scope of the trait imports and thus avoid the ambiguity.
Please note that it is also possible to import traits *inside a function*.

## `rng` module

The `rng` module and its traits have been removed in favor of the [`rand_core`] traits.

[`rand_core`]: https://crates.io/crates/rand_core

## Removed blanket implementations

There were several blanket implementations of blocking traits using the non-blocking
traits as a base.

Since the non-blocking traits have been extracted into the separate crate `embedded-hal-nb`,
these have been removed.

<!-- TODO assumes nb separation merged -->

## Features

The `unproven` feature has been removed and the traits have been marked as proven.
In the past, managing unproven features, and having "sort of breaking" changes has been a struggling point.
Also, people tended to adopt `unproven` features quickly, but the features would take a very
long time to stabilize.

Instead, we would like to push experimentation OUT of the `embedded-hal` crate, allowing people to
experiment externally, and merge when some kind of feasibility had been proven.

## Companion crates

## Use-case-specific help

### For driver authors

### I2C traits


#### SPI traits


### For HAL authors

TODO


[MeetingSummary]: https://hackmd.io/ck-xRXtMTmKYXdK5bEh82A
