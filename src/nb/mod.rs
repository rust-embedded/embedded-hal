//! Non-blocking API
//!
//! These traits make use of the [`nb`] crate
//! (*please go read that crate documentation before continuing*) to abstract over
//! the execution model and to also provide an optional blocking operation mode.
//!
//! The `nb::Result` enum is used to add an [`Error::WouldBlock`] variant to the errors
//! of the traits. Using this it is possible to execute actions in a non-blocking
//! way.
//!
//! `block!`, `Result` and `Error` from the [`nb`] crate are re-exported here to avoid
//! crate version mismatches. These should be used instead of importing the `nb` crate
//! directly again in dependent crates.
//!
//! [`nb`]: https://crates.io/crates/nb

pub use nb::{block, Error, Result};
pub mod adc;
pub mod capture;
pub mod serial;
pub mod spi;
pub mod timer;
