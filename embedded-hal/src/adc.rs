//! Blocking analog-digital conversion traits.

use core::fmt::Debug;

#[cfg(feature = "defmt-03")]
use crate::defmt;

/// Read data from an ADC.
///
/// # Examples
///
/// In the first naive example, [`read`](crate::adc::AdcChannel::read) is implemented
/// using a spin loop and only returns once data is ready.
///
/// ```
/// use embedded_hal::adc::{AdcChannel, ErrorKind, ErrorType, Error};
///
/// struct MySpinningAdc;
///
/// impl MySpinningAdc {
///     pub fn is_ready(&mut self) -> bool {
///         // Just pretend this returns `false` the first few times.
///         true
///     }
///
///     pub fn data(&mut self) -> u32 {
///         42
///     }
/// }
///
/// impl ErrorType for MySpinningAdc {
///     type Error = ErrorKind;
/// }
///
/// impl AdcChannel for MySpinningAdc {
///     fn read(&mut self) -> Result<u32, Self::Error> {
///         while !self.is_ready() {
///             core::hint::spin_loop();
///         }
///
///         Ok(self.data())
///     }
/// }
/// ```
pub trait AdcChannel: ErrorType {
    /// Reads data from the ADC.
    ///
    /// # Note for Implementers
    ///
    /// This should wait until data is ready and then read it.
    /// If the ADC's precision is less than 32 bits, the value must be scaled accordingly.
    fn read(&mut self) -> Result<u32, Self::Error>;
}

impl<T> AdcChannel for &mut T
where
    T: AdcChannel + ?Sized,
{
    #[inline]
    fn read(&mut self) -> Result<u32, Self::Error> {
        (*self).read()
    }
}

/// ADC error.
pub trait Error: Debug {
    /// Convert error to a generic ADC error kind.
    ///
    /// By using this method, ADC errors freely defined by HAL implementations
    /// can be converted to a set of generic ADC errors upon which generic
    /// code can act.
    fn kind(&self) -> ErrorKind;
}

impl Error for core::convert::Infallible {
    #[inline]
    fn kind(&self) -> ErrorKind {
        match *self {}
    }
}

/// ADC error kind.
///
/// This represents a common set of ADC operation errors. HAL implementations are
/// free to define more specific or additional error types. However, by providing
/// a mapping to these common ADC errors, generic code can still react to them.
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

/// ADC error type trait.
///
/// This just defines the error type, to be used by the other ADC traits.
pub trait ErrorType {
    /// Error type.
    type Error: Error;
}

impl<T: ErrorType + ?Sized> ErrorType for &mut T {
    type Error = T::Error;
}
