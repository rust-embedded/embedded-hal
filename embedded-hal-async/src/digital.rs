//! Asynchronous digital I/O
//!
//! # Example
//!
//! ```rust
//! # use embedded_hal_async::digital::Wait;
//! /// Asynchronously wait until the `ready_pin` becomes high.
//! async fn wait_until_ready<P>(ready_pin: &mut P)
//! where
//!     P: Wait,
//! {
//!     ready_pin
//!         .wait_for_high()
//!         .await
//!         .expect("failed to await input pin")
//! }
//! ```

use core::future::Future;

/// Asynchronously wait for GPIO pin state.
pub trait Wait: embedded_hal::digital::ErrorType {
    /// The future returned by the `wait_for_high` function.
    type WaitForHighFuture<'a>: Future<Output = Result<(), Self::Error>>
    where
        Self: 'a;

    /// Wait until the pin is high. If it is already high, return immediately.
    ///
    /// # Note for implementers
    /// The pin may have switched back to low before the task was run after
    /// being woken. The future should still resolve in that case.
    fn wait_for_high<'a>(&'a mut self) -> Self::WaitForHighFuture<'a>;

    /// The future returned by `wait_for_low`.
    type WaitForLowFuture<'a>: Future<Output = Result<(), Self::Error>>
    where
        Self: 'a;

    /// Wait until the pin is low. If it is already low, return immediately.
    ///
    /// # Note for implementers
    /// The pin may have switched back to high before the task was run after
    /// being woken. The future should still resolve in that case.
    fn wait_for_low<'a>(&'a mut self) -> Self::WaitForLowFuture<'a>;

    /// The future returned from `wait_for_rising_edge`.
    type WaitForRisingEdgeFuture<'a>: Future<Output = Result<(), Self::Error>>
    where
        Self: 'a;

    /// Wait for the pin to undergo a transition from low to high.
    ///
    /// If the pin is already low, this does *not* return immediately, it'll wait for the
    /// pin to go high and then low again.
    fn wait_for_rising_edge<'a>(&'a mut self) -> Self::WaitForRisingEdgeFuture<'a>;

    /// The future returned from `wait_for_falling_edge`.
    type WaitForFallingEdgeFuture<'a>: Future<Output = Result<(), Self::Error>>
    where
        Self: 'a;

    /// Wait for the pin to undergo a transition from high to low.
    ///
    /// If the pin is already low, this does *not* return immediately, it'll wait for the
    /// pin to go high and then low again.
    fn wait_for_falling_edge<'a>(&'a mut self) -> Self::WaitForFallingEdgeFuture<'a>;

    /// The future returned from `wait_for_any_edge`.
    type WaitForAnyEdgeFuture<'a>: Future<Output = Result<(), Self::Error>>
    where
        Self: 'a;

    /// Wait for the pin to undergo any transition, i.e low to high OR high to low.
    fn wait_for_any_edge<'a>(&'a mut self) -> Self::WaitForAnyEdgeFuture<'a>;
}

impl<T: Wait> Wait for &mut T {
    type WaitForHighFuture<'a> = T::WaitForHighFuture<'a> where Self: 'a;

    fn wait_for_high<'a>(&'a mut self) -> Self::WaitForHighFuture<'a> {
        T::wait_for_high(self)
    }

    type WaitForLowFuture<'a> = T::WaitForLowFuture<'a> where Self: 'a;

    fn wait_for_low<'a>(&'a mut self) -> Self::WaitForLowFuture<'a> {
        T::wait_for_low(self)
    }

    type WaitForRisingEdgeFuture<'a> = T::WaitForRisingEdgeFuture<'a> where Self: 'a;

    fn wait_for_rising_edge<'a>(&'a mut self) -> Self::WaitForRisingEdgeFuture<'a> {
        T::wait_for_rising_edge(self)
    }

    type WaitForFallingEdgeFuture<'a> = T::WaitForFallingEdgeFuture<'a> where Self: 'a;

    fn wait_for_falling_edge<'a>(&'a mut self) -> Self::WaitForFallingEdgeFuture<'a> {
        T::wait_for_falling_edge(self)
    }

    type WaitForAnyEdgeFuture<'a> = T::WaitForAnyEdgeFuture<'a> where Self: 'a;

    fn wait_for_any_edge<'a>(&'a mut self) -> Self::WaitForAnyEdgeFuture<'a> {
        T::wait_for_any_edge(self)
    }
}
