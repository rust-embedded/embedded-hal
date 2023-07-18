#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(feature = "async", feature(async_fn_in_trait, impl_trait_projections))]

pub mod i2c;
pub mod spi;
