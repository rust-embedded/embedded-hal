//! Digital I/O

use core::{convert::From, ops::Not};

/// Error
pub trait Error: core::fmt::Debug {
    /// Convert error to a generic error kind
    ///
    /// By using this method, errors freely defined by HAL implementations
    /// can be converted to a set of generic errors upon which generic
    /// code can act.
    fn kind(&self) -> ErrorKind;
}

impl Error for core::convert::Infallible {
    fn kind(&self) -> ErrorKind {
        match *self {}
    }
}

/// Error kind
///
/// This represents a common set of operation errors. HAL implementations are
/// free to define more specific or additional error types. However, by providing
/// a mapping to these common errors, generic code can still react to them.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    /// A different error occurred. The original error may contain more information.
    Other,
}

impl Error for ErrorKind {
    fn kind(&self) -> ErrorKind {
        *self
    }
}

impl core::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Other => write!(
                f,
                "A different error occurred. The original error may contain more information"
            ),
        }
    }
}

/// Error type trait
///
/// This just defines the error type, to be used by the other traits.
pub trait ErrorType {
    /// Error type
    type Error: Error;
}

impl<T: ErrorType> ErrorType for &T {
    type Error = T::Error;
}

impl<T: ErrorType> ErrorType for &mut T {
    type Error = T::Error;
}

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

/// Single digital push-pull output pin
pub trait OutputPin: ErrorType {
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
pub trait ToggleableOutputPin: ErrorType {
    /// Toggle pin output.
    fn toggle(&mut self) -> Result<(), Self::Error>;
}

impl<T: ToggleableOutputPin> ToggleableOutputPin for &mut T {
    fn toggle(&mut self) -> Result<(), Self::Error> {
        T::toggle(self)
    }
}

/// Single digital input pin
pub trait InputPin: ErrorType {
    /// Is the input pin high?
    fn is_high(&self) -> Result<bool, Self::Error>;

    /// Is the input pin low?
    fn is_low(&self) -> Result<bool, Self::Error>;
}

impl<T: InputPin> InputPin for &T {
    fn is_high(&self) -> Result<bool, Self::Error> {
        T::is_high(self)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        T::is_low(self)
    }
}
