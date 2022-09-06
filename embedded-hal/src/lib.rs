//! A Hardware Abstraction Layer (HAL) for embedded systems
//!
//! **NOTE** This HAL is still is active development. Expect the traits presented here to be
//! tweaked, split or be replaced wholesale before being stabilized, i.e. before hitting the 1.0.0
//! release.
//!
//! **NOTE** If you want to use an alpha release of the 1.0.0 version, use an exact version
//! specifier in your `Cargo.toml` like: `embedded-hal = "=1.0.0-alpha.2"`.
//!
//! # Companion crates
//!
//! The main `embedded-hal` crate contains only blocking traits, where the operation is done
//! synchronously before returning. Check out the following crates, which contain versions
//! of the traits for other execution models:
//!
//! - [`embedded-hal-async`](https://docs.rs/embedded-hal-async): async/await-based.
//! - [`embedded-hal-nb`](https://docs.rs/embedded-hal-nb): polling-based, using the `nb` crate.
//!
//! The [`embedded-hal-bus`](https://docs.rs/embedded-hal-bus) crate provides utilities for sharing
//! SPI and I2C buses.
//!
//! Additionally, more domain-specific traits are available in separate crates:
//! - [`embedded-can`](https://docs.rs/embedded-can): Controller Area Network (CAN)
//!
//! # Design goals
//!
//! The HAL
//!
//! - Must *erase* device specific details. Neither register, register blocks or magic values should
//! appear in the API.
//!
//! - Must be generic *within* a device and *across* devices. The API to use a serial interface must
//! be the same regardless of whether the implementation uses the USART1 or UART4 peripheral of a
//! device or the UART0 peripheral of another device.
//!
//! - Where possible must *not* be tied to a specific asynchronous model. The API should be usable
//! in blocking mode, with the `futures` model, with an async/await model or with a callback model.
//! (cf. the [`nb`] crate)
//!
//! - Must be minimal, and thus easy to implement and zero cost, yet highly composable. People that
//! want higher level abstraction should *prefer to use this HAL* rather than *re-implement*
//! register manipulation code.
//!
//! - Serve as a foundation for building an ecosystem of platform agnostic drivers. Here driver
//! means a library crate that lets a target platform interface an external device like a digital
//! sensor or a wireless transceiver. The advantage of this system is that by writing the driver as
//! a generic library on top of `embedded-hal` driver authors can support any number of target
//! platforms (e.g. Cortex-M microcontrollers, AVR microcontrollers, embedded Linux, etc.). The
//! advantage for application developers is that by adopting `embedded-hal` they can unlock all
//! these drivers for their platform.
//!
//! - Trait methods must be fallible so that they can be used in any possible situation.
//! Nevertheless, HAL implementations can additionally provide infallible versions of the same methods
//! if they can never fail in their platform. This way, generic code can use the fallible abstractions
//! provided here but platform-specific code can avoid fallibility-related boilerplate if possible.
//!
//! # Out of scope
//!
//! - Initialization and configuration stuff like "ensure this serial interface and that SPI
//! interface are not using the same pins". The HAL will focus on *doing I/O*.
//!
//! # Reference implementation
//!
//! The [`stm32f1xx-hal`] crate contains a reference implementation of this HAL.
//!
//! [`stm32f1xx-hal`]: https://crates.io/crates/stm32f1xx-hal
//!
//! # Platform agnostic drivers
//!
//! You can find platform agnostic drivers built on top of `embedded-hal` on crates.io by [searching
//! for the *embedded-hal* keyword](https://crates.io/keywords/embedded-hal).
//!
//! If you are writing a platform agnostic driver yourself you are highly encouraged to [add the
//! embedded-hal keyword](https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata)
//! to your crate before publishing it!

#![warn(missing_docs)]
#![no_std]

pub mod delay;
pub mod digital;
pub mod i2c;
pub mod serial;
pub mod spi;

mod private {
    use crate::i2c::{SevenBitAddress, TenBitAddress};
    pub trait Sealed {}

    impl Sealed for SevenBitAddress {}
    impl Sealed for TenBitAddress {}
}
