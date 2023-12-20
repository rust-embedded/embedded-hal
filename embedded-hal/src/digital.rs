//! Digital I/O.

use core::{convert::From, ops::Not};

#[cfg(feature = "defmt-03")]
use crate::defmt;

/// Error.
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

/// Error kind.
///
/// This represents a common set of operation errors. HAL implementations are
/// free to define more specific or additional error types. However, by providing
/// a mapping to these common errors, generic code can still react to them.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[non_exhaustive]
pub enum ErrorKind {
    /// A different error occurred. The original error may contain more information.
    Other,
}

impl Error for ErrorKind {
    #[inline]
    fn kind(&self) -> ErrorKind {
        *self
    }
}

impl core::fmt::Display for ErrorKind {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Other => write!(
                f,
                "A different error occurred. The original error may contain more information"
            ),
        }
    }
}

/// Error type trait.
///
/// This just defines the error type, to be used by the other traits.
pub trait ErrorType {
    /// Error type
    type Error: Error;
}

impl<T: ErrorType + ?Sized> ErrorType for &T {
    type Error = T::Error;
}

impl<T: ErrorType + ?Sized> ErrorType for &mut T {
    type Error = T::Error;
}

/// Digital output pin state.
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
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum PinState {
    /// Low pin state.
    Low,
    /// High pin state.
    High,
}

impl From<bool> for PinState {
    #[inline]
    fn from(value: bool) -> Self {
        match value {
            false => PinState::Low,
            true => PinState::High,
        }
    }
}

impl Not for PinState {
    type Output = PinState;

    #[inline]
    fn not(self) -> Self::Output {
        match self {
            PinState::High => PinState::Low,
            PinState::Low => PinState::High,
        }
    }
}

impl From<PinState> for bool {
    #[inline]
    fn from(value: PinState) -> bool {
        match value {
            PinState::Low => false,
            PinState::High => true,
        }
    }
}

/// Single digital push-pull output pin.
pub trait OutputPin: ErrorType {
    /// Drives the pin low.
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be low, e.g. due to external
    /// electrical sources.
    fn set_low(&mut self) -> Result<(), Self::Error>;

    /// Drives the pin high.
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be high, e.g. due to external
    /// electrical sources.
    fn set_high(&mut self) -> Result<(), Self::Error>;

    /// Drives the pin high or low depending on the provided value.
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be high or low, e.g. due to external
    /// electrical sources.
    #[inline]
    fn set_state(&mut self, state: PinState) -> Result<(), Self::Error> {
        match state {
            PinState::Low => self.set_low(),
            PinState::High => self.set_high(),
        }
    }
}

impl<T: OutputPin + ?Sized> OutputPin for &mut T {
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        T::set_low(self)
    }

    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        T::set_high(self)
    }

    #[inline]
    fn set_state(&mut self, state: PinState) -> Result<(), Self::Error> {
        T::set_state(self, state)
    }
}

/// Push-pull output pin that can read its output state.
pub trait StatefulOutputPin: OutputPin {
    /// Is the pin in drive high mode?
    ///
    /// *NOTE* this does *not* read the electrical state of the pin.
    fn is_set_high(&mut self) -> Result<bool, Self::Error>;

    /// Is the pin in drive low mode?
    ///
    /// *NOTE* this does *not* read the electrical state of the pin.
    fn is_set_low(&mut self) -> Result<bool, Self::Error>;

    /// Toggle pin output.
    fn toggle(&mut self) -> Result<(), Self::Error> {
        let was_low: bool = self.is_set_low()?;
        self.set_state(PinState::from(was_low))
    }
}

impl<T: StatefulOutputPin + ?Sized> StatefulOutputPin for &mut T {
    #[inline]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        T::is_set_high(self)
    }

    #[inline]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        T::is_set_low(self)
    }

    #[inline]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        T::toggle(self)
    }
}

/// Single digital input pin.
pub trait InputPin: ErrorType {
    /// Is the input pin high?
    fn is_high(&mut self) -> Result<bool, Self::Error>;

    /// Is the input pin low?
    fn is_low(&mut self) -> Result<bool, Self::Error>;
}

impl<T: InputPin + ?Sized> InputPin for &mut T {
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        T::is_high(self)
    }

    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        T::is_low(self)
    }
}
