//! Non-blocking API
//!
//! These traits make use of the [`nb`] crate
//! (*please go read that crate documentation before continuing*) to abstract over
//! the execution model and to also provide an optional blocking operation mode.
//!
//! The `nb::Result` enum is used to add a [`WouldBlock`] variant to the errors
//! of the traits. Using this it is possible to execute actions in a non-blocking
//! way.
//!
//! [`nb`]: https://crates.io/crates/nb

pub mod adc;
pub mod capture;
pub mod rng;
pub mod serial;
pub mod spi;
pub mod timer;
