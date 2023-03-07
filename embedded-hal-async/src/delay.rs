//! Delays

/// Microsecond delay
pub trait DelayUs {
    /// Pauses execution for at minimum `us` microseconds. Pause can be longer
    /// if the implementation requires it due to precision/timing issues.
    async fn delay_us(&mut self, us: u32);

    /// Pauses execution for at minimum `ms` milliseconds. Pause can be longer
    /// if the implementation requires it due to precision/timing issues.
    async fn delay_ms(&mut self, ms: u32);
}

impl<T> DelayUs for &mut T
where
    T: DelayUs,
{
    async fn delay_us(&mut self, us: u32) {
        T::delay_us(self, us).await
    }

    async fn delay_ms(&mut self, ms: u32) {
        T::delay_ms(self, ms).await
    }
}
