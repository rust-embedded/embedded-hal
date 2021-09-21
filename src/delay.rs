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
    /// Simple microsecond delay
    pub trait DelayUs {
        /// Enumeration of `DelayUs` errors
        type Error: core::fmt::Debug;

        /// Pauses execution for `us` microseconds
        fn delay_us(&mut self, us: u32) -> Result<(), Self::Error>;
    }
}
