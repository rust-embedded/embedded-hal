//! Asynchronous versions of HAL support.
//!
//! This module uses the built-in Rust language support for asynchronous programming.
//!
//! This module is unfortunately not called `async`, because that's a reserved keyword.
pub mod gpio;
pub mod i2c;
pub mod io;
pub mod prelude;
pub mod serial;
pub mod spi;
pub mod timer;
