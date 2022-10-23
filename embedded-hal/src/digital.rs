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

/// Convert (some kind of pin) into an InputPin.
///
/// This allow going back and forth between input and output in a typestate fashion.
pub trait TryIntoInputPin<I:InputPin> {
    /// Error produced during conversion
    type Error;
    /// In case of error the pin object has disapeared
    fn try_into_input_pin(self) -> Result<I, Self::Error>;
}

/// Convert (some kind of pin) into an OutputPin.
///
/// This allow going back and forth between input and output in a typestate fashion.
pub trait TryIntoOutputPin<O:OutputPin> {
    /// Error produced during conversion
    type Error;
    /// In case of error the pin object has disapeared
    fn try_into_output_pin(self, state: PinState) -> Result<O, Self::Error>;
}

/// Single pin that can switch from input to output mode, and vice-versa.
///
/// Implementor can implement `TryIntoInputPin` and `TryIntoOutputPin`
/// to automatically get an implementation of `IoPin`.
///
/// Example Usage :
/// ```
/// let pin = MyGenericPin::new();              // implementor specific new
/// let input = pin.as_input()?;                // get a temporary input pin
/// input.is_low()?;                            // use it
/// let output = pin.as_output(PinState::Low)?  // get a temporary output pin
/// output.set_high()?;                         // use it
/// // this is a compile error because input has been dropped when we called as_output()
/// input.is_high()?;
/// ```
pub trait IoPin<I:InputPin,O:OutputPin> {
    /// Error type.
    type Error;

    /// Tries to convert this pin to input mode.
    ///
    /// If the pin is already in input mode, this method should succeed.
    ///
    /// After this call (and after the the result has been dropped),
    /// this pin is not anymore in the original state.
    fn as_input_pin(&mut self) -> Result<&I, Self::Error>;

    /// Tries to convert this pin to output mode.
    ///
    /// If the pin is already in output mode, this method should succeed.
    ///
    /// After this call (and after the the result has been dropped),
    /// this pin is not anymore in the original state.
    fn as_output_pin(&mut self, state: PinState) -> Result<&mut O, Self::Error>;
}

/// Generic implemnation of an IoPin.
/// An `IoPin`implementation is automatically provided if there is a way to
/// convert back and forth between `InputPin` and `OutputPin`
///
/// Implementors of specific Pins shoud provide a type alias
/// `type MyIoPin<I,O> = GenericIoPin<I,O>` to signal this is the prefered
/// way to get an `IoPin`
pub struct GenericIoPin<I,O> {
    // we use an option here to be able to take out the pin and convert it
    // before putting it back
    pin: Option<RealGenericIoPin<I,O>>
}

// GenericIoPin sub type
enum RealGenericIoPin<I,O> {
    Input(I),
    Output(O),
}

impl<I,O> GenericIoPin<I,O> {
    /// Create a new `GenericIoPin` from an `InputPin`
    pub fn from_input(pin: I) -> Self {
        GenericIoPin { pin: Some(RealGenericIoPin::Input(pin)) }
    }

    /// Create a new `GenericIoPin` from an `OutputPin`
    pub fn from_output(pin: O) -> Self {
        GenericIoPin { pin: Some(RealGenericIoPin::Output(pin)) }
    }
}

/// Error for GenericIoPin
#[derive(Debug)]
pub enum GenericIoPinError<E> {
    /// Happens if the pin is reused after an error
    MissingPin,
    /// Original error from Pin conversion
    IntoError(E),
}
impl<E> From<E> for GenericIoPinError<E> {
    fn from(e: E) -> Self { GenericIoPinError::IntoError(e) }
}

// This implementation uses `Option::take` to take out the stored pin
// and converts it before putting it back.
// This is why in case of error, `GenericIoPin` is in an invalid state.
impl<I,O,E> IoPin<I,O> for GenericIoPin<I,O>
where I: InputPin + TryIntoOutputPin<O,Error=E>,
      O: OutputPin + TryIntoInputPin<I,Error=E>,
{
    type Error=GenericIoPinError<E>;

    fn as_input_pin(&mut self) -> Result<&I, Self::Error> {
        if self.pin.is_none() {
            return Err(GenericIoPinError::MissingPin);
        }
        if let Some(RealGenericIoPin::Input(ref i)) = self.pin {
            return Ok(i);
        }
        // easy cases done let's convert
        let pin = self.pin.take();
        let input = match pin {
            Some(RealGenericIoPin::Output(p)) => p.try_into_input_pin()?,
            _ => return Err(GenericIoPinError::MissingPin), // cannot happen
        };
        self.pin = Some(RealGenericIoPin::Input(input));
        if let Some(RealGenericIoPin::Input(ref i)) = self.pin {
            return Ok(i);
        }
        // cannot happen
        Err(GenericIoPinError::MissingPin)
    }

    fn as_output_pin(&mut self, state: PinState) -> Result<&mut O, Self::Error> {
        if self.pin.is_none() {
            return Err(GenericIoPinError::MissingPin);
        }
        if let Some(RealGenericIoPin::Output(ref mut o)) = self.pin {
            return Ok(o);
        }
        // easy cases done let's convert
        let pin = self.pin.take();
        let output = match pin {
            Some(RealGenericIoPin::Input(p)) => p.try_into_output_pin(state)?,
            _ => return Err(GenericIoPinError::MissingPin), // cannot happen
        };
        self.pin = Some(RealGenericIoPin::Output(output));
        if let Some(RealGenericIoPin::Output(ref mut o)) = self.pin {
            return Ok(o);
        }
        // cannot happen
        Err(GenericIoPinError::MissingPin)
    }
}

