//! Delays
//!
//! # What's the difference between these traits and the `timer::CountDown` trait?
//!
//! The `Timer` trait provides a *non-blocking* timer abstraction and it's meant to be used to build
//! higher level abstractions like I/O operations with timeouts. OTOH, these delays traits only
//! provide *blocking* functionality. Note that you can also use the `timer::CountDown` trait to
//! implement blocking delays.

/// Blocking delay traits
pub mod blocking {
    /// Millisecond delay
    ///
    /// `UXX` denotes the range type of the delay time. `UXX` can be `u8`, `u16`, etc. A single type can
    /// implement this trait for different types of `UXX`.
    pub trait DelayMs<UXX> {
        /// Enumeration of `DelayMs` errors
        type Error: core::fmt::Debug;

        /// Pauses execution for `ms` milliseconds
        fn delay_ms(&mut self, ms: UXX) -> Result<(), Self::Error>;
    }

    impl<UXX, T: DelayMs<UXX>> DelayMs<UXX> for &mut T {
        type Error = T::Error;

        fn delay_ms(&mut self, ms: UXX) -> Result<(), Self::Error> {
            (*self).delay_ms(ms)
        }
    }

    /// Microsecond delay
    ///
    /// `UXX` denotes the range type of the delay time. `UXX` can be `u8`, `u16`, etc. A single type can
    /// implement this trait for different types of `UXX`.
    pub trait DelayUs<UXX> {
        /// Enumeration of `DelayMs` errors
        type Error: core::fmt::Debug;

        /// Pauses execution for `us` microseconds
        fn delay_us(&mut self, us: UXX) -> Result<(), Self::Error>;
    }

    impl<UXX, T: DelayUs<UXX>> DelayUs<UXX> for &mut T {
        type Error = T::Error;

        fn delay_us(&mut self, us: UXX) -> Result<(), Self::Error> {
            (*self).delay_us(us)
        }
    }
}
