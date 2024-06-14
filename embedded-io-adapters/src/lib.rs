#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
// disable warning for already-stabilized features.
// Needed to pass CI, because we deny warnings.
// We don't immediately remove them to not immediately break older nightlies.
// When all features are stable, we'll remove them.
#![cfg_attr(
    all(
        any(
            feature = "tokio-1",
            feature = "futures-03",
            feature = "embedded-storage-async"
        ),
        nightly
    ),
    allow(stable_features)
)]
#![cfg_attr(
    all(
        any(
            feature = "tokio-1",
            feature = "futures-03",
            feature = "embedded-storage-async"
        ),
        nightly
    ),
    feature(async_fn_in_trait, impl_trait_projections)
)]

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod std;

#[cfg(feature = "futures-03")]
#[cfg_attr(docsrs, doc(cfg(feature = "futures-03")))]
pub mod futures_03;

#[cfg(feature = "tokio-1")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-1")))]
pub mod tokio_1;

#[cfg(feature = "embedded-storage")]
#[cfg_attr(docsrs, doc(cfg(feature = "embedded-storage")))]
pub mod embedded_storage;

#[cfg(feature = "embedded-storage-async")]
#[cfg_attr(docsrs, doc(cfg(feature = "embedded-storage-async")))]
pub mod embedded_storage_async;
