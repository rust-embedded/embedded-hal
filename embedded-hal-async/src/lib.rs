#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![no_std]
#![feature(async_fn_in_trait, impl_trait_projections)]

pub mod delay;
pub mod digital;
pub mod i2c;
pub mod spi;
