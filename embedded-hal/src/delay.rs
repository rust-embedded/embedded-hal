//! Delays

/// Microsecond delay
///
pub trait DelayUs {
    /// Enumeration of `DelayUs` errors
    type Error: core::fmt::Debug;

    /// Pauses execution for at minimum `us` microseconds. Pause can be longer
    /// if the implementation requires it due to precision/timing issues.
    fn delay_us(&mut self, us: u32) -> Result<(), Self::Error>;

    /// Pauses execution for at minimum `ms` milliseconds. Pause can be longer
    /// if the implementation requires it due to precision/timing issues.
    fn delay_ms(&mut self, ms: u32) -> Result<(), Self::Error> {
        for _ in 0..ms {
            self.delay_us(1000)?;
        }

        Ok(())
    }
}

impl<T> DelayUs for &mut T
where
    T: DelayUs,
{
    type Error = T::Error;

    fn delay_us(&mut self, us: u32) -> Result<(), Self::Error> {
        T::delay_us(self, us)
    }

    fn delay_ms(&mut self, ms: u32) -> Result<(), Self::Error> {
        T::delay_ms(self, ms)
    }
}
