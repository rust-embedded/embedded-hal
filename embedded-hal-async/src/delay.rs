//! Delays

use core::future::Future;

/// Microsecond delay
pub trait DelayUs {
    /// Enumeration of errors
    type Error: core::fmt::Debug;

    /// The future returned by the `delay_us` function.
    type DelayUsFuture<'a>: Future<Output = Result<(), Self::Error>>
    where
        Self: 'a;

    /// Pauses execution for at minimum `us` microseconds. Pause can be longer
    /// if the implementation requires it due to precision/timing issues.
    fn delay_us(&mut self, us: u32) -> Self::DelayUsFuture<'_>;

    /// The future returned by the `delay_ms` function.
    type DelayMsFuture<'a>: Future<Output = Result<(), Self::Error>>
    where
        Self: 'a;

    /// Pauses execution for at minimum `ms` milliseconds. Pause can be longer
    /// if the implementation requires it due to precision/timing issues.
    fn delay_ms(&mut self, ms: u32) -> Self::DelayMsFuture<'_>;
}

impl<T> DelayUs for &mut T
where
    T: DelayUs,
{
    type Error = T::Error;

    type DelayUsFuture<'a> = T::DelayUsFuture<'a> where Self: 'a;

    fn delay_us(&mut self, us: u32) -> Self::DelayUsFuture<'_> {
        T::delay_us(self, us)
    }

    type DelayMsFuture<'a> = T::DelayMsFuture<'a> where Self: 'a;

    fn delay_ms(&mut self, ms: u32) -> Self::DelayMsFuture<'_> {
        T::delay_ms(self, ms)
    }
}
