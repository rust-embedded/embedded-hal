#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
// disable warning for already-stabilized features.
// Needed to pass CI, because we deny warnings.
// We don't immediately remove them to not immediately break older nightlies.
// When all features are stable, we'll remove them.
#![cfg_attr(all(feature = "async", nightly), allow(stable_features))]
#![cfg_attr(
    all(feature = "async", nightly),
    feature(async_fn_in_trait, impl_trait_projections)
)]

// needed to prevent defmt macros from breaking, since they emit code that does `defmt::blahblah`.
#[cfg(feature = "defmt-03")]
use defmt_03 as defmt;

pub mod i2c;
pub mod spi;
