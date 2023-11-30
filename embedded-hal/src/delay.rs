//! Delays.

/// Nanoseconds per microsecond
const NANOS_PER_MICRO: u32 = 1_000;
/// Nanoseconds per millisecond
const NANOS_PER_MILLI: u32 = 1_000_000;

/// Delay with up to nanosecond precision.
pub trait DelayNs {
    /// Pauses execution for at minimum `ns` nanoseconds. Pause can be longer
    /// if the implementation requires it due to precision/timing issues.
    fn delay_ns(&mut self, ns: u32);

    /// Pauses execution for at minimum `us` microseconds. Pause can be longer
    /// if the implementation requires it due to precision/timing issues.
    fn delay_us(&mut self, mut us: u32) {
        const MAX_MICROS: u32 = u32::MAX / NANOS_PER_MICRO;

        // Avoid potential overflow if micro -> nano conversion is too large
        while us > MAX_MICROS {
            us -= MAX_MICROS;
            self.delay_ns(MAX_MICROS * NANOS_PER_MICRO);
        }

        self.delay_ns(us * NANOS_PER_MICRO);
    }

    /// Pauses execution for at minimum `ms` milliseconds. Pause can be longer
    /// if the implementation requires it due to precision/timing issues.
    #[inline]
    fn delay_ms(&mut self, mut ms: u32) {
        const MAX_MILLIS: u32 = u32::MAX / NANOS_PER_MILLI;

        // Avoid potential overflow if milli -> nano conversion is too large
        while ms > MAX_MILLIS {
            ms -= MAX_MILLIS;
            self.delay_ns(MAX_MILLIS * NANOS_PER_MILLI);
        }

        self.delay_ns(ms * NANOS_PER_MILLI);
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
