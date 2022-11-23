//! Delays

/// Microsecond delay
pub trait DelayUs {
    /// Enumeration of errors
    type Error: core::fmt::Debug;

    /// Pauses execution for at minimum `us` microseconds. Pause can be longer
    /// if the implementation requires it due to precision/timing issues.
    async fn delay_us(&mut self, us: u32) -> Result<(), Self::Error>;

    /// Pauses execution for at minimum `ms` milliseconds. Pause can be longer
    /// if the implementation requires it due to precision/timing issues.
    async fn delay_ms(&mut self, ms: u32) -> Result<(), Self::Error>;
}

impl<T> DelayUs for &mut T
where
    T: DelayUs,
{
    type Error = T::Error;

    async fn delay_us(&mut self, us: u32) -> Result<(), Self::Error> {
        T::delay_us(self, us).await
    }

    async fn delay_ms(&mut self, ms: u32) -> Result<(), Self::Error> {
        T::delay_ms(self, ms).await
    }
}
