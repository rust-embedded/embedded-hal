#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod fmt;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod std;

#[cfg(feature = "futures-03")]
#[cfg_attr(docsrs, doc(cfg(feature = "futures-03")))]
pub mod futures_03;

#[cfg(feature = "tokio-1")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-1")))]
pub mod tokio_1;

#[cfg(feature = "digest")]
pub mod digest;