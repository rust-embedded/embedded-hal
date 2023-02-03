//! Digital I/O

use core::{convert::From, ops::Not};

/// GPIO error type trait
///
/// This just defines the error type, to be used by the other traits.
pub trait ErrorType {
    /// Error type
    type Error: core::fmt::Debug;
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

impl From<PinState> for bool {
    fn from(value: PinState) -> bool {
        match value {
            PinState::Low => false,
            PinState::High => true,
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

/// A digital output "port"
///
/// `N` is number of pins in "port"
///
/// **NOTE** The "port" doesn't necessarily has to match a hardware GPIO port;
/// it could for instance be a 4-bit ports made up of non contiguous pins, say
/// `PA0`, `PA3`, `PA10` and `PA13`.
pub trait OutputPort<const N: usize>: ErrorType {
    /// Outputs `N` least significant bits of `word` on the port pins
    ///
    /// # Contract
    ///
    /// The state of all the port pins will change atomically ("at the same time"). This usually
    /// means that state of all the pins will be changed in a single register operation.
    fn write(&mut self, word: u16) -> Result<(), Self::Error>;

    /// Set all pins to `PinState::High`
    fn all_high(&mut self) -> Result<(), Self::Error> {
        self.write(!0)
    }

    /// Reset all pins to `PinState::Low`
    fn all_low(&mut self) -> Result<(), Self::Error> {
        self.write(0)
    }
}
