//! Delays.

/// Delay with up to nanosecond precision.
pub trait DelayNs {
    /// Pauses execution for at minimum `ns` nanoseconds. Pause can be longer
    /// if the implementation requires it due to precision/timing issues.
    fn delay_ns(&mut self, ns: u32);

    /// Pauses execution for at minimum `us` microseconds. Pause can be longer
    /// if the implementation requires it due to precision/timing issues.
    fn delay_us(&mut self, mut us: u32) {
        while us > 4_294_967 {
            us -= 4_294_967;
            self.delay_ns(4_294_967_000);
        }
        self.delay_ns(us * 1_000);
    }

    /// Pauses execution for at minimum `ms` milliseconds. Pause can be longer
    /// if the implementation requires it due to precision/timing issues.
    #[inline]
    fn delay_ms(&mut self, mut ms: u32) {
        while ms > 4294 {
            ms -= 4294;
            self.delay_ns(4_294_000_000);
        }
        self.delay_ns(ms * 1_000_000);
    }
}

impl<T> DelayNs for &mut T
where
    T: DelayNs + ?Sized,
{
    #[inline]
    fn delay_ns(&mut self, ns: u32) {
        T::delay_ns(self, ns);
    }

    #[inline]
    fn delay_us(&mut self, us: u32) {
        T::delay_us(self, us);
    }

    #[inline]
    fn delay_ms(&mut self, ms: u32) {
        T::delay_ms(self, ms);
    }
}
