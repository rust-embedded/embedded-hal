//! Asynchronous digital I/O.
//!
//! The [`OutputPin`], [`StatefulOutputPin`] and [`InputPin`] traits are `async` variants
//! of the [blocking traits](embedded_hal::digital). These traits are useful for when
//! digital I/O may block execution, such as access through an I/O expander or over some
//! other transport.
//!
//! The [`Wait`] trait allows asynchronously waiting for a change in pin level.
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
//!
//! # For HAL authors
//!
//! If the digital I/O is implemented using memory mapped I/O and acts immediately, then the async traits
//! (except for [`Wait`]) can be implemented by calling the blocking traits and wrapping the result in
//! [`Poll::Ready`](core::task::Poll::Ready).
pub use embedded_hal::digital::{Error, ErrorKind, ErrorType, PinState};

/// Asynchronous single digital push-pull output pin.
pub trait OutputPin: ErrorType {
    /// Drives the pin low.
    ///
    /// This returns [`Ready`](core::task::Poll::Ready) when the pin has been driven low.
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be low, e.g. due to external
    /// electrical sources.
    async fn set_low(&mut self) -> Result<(), Self::Error>;

    /// Drives the pin high.
    ///
    /// This returns [`Ready`](core::task::Poll::Ready) when the pin has been driven high.
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be high, e.g. due to external
    /// electrical sources.
    async fn set_high(&mut self) -> Result<(), Self::Error>;

    /// Drives the pin high or low depending on the provided value.
    ///
    /// This returns [`Ready`](core::task::Poll::Ready) when the pin has been driven to the provided state.
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be high or low, e.g. due to external
    /// electrical sources.
    #[inline]
    async fn set_state(&mut self, state: PinState) -> Result<(), Self::Error> {
        match state {
            PinState::Low => self.set_low().await,
            PinState::High => self.set_high().await,
        }
    }
}

impl<T: OutputPin + ?Sized> OutputPin for &mut T {
    #[inline]
    async fn set_low(&mut self) -> Result<(), Self::Error> {
        T::set_low(self).await
    }

    #[inline]
    async fn set_high(&mut self) -> Result<(), Self::Error> {
        T::set_high(self).await
    }

    #[inline]
    async fn set_state(&mut self, state: PinState) -> Result<(), Self::Error> {
        T::set_state(self, state).await
    }
}

/// Asynchronous push-pull output pin that can read its output state.
pub trait StatefulOutputPin: OutputPin {
    /// Is the pin in drive high mode?
    ///
    /// This returns [`Ready`](core::task::Poll::Ready) when the pin's drive mode been read.
    ///
    /// *NOTE* this does *not* read the electrical state of the pin.
    async fn is_set_high(&mut self) -> Result<bool, Self::Error>;

    /// Is the pin in drive low mode?
    ///
    /// This returns [`Ready`](core::task::Poll::Ready) when the pin's drive mode been read.
    ///
    /// *NOTE* this does *not* read the electrical state of the pin.
    async fn is_set_low(&mut self) -> Result<bool, Self::Error>;

    /// Toggle pin output.
    ///
    /// This returns [`Ready`](core::task::Poll::Ready) when the pin has been toggled.
    async fn toggle(&mut self) -> Result<(), Self::Error> {
        let was_low: bool = self.is_set_low().await?;
        self.set_state(PinState::from(was_low)).await
    }
}

impl<T: StatefulOutputPin + ?Sized> StatefulOutputPin for &mut T {
    #[inline]
    async fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        T::is_set_high(self).await
    }

    #[inline]
    async fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        T::is_set_low(self).await
    }

    #[inline]
    async fn toggle(&mut self) -> Result<(), Self::Error> {
        T::toggle(self).await
    }
}

/// Asynchronous single digital input pin.
pub trait InputPin: ErrorType {
    /// Is the input pin high?
    ///
    /// This returns [`Ready`](core::task::Poll::Ready) when the pin's electrical state has been read.
    ///
    /// *NOTE* the input state of the pin may have changed before the future is polled.
    async fn is_high(&mut self) -> Result<bool, Self::Error>;

    /// Is the input pin low?
    ///
    /// This returns [`Ready`](core::task::Poll::Ready) when the pin's electrical state has been read.
    ///
    /// *NOTE* the input state of the pin may have changed before the future is polled.
    async fn is_low(&mut self) -> Result<bool, Self::Error>;
}

impl<T: InputPin + ?Sized> InputPin for &mut T {
    #[inline]
    async fn is_high(&mut self) -> Result<bool, Self::Error> {
        T::is_high(self).await
    }

    #[inline]
    async fn is_low(&mut self) -> Result<bool, Self::Error> {
        T::is_low(self).await
    }
}

/// Asynchronously wait for GPIO pin state.
pub trait Wait: ErrorType {
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

impl<T: Wait + ?Sized> Wait for &mut T {
    #[inline]
    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        T::wait_for_high(self).await
    }

    #[inline]
    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        T::wait_for_low(self).await
    }

    #[inline]
    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        T::wait_for_rising_edge(self).await
    }

    #[inline]
    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        T::wait_for_falling_edge(self).await
    }

    #[inline]
    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        T::wait_for_any_edge(self).await
    }
}
