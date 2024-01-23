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

/// This adapter will [panic] if the inner device encounters an error.
///
/// It currently supports [embedded_hal::digital::OutputPin], but other traits may be added in the future.
///
/// TODO: add usage example
#[repr(transparent)]
pub struct UnwrappingAdapter<T>(pub T);

impl<T> embedded_hal::digital::ErrorType for UnwrappingAdapter<T> {
    type Error = core::convert::Infallible;
}

impl<T> embedded_hal::digital::OutputPin for UnwrappingAdapter<T>
where
    T: embedded_hal::digital::OutputPin,
{
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.0.set_low().unwrap();
        Ok(())
    }

    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.0.set_high().unwrap();
        Ok(())
    }

    #[inline]
    fn set_state(&mut self, state: embedded_hal::digital::PinState) -> Result<(), Self::Error> {
        self.0.set_state(state).unwrap();
        Ok(())
    }
}
