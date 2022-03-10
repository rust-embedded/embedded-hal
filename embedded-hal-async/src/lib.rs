//! An asynchronous Hardware Abstraction Layer (HAL) for embedded systems
//!
//! **NOTE** These traits are still experimental. At least one breaking
//! change to this crate is expected in the future (changing from GATs to
//! `async fn`), but there might be more.
//!
//! **NOTE** The traits and modules in this crate should follow the same structure as in
//! `embedded-hal` to ease merging and migration.

#![deny(missing_docs)]
#![no_std]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

pub mod delay;
pub mod digital;
pub mod i2c;
pub mod spi;
