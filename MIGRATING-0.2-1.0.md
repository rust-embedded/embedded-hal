# Migrating from embedded-hal 0.2.x to 1.0.0

## Table of contents

- [Overview and reasoning](#overview-and-reasoning)
- [Trait organization](#trait-organization)
- [Trait unification](#trait-unification)
- [Removed traits](#removed-traits)
  - [Unconstrained associated types](#unconstrained-associated-types)
  - [Impractical traits](#impractical-traits)
  - [Serial traits](#serial-traits)
  - [RNG traits](#rng-traits)
  - [CAN traits](#can-traits)
- [SPI Bus/device separation](#spi-busdevice-separation)
- [Fallibility](#fallibility)
- [SPI transfer return type](#spi-transfer-return-type)
- [Error type bounds](#error-type-bounds)
- [Prelude](#prelude)
- [Removed blanket implementations](#removed-blanket-implementations)
- [Cargo Features](#cargo-features)
- [Companion crates](#companion-crates)

## Overview and reasoning

There have been _a lot_ of changes in `embedded_hal` between versions 0.2.x and 1.0.0.
We understand the significance of `embedded-hal` in the Rust embedded
ecosystem and thus intend to release a version that stays compatible for a long time.

The main difference betewen `embedded-hal` 0.2 and 1.0 is the project is now focused
on a single goal: traits for writing drivers that work on any HAL.

In `embedded-hal` 0.2, the traits had dual goals:
- Standardize the API of HAL crates, so HAL crate authors get guidance on how to design their APIs and
  end users writing code directly against one HAL get a familiar API.
- Allowing writing generic drivers using the traits, so they work on top of any HAL crate.

For `embedded-hal` 1.0, we decided to drop the first goal, targeting only the second. The reasons are:

- Standardizing HAL APIs is difficult, because hardware out there has wildly different sets of capabilities. Modeling all capabilities required many different variants of the traits, and required "customization points" like associated types, significantly increasing complexity.
- There is a tension between both goals. "Customization points" like associated types make the traits hard to use in generic HAL-independent drivers.
- The second goal delivers much more value. Being able to use any driver together with any HAL crate, out of the box, and across the entire Rust Embedded ecosystem, is just plain awesome.

This refocusing on drivers is the root cause of many of the changes between `embedded-hal` 0.2 and 1.0:
- [Associated type compatibiilty](#removed-traits)
- [Trait fragmentation](#trait-organization)
- [Bus/device separation](#bus-device-separation)
- [Fallibility](#fallibility)
- [Execution model support](#trait-organization)

## Trait organization

All traits have been organized in modules for each peripheral. For example `embedded_hal::spi` and `embedded_hal::i2c`.
We only foresee having blocking traits in `embedded-hal`. We have put the traits for different execution models
into separate crates. Notably `embedded-hal-async` and `embedded-hal-nb`. See [companion crates](#companion-crates).
This allows for a separate and more tailored evolution.

Execution-model-independent definitions have been moved into the peripheral's module. For example, SPI `Phase` is now defined in `embedded_hal::spi::Phase`.

## Trait unification

Previously, there were multiple traits for the same peripheral, for different sets of capabilities. The reasoning
was different hardware supports a different set of features, so by making the traits granular each HAL implementation
can implement only the features supported by the hardware.

However, this has proven to be troublesome for generic drivers, in cases where a driver expects to use one
trait, but the HAL crate implements only other traits. To avoid this fragmentation and ensure
interoperability for generic code, these have now been unified.

- I2C: `Read`, `Write`, `WriteIter`, `WriteIterRead`, `WriteRead`, `Transactional`, `TransactionalIter` have now been unified into a single `I2c` trait.
- SPI: `Write` `WriteIter`, `Transfer`, `Transactional` have been unified into `SpiBus`.
- GPIO: `ToggleableOutputPin` has been merged into `StatefulOutputPin`.
- Delays: `DelayMs`, `DelayUs` has been unified into `DelayNs` (and precision extended to nanoseconds).

HAL implementation crates should implement the full functionality of the traits. If a feature is not supported natively by the hardware, it should be polyfilled/emulated in software. In no case should "not supported" errors be returned. This ensures maximum compatibility.

## Removed traits

These traits have been removed in the 1.0.0 release, with no replacement for now:

- [`adc::OneShot`][adc]
- [`adc::Channel`][adc]
- [`capture::Capture`][capture]
- [`digital::IoPin`][iopin]
- [`pwm::Pwm`][pwm]
- [`qei::Qei`][qei]
- [`timer::Cancel`][timer]
- [`timer::CountDown`][timer]
- [`timer::Periodic`][timer]
- [`watchdog::Disable`][watchdog]
- [`watchdog::Enable`][watchdog]
- [`watchdog::Watchdog`][watchdog]

Please find a general [roadmap with further guidance here][roadmap-rm-traits] about
whether and how to get these traits back in a future release

If you are a generic driver author and need one of them, we would like to hear from you. Please add your use case to the appropriate issue for the trait affected.

HAL implementation crates are encouraged to provide their own APIs for functionality for the removed traits, and not implement any traits. This will allow the APIs to more closely match the hardware capabilities, and allow users to continue to use them.

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
When a trait has an unconstrained associated type (for example `type Time;`), it is not possible to write generic code around it. Each side (implementer and user) need to specify which type the associated type will be. If the types match, the both parts can work together, however, this is not truly generic code.

For example, if somebody creates a device driver that receives a `CountDown` struct, it needs to specify what its `Time` type should be. If they choose a type coming from `fugit`, somebody else cannot use this driver if the HAL implementation for the MCU they are using only provides `CountDown` with `Time` types defined in `embedded-time`. It is also not possible for the user to implement `CountDown` for `Time` types defined by `fugit` in a straight-forward way due to the orphan rule.
In summary, it is not possible for anybody to start a countdown for a certain duration in a generic way, without it being tied to a particular time implementation and thus forcing everybody to use that one. This means all these traits don't fulfill the "allow writing generic drivers" goal.

At the moment no solution for this has been found so we have decided to remove such traits hoping that a solution may be found
and we can add them back in a future 1.x release.

### Impractical traits

The [`digital::IoPin` trait][iopin] and the [`adc` traits][adc] have been deemed impractical for use and have thus been removed.
Please feel free to comment on the appropriate issue if you need any of these traitsk and propose a solution.

### Serial traits

The `blocking::serial::Write` trait has been removed in favor of the [`embedded-io`] traits, also maintained within the `embedded-hal` repository.

[`embedded-io`]: https://crates.io/crates/embedded-io

### RNG traits

The `rng` module and its traits have been removed in favor of the [`rand_core`] traits.

[`rand_core`]: https://crates.io/crates/rand_core

### CAN traits

The `can` module and its traits have been removed in favor of the [`embedded-can`] traits.

[`embedded-can`]: https://crates.io/crates/embedded-can

## SPI Bus/device separation

The SPI traits have been unified into a single `SpiBus` trait. However, to allow sharing an SPI bus, and hardware control of the CS pin, 1.0 adds the `SpiDevice` trait.

The short summary is:
- `SpiBus` represents an entire SPI bus (with SCK, MOSI, MISO) pins
- `SpiDevice` represents a single device on an SPI bus, selected by a CS pin.

See the [SPI documentation](https://docs.rs/embedded-hal/1.0.0/embedded_hal/spi/index.html) for more details.

When upgrading code to `embedded-hal` 1.0, it is critical to implement/use the right trait depending on the underlying situation.

### For HAL implementation crates

- If you previously implemented the SPI traits, and did *not* manage a CS pin automatically, you should now implement `SpiBus`, which is the equivalent in 1.0.
- Optionally, if the API *does* manage a CS pin automatically, you may implement `SpiDevice`.
  - This is required if the underlying API requires it to manage the CS pin, like `spidev` on Linux.

Do not implement `SpiBus` and `SpiDevice` on the same struct, since this is never correct. When there's no CS pin being controlled you must implement only `SpiBus`, and when there is, implement only `SpiDevice`. If you want to offer both APIs, implement them on separate structs so the user has to cohose one or the other.

### For driver crates

- If your device has SCK, MOSI, MISO, CS pins: use `SpiDevice`.
  - Do NOT take the CS pin as a separate `OutputPin`, the `SpiDevice` will manage it for you. Taking the CS pin separately will make your driver not work on shared buses.
- If your device only has SCK, MOSI, MISO: use `SpiBus`.
  - This means bus sharing won't be supported, but there's no way to share without a CS pin anyway.
- If you're using SPI to bitbang non-SPI protocols (for example, WS2812 smart LEDs), use `SpiBus`.

### For end users

You will most likely find the HAL crate you're using implements `SpiBus`, and the driver you want to use
requires `SpiDevice`. To convert from `SpiBus` to `SpiDevice`, wrap it with a [`embedded_hal_bus::spi::ExclusiveDevice`](https://docs.rs/embedded-hal-bus/0.1.0/embedded_hal_bus/spi/struct.ExclusiveDevice.html), together with the CS pin: 

```rust
use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};

// Create the SPI from the HAL. This implements SpiBus, not SpiDevice!
let spi_bus = my_hal::spi::Spi::new(...);
// Create the CS. This must implement OutputPin.
let cs = my_hal::gpio::Output::new(...);

// Combine the SPI bus and the CS pin into a SPI device. This now does implement SpiDevice!
let spi_device = ExclusiveDevice::new(spi_bus, cs, NoDelay);

// Now you can create your driver with it!
let driver = my_driver::Driver::new(spi_device, ...);
```

If you want multiple drivers to share the same SPI bus, [`embedded_hal_bus::spi`](https://docs.rs/embedded-hal-bus/0.1.0/embedded_hal_bus/spi/index.html)
has a few options depending on the kind of mutex you want to use. This is now built-in to `embedded-hal`, using external crates like `shared-bus` is discouraged.

For example, you can use `RefCellDevice` when you don't need drivers to be `Send`.

```rust
use core::cell::RefCell;
use embedded_hal_bus::spi::{RefCellDevice, NoDelay};

// Create the SPI bus and CS pins.
let spi_bus = my_hal::spi::Spi::new(...);
let cs1 = my_hal::gpio::Output::new(...);
let cs2 = my_hal::gpio::Output::new(...);

// Wrap the bus with a RefCell.
let spi_bus = RefCell::new(spi_bus);

// Combine references to the SPI bus with a CS pin to get a SpiDevice for one device on the bus.
let device1 = RefCellDevice::new(&spi_bus, cs1, NoDelay);
let device2 = RefCellDevice::new(&spi_bus, cs2, NoDelay);

// Now you can create drivers. They will transparently talk each to its own device, sharing the same bus.
let driver1 = my_driver::Driver::new(device1, ...);
let driver2 = my_driver::Driver::new(device2, ...);
```

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

Additionally, for the I2C and SPI communication interfaces we have added a dedicated mechanism
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

## Removed blanket implementations

There were several blanket implementations of blocking traits using the non-blocking
traits as a base.

Since the non-blocking traits have been extracted into the separate crate `embedded-hal-nb`,
these have been removed.

## Cargo features

The `unproven` feature has been removed and the traits have been marked as proven.
In the past, managing unproven features, and having "sort of breaking" changes has been a struggling point.
Also, people tended to adopt `unproven` features quickly, but the features would take a very
long time to stabilize.

Instead, we would like to push experimentation OUT of the `embedded-hal` crate, allowing people to
experiment externally, and merge when some kind of feasibility had been proven.

## Companion crates

The `embedded-hal` project now spans several crates, where some functionality has been moved out from the main `embedded-hal` crate to separate crates as detailed above. Here is the full listing of crates:

| Crate | crates.io | Docs | |
|-|-|-|-|
| [embedded-hal](./embedded-hal)       | [![crates.io](https://img.shields.io/crates/v/embedded-hal.svg)](https://crates.io/crates/embedded-hal) | [![Documentation](https://docs.rs/embedded-hal/badge.svg)](https://docs.rs/embedded-hal) | Core traits, blocking version |
| [embedded-hal-async](./embedded-hal-async) | [![crates.io](https://img.shields.io/crates/v/embedded-hal-async.svg)](https://crates.io/crates/embedded-hal-async) | [![Documentation](https://docs.rs/embedded-hal-async/badge.svg)](https://docs.rs/embedded-hal-async) | Core traits, async version |
| [embedded-hal-nb](./embedded-hal-nb)    | [![crates.io](https://img.shields.io/crates/v/embedded-hal-nb.svg)](https://crates.io/crates/embedded-hal-nb) | [![Documentation](https://docs.rs/embedded-hal-nb/badge.svg)](https://docs.rs/embedded-hal-nb) | Core traits, polling version using the `nb` crate |
| [embedded-hal-bus](./embedded-hal-bus)   | [![crates.io](https://img.shields.io/crates/v/embedded-hal-bus.svg)](https://crates.io/crates/embedded-hal-bus) | [![Documentation](https://docs.rs/embedded-hal-bus/badge.svg)](https://docs.rs/embedded-hal-bus) | Utilities for sharing SPI and I2C buses |
| [embedded-can](./embedded-can)       | [![crates.io](https://img.shields.io/crates/v/embedded-can.svg)](https://crates.io/crates/embedded-can) | [![Documentation](https://docs.rs/embedded-can/badge.svg)](https://docs.rs/embedded-can) | Controller Area Network (CAN) traits |
| [embedded-io](./embedded-io)       | [![crates.io](https://img.shields.io/crates/v/embedded-io.svg)](https://crates.io/crates/embedded-io) | [![Documentation](https://docs.rs/embedded-io/badge.svg)](https://docs.rs/embedded-io) | I/O traits (read, write, seek, etc.), blocking and nonblocking version. |
| [embedded-io-async](./embedded-io-async)       | [![crates.io](https://img.shields.io/crates/v/embedded-io-async.svg)](https://crates.io/crates/embedded-io-async) | [![Documentation](https://docs.rs/embedded-io-async/badge.svg)](https://docs.rs/embedded-io-async) | I/O traits, async version  |
| [embedded-io-adapters](./embedded-io-adapters)       | [![crates.io](https://img.shields.io/crates/v/embedded-io-adapters.svg)](https://crates.io/crates/embedded-io-adapters) | [![Documentation](https://docs.rs/embedded-io-adapters/badge.svg)](https://docs.rs/embedded-io-adapters) | Adapters between the [`embedded-io`](https://crates.io/crates/embedded-io) and [`embedded-io-async`](https://crates.io/crates/embedded-io-async) traits and other IO traits (`std`, `tokio`, `futures`...)  |

