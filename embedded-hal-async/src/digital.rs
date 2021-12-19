//! Asynchronous digital I/O
//!
//! # Example
//!
//! ```rust
//! # use embedded_hal_async::digital::WaitForHigh;
//! /// Asynchronously wait until the `ready_pin` becomes high.
//! async fn wait_until_ready<P>(ready_pin: &mut P)
//! where
//!     P: WaitForHigh,
//! {
//!     ready_pin
//!         .wait_for_high()
//!         .await
//!         .expect("failed to await input pin")
//! }
//! ```

use core::future::Future;

/// Asynchronously wait for a pin to be high.
pub trait WaitForHigh {
    /// Enumeration of errors.
    type Error: core::fmt::Debug;

    /// The future returned by the `wait_for_high` function.
    type WaitForHighFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Returns a future that resolves when this pin _is_ high. If the pin
    /// is already high, the future resolves immediately.
    ///
    /// # Note for implementers
    /// The pin may have switched back to low before the task was run after
    /// being woken. The future should still resolve in that case.
    fn wait_for_high<'a>(&'a mut self) -> Self::WaitForHighFuture<'a>;
}

/// Asynchronously wait for a pin to be low.
pub trait WaitForLow {
    /// Enumeration of errors.
    type Error: core::fmt::Debug;

    /// The future returned by `wait_for_low`.
    type WaitForLowFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Returns a future that resolves when this pin _is_ low. If the pin
    /// is already low, the future resolves immediately.
    ///
    /// # Note for implementers
    /// The pin may have switched back to high before the task was run after
    /// being woken. The future should still resolve in that case.
    fn wait_for_low<'a>(&'a mut self) -> Self::WaitForLowFuture<'a>;
}

/// Wait for a rising edge (transition from low to high).
pub trait WaitForRisingEdge {
    /// Enumeration of errors.
    type Error: core::fmt::Debug;

    /// The future returned from `wait_for_rising_edge`.
    type WaitForRisingEdgeFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Returns a future that resolves when this pin transitions from low to high.
    fn wait_for_rising_edge<'a>(&'a mut self) -> Self::WaitForRisingEdgeFuture<'a>;
}

/// Wait for a falling edge (transition from high to low).
pub trait WaitForFallingEdge {
    /// Enumeration of errors.
    type Error: core::fmt::Debug;

    /// The future returned from `wait_for_falling_edge`.
    type WaitForFallingEdgeFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Returns a future that resolves when this pin transitions from high to low.
    fn wait_for_falling_edge<'a>(&'a mut self) -> Self::WaitForFallingEdgeFuture<'a>;
}

/// Wait for any edge (transition from low to high OR high to low).
pub trait WaitForAnyEdge {
    /// Enumeration of errors.
    type Error: core::fmt::Debug;

    /// The future returned from `wait_for_any_edge`.
    type WaitForAnyEdgeFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Returns a future that resolves when this pin undergoes any transition, e.g.
    /// low to high OR high to low.
    fn wait_for_any_edge<'a>(&'a mut self) -> Self::WaitForAnyEdgeFuture<'a>;
}
