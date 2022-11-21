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

/// Asynchronously wait for GPIO pin state.
pub trait Wait: embedded_hal::digital::ErrorType {
    /// Wait until the pin is high. If it is already high, return immediately.
    ///
    /// # Note for implementers
    /// The pin may have switched back to low before the task was run after
    /// being woken. The future should still resolve in that case.
    async fn wait_for_high(&mut self) -> Result<(), Self::Error>;

    /// Wait until the pin is low. If it is already low, return immediately.
    ///
    /// # Note for implementers
    /// The pin may have switched back to high before the task was run after
    /// being woken. The future should still resolve in that case.
    async fn wait_for_low(&mut self) -> Result<(), Self::Error>;

    /// Wait for the pin to undergo a transition from low to high.
    ///
    /// If the pin is already high, this does *not* return immediately, it'll wait for the
    /// pin to go low and then high again.
    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error>;

    /// Wait for the pin to undergo a transition from high to low.
    ///
    /// If the pin is already low, this does *not* return immediately, it'll wait for the
    /// pin to go high and then low again.
    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error>;

    /// Wait for the pin to undergo any transition, i.e low to high OR high to low.
    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error>;
}

impl<T: Wait> Wait for &mut T {
    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        T::wait_for_high(self).await
    }

    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        T::wait_for_low(self).await
    }

    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        T::wait_for_rising_edge(self).await
    }

    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        T::wait_for_falling_edge(self).await
    }

    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        T::wait_for_any_edge(self).await
    }
}
