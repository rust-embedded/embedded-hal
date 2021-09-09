//! Digital I/O
//!
//! In some cases it's possible to implement these blocking traits on top of one of the core HAL
//! traits. To save boilerplate when that's the case a `Default` marker trait may be provided.
//! Implementing that marker trait will opt in your type into a blanket implementation.

use core::{convert::From, ops::Not};

/// Digital output pin state
///
/// Conversion from `bool` and logical negation are also implemented
/// for this type.
/// ```rust
/// # use embedded_hal::digital::PinState;
/// let state = PinState::from(false);
/// assert_eq!(state, PinState::Low);
/// assert_eq!(!state, PinState::High);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PinState {
    /// Low pin state
    Low,
    /// High pin state
    High,
}

impl From<bool> for PinState {
    fn from(value: bool) -> Self {
        match value {
            false => PinState::Low,
            true => PinState::High,
        }
    }
}

impl Not for PinState {
    type Output = PinState;

    fn not(self) -> Self::Output {
        match self {
            PinState::High => PinState::Low,
            PinState::Low => PinState::High,
        }
    }
}

/// Blocking digital I/O traits
pub mod blocking {
    use super::PinState;

    /// Single digital push-pull output pin
    pub trait OutputPin {
        /// Error type
        type Error: core::fmt::Debug;

        /// Drives the pin low
        ///
        /// *NOTE* the actual electrical state of the pin may not actually be low, e.g. due to external
        /// electrical sources
        fn set_low(&mut self) -> Result<(), Self::Error>;

        /// Drives the pin high
        ///
        /// *NOTE* the actual electrical state of the pin may not actually be high, e.g. due to external
        /// electrical sources
        fn set_high(&mut self) -> Result<(), Self::Error>;

        /// Drives the pin high or low depending on the provided value
        ///
        /// *NOTE* the actual electrical state of the pin may not actually be high or low, e.g. due to external
        /// electrical sources
        fn set_state(&mut self, state: PinState) -> Result<(), Self::Error> {
            match state {
                PinState::Low => self.set_low(),
                PinState::High => self.set_high(),
            }
        }
    }

    impl<T: OutputPin> OutputPin for &mut T {
        type Error = T::Error;

        fn set_low(&mut self) -> Result<(), Self::Error> {
            T::set_low(self)
        }

        fn set_high(&mut self) -> Result<(), Self::Error> {
            T::set_high(self)
        }

        fn set_state(&mut self, state: PinState) -> Result<(), Self::Error> {
            T::set_state(self, state)
        }
    }

    /// Push-pull output pin that can read its output state
    pub trait StatefulOutputPin: OutputPin {
        /// Is the pin in drive high mode?
        ///
        /// *NOTE* this does *not* read the electrical state of the pin
        fn is_set_high(&self) -> Result<bool, Self::Error>;

        /// Is the pin in drive low mode?
        ///
        /// *NOTE* this does *not* read the electrical state of the pin
        fn is_set_low(&self) -> Result<bool, Self::Error>;
    }

    impl<T: StatefulOutputPin> StatefulOutputPin for &mut T {
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            T::is_set_high(self)
        }

        fn is_set_low(&self) -> Result<bool, Self::Error> {
            T::is_set_low(self)
        }
    }

    /// Output pin that can be toggled
    ///
    /// See [toggleable](toggleable) to use a software implementation if
    /// both [OutputPin](trait.OutputPin.html) and
    /// [StatefulOutputPin](trait.StatefulOutputPin.html) are
    /// implemented. Otherwise, implement this using hardware mechanisms.
    pub trait ToggleableOutputPin {
        /// Error type
        type Error: core::fmt::Debug;

        /// Toggle pin output.
        fn toggle(&mut self) -> Result<(), Self::Error>;
    }

    impl<T: ToggleableOutputPin> ToggleableOutputPin for &mut T {
        type Error = T::Error;

        fn toggle(&mut self) -> Result<(), Self::Error> {
            T::toggle(self)
        }
    }

    /// Single digital input pin
    pub trait InputPin {
        /// Error type
        type Error: core::fmt::Debug;

        /// Is the input pin high?
        fn is_high(&self) -> Result<bool, Self::Error>;

        /// Is the input pin low?
        fn is_low(&self) -> Result<bool, Self::Error>;
    }

    impl<T: InputPin> InputPin for &T {
        type Error = T::Error;

        fn is_high(&self) -> Result<bool, Self::Error> {
            T::is_high(self)
        }

        fn is_low(&self) -> Result<bool, Self::Error> {
            T::is_low(self)
        }
    }

    /// Single pin that can switch from input to output mode, and vice-versa.
    ///
    /// Example use (assumes the `Error` type is the same for the `IoPin`,
    /// `InputPin`, and `OutputPin`):
    ///
    /// ```
    /// use core::time::Duration;
    /// use embedded_hal::digital::blocking::{IoPin, InputPin, OutputPin};
    ///
    /// pub fn ping_and_read<TInputPin, TOutputPin, TError: core::fmt::Debug>(
    ///     mut pin: TOutputPin, delay_fn: &dyn Fn(Duration) -> ()) -> Result<bool, TError>
    /// where
    ///     TInputPin : InputPin<Error = TError> + IoPin<TInputPin, TOutputPin, Error = TError>,
    ///     TOutputPin : OutputPin<Error = TError> + IoPin<TInputPin, TOutputPin, Error = TError>,
    /// {
    ///     // Ping
    ///     pin.set_low()?;
    ///     delay_fn(Duration::from_millis(10));
    ///     pin.set_high()?;
    ///
    ///     // Read
    ///     let pin = pin.into_input_pin()?;
    ///     delay_fn(Duration::from_millis(10));
    ///     pin.is_high()
    /// }
    /// ```
    pub trait IoPin<TInput, TOutput>
    where
        TInput: InputPin + IoPin<TInput, TOutput>,
        TOutput: OutputPin + IoPin<TInput, TOutput>,
    {
        /// Error type.
        type Error: core::fmt::Debug;

        /// Tries to convert this pin to input mode.
        ///
        /// If the pin is already in input mode, this method should succeed.
        fn into_input_pin(self) -> Result<TInput, Self::Error>;

        /// Tries to convert this pin to output mode with the given initial state.
        ///
        /// If the pin is already in the requested state, this method should
        /// succeed.
        fn into_output_pin(self, state: PinState) -> Result<TOutput, Self::Error>;
    }
}
