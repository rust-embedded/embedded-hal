//! Digital I/O
//!
//! Version 2 / fallible traits. Infallible implementations should set Error to `!`.

use core::{convert::From, ops::Not};

/// Digital output pin state
///
/// Conversion from `bool` and logical negation are also implemented
/// for this type.
/// ```rust
/// # use embedded_hal::digital::v2::PinState;
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

/// Single digital push-pull output pin
pub trait OutputPin {
    /// Error type
    type Error;

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

/// Push-pull output pin that can read its output state
///
/// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
#[cfg(feature = "unproven")]
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

/// Output pin that can be toggled
///
/// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
///
/// See [toggleable](toggleable) to use a software implementation if
/// both [OutputPin](trait.OutputPin.html) and
/// [StatefulOutputPin](trait.StatefulOutputPin.html) are
/// implemented. Otherwise, implement this using hardware mechanisms.
#[cfg(feature = "unproven")]
pub trait ToggleableOutputPin {
    /// Error type
    type Error;

    /// Toggle pin output.
    fn toggle(&mut self) -> Result<(), Self::Error>;
}

/// If you can read **and** write the output state, a pin is
/// toggleable by software.
///
/// ```
/// use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin, ToggleableOutputPin};
/// use embedded_hal::digital::v2::toggleable;
///
/// /// A virtual output pin that exists purely in software
/// struct MyPin {
///     state: bool
/// }
///
/// impl OutputPin for MyPin {
///    type Error = void::Void;
///
///    fn set_low(&mut self) -> Result<(), Self::Error> {
///        self.state = false;
///        Ok(())
///    }
///    fn set_high(&mut self) -> Result<(), Self::Error> {
///        self.state = true;
///        Ok(())
///    }
/// }
///
/// impl StatefulOutputPin for MyPin {
///    fn is_set_low(&self) -> Result<bool, Self::Error> {
///        Ok(!self.state)
///    }
///    fn is_set_high(&self) -> Result<bool, Self::Error> {
///        Ok(self.state)
///    }
/// }
///
/// /// Opt-in to the software implementation.
/// impl toggleable::Default for MyPin {}
///
/// let mut pin = MyPin { state: false };
/// pin.toggle().unwrap();
/// assert!(pin.is_set_high().unwrap());
/// pin.toggle().unwrap();
/// assert!(pin.is_set_low().unwrap());
/// ```
#[cfg(feature = "unproven")]
pub mod toggleable {
    use super::{OutputPin, StatefulOutputPin, ToggleableOutputPin};

    /// Software-driven `toggle()` implementation.
    ///
    /// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
    pub trait Default: OutputPin + StatefulOutputPin {}

    impl<P> ToggleableOutputPin for P
    where
        P: Default,
    {
        type Error = P::Error;

        /// Toggle pin output
        fn toggle(&mut self) -> Result<(), Self::Error> {
            if self.is_set_low()? {
                self.set_high()
            } else {
                self.set_low()
            }
        }
    }
}

/// Single digital input pin
///
/// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
#[cfg(feature = "unproven")]
pub trait InputPin {
    /// Error type
    type Error;

    /// Is the input pin high?
    fn is_high(&self) -> Result<bool, Self::Error>;

    /// Is the input pin low?
    fn is_low(&self) -> Result<bool, Self::Error>;
}

/// Single pin that can switch from input to output mode, and vice-versa.
///
/// Example use (assumes the `Error` type is the same for the `IoPin`,
/// `InputPin`, and `OutputPin`):
///
/// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
#[cfg(feature = "unproven")]
pub trait IoPin<TInput, TOutput>
where
    TInput: InputPin + IoPin<TInput, TOutput>,
    TOutput: OutputPin + IoPin<TInput, TOutput>,
{
    /// Error type.
    type Error;

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
