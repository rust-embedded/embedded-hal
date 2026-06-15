//! Blocking Analog to Digital conversion.
//!
//! # Multi channel ADCs
//!
//! One of the challenges of writing portable drivers is that the device may support
//! multiple ADC channels but driver may only be capable of sampling one channel at a
//! time..
//!
//! To solve this, the implementor of [`OneShotChannel`] is expected to multiplex access to
//! the ADC. This means a driver may not directly implement the ADC traits for you.

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

/// ADC channel.
///
/// This does not contain any logic. It only defines the type of a sample read by the ADC.
pub trait Channel {
    /// The size of a sample value from the ADC.
    type Word: Sized + Copy;
}

/// ADC channel capable of a one shot measurement.
pub trait OneShotChannel: Channel + ErrorType {
    /// The maximum value that may be produced by the ADC.
    ///
    /// This is guaranteed to not change once a channel is constructed.
    ///
    /// For example, a 12-bit ADC would return 4095.
    fn max_value(&self) -> Result<Self::Word, Self::Error>;

    /// Sample from the ADC.
    fn sample(&mut self) -> Result<Self::Word, Self::Error>;
}

impl<T: Channel + ?Sized> Channel for &T {
    type Word = T::Word;
}

impl<T: Channel + ?Sized> Channel for &mut T {
    type Word = T::Word;
}

impl<T: OneShotChannel + ?Sized> OneShotChannel for &mut T {
    #[inline]
    fn max_value(&self) -> Result<Self::Word, Self::Error> {
        T::max_value(self)
    }

    #[inline]
    fn sample(&mut self) -> Result<Self::Word, Self::Error> {
        T::sample(self)
    }
}

#[cfg(test)]
mod tests {
    use core::convert::Infallible;

    use crate::adc::OneShotChannel;

    /// Verify the blocking trait is dyn compatible.
    #[allow(unused)]
    fn dyn_compatible(channel: &mut dyn OneShotChannel<Word = u16, Error = Infallible>) {}
}
