//! Blocking analog-digital conversion traits.

use core::fmt::{Debug, Display};

#[cfg(feature = "defmt-03")]
use crate::defmt;

/// Blocking voltmeter for measuring voltage.
///
/// # Examples
///
/// In the first naive example, [`Voltmeter`] is implemented using a spin loop.
///
/// ```
/// use embedded_hal::adc::{ErrorKind, ErrorType, Error, Voltmeter};
///
/// struct MySpinningVoltmeter;
///
/// impl MySpinningVoltmeter {
///     pub fn is_ready(&mut self) -> bool {
///         // Just pretend this returns `false` the first few times.
///         true
///     }
///
///     pub fn data(&mut self) -> u16 {
///         3300
///     }
/// }
///
/// impl ErrorType for MySpinningVoltmeter {
///     type Error = ErrorKind;
/// }
///
/// impl Voltmeter for MySpinningVoltmeter {
///     fn measure_nv(&mut self) -> Result<i64, Self::Error> {
///         Ok(self.measure_mv()? as i64 * 1_000_000)
///     }
///
///     fn measure_mv(&mut self) -> Result<i16, Self::Error> {
///         while !self.is_ready() {
///             core::hint::spin_loop();
///         }
///
///         Ok(self.data() as i16)
///     }
/// }
/// ```
pub trait Voltmeter: ErrorType {
    /// Measures voltage in nV (nanovolts).
    ///
    /// This can measure between -9223372036.854775808V and 9223372036.854775807V.
    fn measure_nv(&mut self) -> Result<i64, Self::Error>;

    /// Measures voltage in mV (microvolts).
    ///
    /// This can measure between -2147.483648V and 2147.483647V.
    /// If you need to measure a larger range, use [`measure_nv`](Voltmeter::measure_nv) instead.
    ///
    /// When overriding the default implementation, ensure that the measured voltage is clamped
    /// between [`i32::MIN`] and [`i32::MAX`].
    fn measure_uv(&mut self) -> Result<i32, Self::Error> {
        Ok((self.measure_nv()? / 1_000).clamp(i32::MIN.into(), i32::MAX.into()) as i32)
    }

    /// Measures voltage in mV (millivolts).
    ///
    /// This can measure between between -32.768V and 32.767V.
    /// If you need to measure a larger range,
    /// use [`measure_uv`](Voltmeter::measure_uv) or [`measure_mv`](Voltmeter::measure_mv) instead.
    ///
    /// When overriding the default implementation, ensure that the measured voltage is clamped
    /// between [`i16::MIN`] and [`i16::MAX`].
    fn measure_mv(&mut self) -> Result<i16, Self::Error> {
        Ok((self.measure_uv()? / 1_000).clamp(i16::MIN.into(), i16::MAX.into()) as i16)
    }
}

impl<T> Voltmeter for &mut T
where
    T: Voltmeter + ?Sized,
{
    #[inline]
    fn measure_nv(&mut self) -> Result<i64, Self::Error> {
        (*self).measure_nv()
    }

    #[inline]
    fn measure_uv(&mut self) -> Result<i32, Self::Error> {
        (*self).measure_uv()
    }

    #[inline]
    fn measure_mv(&mut self) -> Result<i16, Self::Error> {
        (*self).measure_mv()
    }
}

/// Blocking ammeter (ampere meter) for measuring current.
pub trait Ammeter: ErrorType {
    /// Measures current in nA (nanoampere).
    ///
    /// This can measure between -9223372036.854775808A and 9223372036.854775807A.
    fn measure_na(&mut self) -> Result<i64, Self::Error>;

    /// Measures current in uA (microampere).
    ///
    /// This can measure between -2147.483648A and 2147.483647A.
    /// If you need to measure a larger range, use [`measure_na`](Ammeter::measure_na) instead.
    ///
    /// When overriding the default implementation, ensure that the measured current is clamped
    /// between [`i32::MIN`] and [`i32::MAX`].
    fn measure_ua(&mut self) -> Result<i32, Self::Error> {
        Ok((self.measure_na()? / 1_000).clamp(i32::MIN.into(), i32::MAX.into()) as i32)
    }

    /// Measures current in mA (milliampere).
    ///
    /// This can measure between between -32.768A and 32.767A.
    /// If you need to measure a larger range,
    /// use [`measure_ua`](Ammeter::measure_ua) or [`measure_na`](Ammeter::measure_na) instead.
    ///
    /// When overriding the default implementation, ensure that the measured voltage is clamped
    /// between [`i16::MIN`] and [`i16::MAX`].
    fn measure_ma(&mut self) -> Result<i16, Self::Error> {
        Ok((self.measure_ua()? / 1_000).clamp(i16::MIN.into(), i16::MAX.into()) as i16)
    }
}

impl<T> Ammeter for &mut T
where
    T: Ammeter + ?Sized,
{
    #[inline]
    fn measure_na(&mut self) -> Result<i64, Self::Error> {
        (*self).measure_na()
    }

    #[inline]
    fn measure_ua(&mut self) -> Result<i32, Self::Error> {
        (*self).measure_ua()
    }

    #[inline]
    fn measure_ma(&mut self) -> Result<i16, Self::Error> {
        (*self).measure_ma()
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
    /// Measurement was clipped.
    Clip(Clip),
    /// A different error occurred. The original error may contain more information.
    Other,
}

/// ADC clip error.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Clip {
    /// Measurement was clipped due to an undershoot of the measurement range.
    Undershoot,
    /// Measurement was clipped due to an overshoot of the measurement range.
    Overshoot,
}

impl Error for ErrorKind {
    #[inline]
    fn kind(&self) -> ErrorKind {
        *self
    }
}

impl Display for ErrorKind {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(
            match self {
                Self::Clip(Clip::Undershoot) => {
                    "Measurement was clipped due to an undershoot of the measurement range."
                }
                Self::Clip(Clip::Overshoot) => {
                    "Measurement was clipped due to an overshoot of the measurement range."
                }
                Self::Other => {
                    "A different error occurred. The original error may contain more information."
                }
            },
            f,
        )
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
