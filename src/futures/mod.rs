//! Asynchronous APIs
//!
//! This traits use `core::future::Future` and generic associated types.

pub mod delay;
pub mod digital;
pub mod i2c;
pub mod serial;
pub mod spi;
