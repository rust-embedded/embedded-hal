//! Delays
//!
//! # What's the difference between these traits and the `timer::CountDown` trait?
//!
//! The `Timer` trait provides a *non-blocking* timer abstraction and it's meant to be used to build
//! higher level abstractions like I/O operations with timeouts. OTOH, these delays traits only
//! provide *blocking* functionality. Note that you can also use the `timer::CountDown` trait to
//! implement blocking delays.

/// Millisecond delay
///
/// `UXX` denotes the range type of the delay time. `UXX` can be `u8`, `u16`, etc. A single type can
/// implement this trait for different types of `UXX`.
pub trait DelayMs<UXX> {
    /// Pauses execution for `ms` milliseconds
    fn delay_ms(&mut self, ms: UXX);
}

/// Microsecond delay
///
/// `UXX` denotes the range type of the delay time. `UXX` can be `u8`, `u16`, etc. A single type can
/// implement this trait for different types of `UXX`.
pub trait DelayUs<UXX> {
    /// Pauses execution for `us` microseconds
    fn delay_us(&mut self, us: UXX);
}
