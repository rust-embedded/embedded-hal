#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(feature = "async", feature(async_fn_in_trait, impl_trait_projections))]

// needed to prevent defmt macros from breaking, since they emit code that does `defmt::blahblah`.
#[cfg(feature = "defmt-03")]
use defmt_03 as defmt;

pub mod i2c;
pub mod spi;
