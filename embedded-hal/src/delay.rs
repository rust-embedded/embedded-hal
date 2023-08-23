//! Delays.

/// Microsecond delay.
pub trait DelayUs {
    /// Pauses execution for at minimum `us` microseconds. Pause can be longer
    /// if the implementation requires it due to precision/timing issues.
    fn delay_us(&mut self, us: u32);

    /// Pauses execution for at minimum `ms` milliseconds. Pause can be longer
    /// if the implementation requires it due to precision/timing issues.
    #[inline]
    fn delay_ms(&mut self, ms: u32) {
        for _ in 0..ms {
            self.delay_us(1000);
        }
    }
}

impl<T> DelayUs for &mut T
where
    T: DelayUs + ?Sized,
{
    #[inline]
    fn delay_us(&mut self, us: u32) {
        T::delay_us(self, us)
    }

    #[inline]
    fn delay_ms(&mut self, ms: u32) {
        T::delay_ms(self, ms)
    }
}
