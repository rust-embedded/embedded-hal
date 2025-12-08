#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod i2c;
pub mod spi;
pub mod util;
