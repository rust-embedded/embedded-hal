//! An asynchronous Hardware Abstraction Layer (HAL) for embedded systems
//!
//! **NOTE** These traits are still experimental. At least one breaking
//! change to this crate is expected in the future (changing from GATs to
//! `async fn`), but there might be more.
//!
//! **NOTE** The traits and modules in this crate should follow the same structure as in
//! `embedded-hal` to ease merging and migration.

#![warn(missing_docs)]
#![no_std]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait, impl_trait_projections)]

pub mod delay;
pub mod digital;
pub mod i2c;
pub mod spi;
