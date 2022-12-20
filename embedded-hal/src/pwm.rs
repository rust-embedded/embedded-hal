//! Pulse Width Modulation (PWM) traits

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

impl<T: ErrorType> ErrorType for &mut T {
    type Error = T::Error;
}

/// Single PWM channel / pin
pub trait SetDuty: ErrorType {
    /// Set the duty cycle.
    ///
    /// `duty` is the duty cycle. Valid values span the entire `u16` range:
    ///
    /// - `duty = 0` is considered 0% duty, which makes the pin permanently low.
    /// - `duty = u16::MAX` is considered 100% duty, which makes the pin permanently high.
    ///
    /// Implementations must scale the duty value linearly to the range required by the hardware.
    fn set_duty(&mut self, duty: u16) -> Self::Error;
}

impl<T: SetDuty> SetDuty for &mut T {
    fn set_duty(&mut self, duty: u16) -> Self::Error {
        T::set_duty(self, duty)
    }
}
