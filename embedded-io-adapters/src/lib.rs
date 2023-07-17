#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(
    any(feature = "tokio-1", feature = "futures-03"),
    feature(async_fn_in_trait, impl_trait_projections)
)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod std;

#[cfg(feature = "futures-03")]
#[cfg_attr(docsrs, doc(cfg(feature = "futures-03")))]
pub mod futures_03;

#[cfg(feature = "tokio-1")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-1")))]
pub mod tokio_1;
