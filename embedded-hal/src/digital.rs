//! Blocking Digital I/O.

use core::ops::Not;

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

impl core::error::Error for ErrorKind {}

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

