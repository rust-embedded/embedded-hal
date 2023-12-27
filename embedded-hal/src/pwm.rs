//! Pulse Width Modulation (PWM) traits.

#[cfg(feature = "defmt-03")]
use crate::defmt;

/// Error
pub trait Error: core::fmt::Debug {
    /// Convert error to a generic error kind.
    ///
    /// By using this method, errors freely defined by HAL implementations
    /// can be converted to a set of generic errors upon which generic
    /// code can act.
    fn kind(&self) -> ErrorKind;
}

impl Error for core::convert::Infallible {
    #[inline]
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

impl<T: ErrorType + ?Sized> ErrorType for &mut T {
    type Error = T::Error;
}

/// Single PWM channel / pin.
pub trait SetDutyCycle: ErrorType {
    /// Get the maximum duty cycle value.
    ///
    /// This value corresponds to a 100% duty cycle.
    fn max_duty_cycle(&self) -> u16;

    /// Set the duty cycle to `duty / max_duty`.
    ///
    /// The caller is responsible for ensuring that the duty cycle value is less than or equal to the maximum duty cycle value,
    /// as reported by [`max_duty_cycle`].
    ///
    /// [`max_duty_cycle`]: SetDutyCycle::max_duty_cycle
    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error>;

    /// Set the duty cycle to 0%, or always inactive.
    #[inline]
    fn set_duty_cycle_fully_off(&mut self) -> Result<(), Self::Error> {
        self.set_duty_cycle(0)
    }

    /// Set the duty cycle to 100%, or always active.
    #[inline]
    fn set_duty_cycle_fully_on(&mut self) -> Result<(), Self::Error> {
        self.set_duty_cycle(self.max_duty_cycle())
    }

    /// Set the duty cycle to `num / denom`.
    ///
    /// The caller is responsible for ensuring that `num` is less than or equal to `denom`,
    /// and that `denom` is not zero.
    #[inline]
    fn set_duty_cycle_fraction(&mut self, num: u16, denom: u16) -> Result<(), Self::Error> {
        debug_assert!(denom != 0);
        debug_assert!(num <= denom);
        let duty = u32::from(num) * u32::from(self.max_duty_cycle()) / u32::from(denom);

        // This is safe because we know that `num <= denom`, so `duty <= self.max_duty_cycle()` (u16)
        #[allow(clippy::cast_possible_truncation)]
        {
            self.set_duty_cycle(duty as u16)
        }
    }

    /// Set the duty cycle to `percent / 100`
    ///
    /// The caller is responsible for ensuring that `percent` is less than or equal to 100.
    #[inline]
    fn set_duty_cycle_percent(&mut self, percent: u8) -> Result<(), Self::Error> {
        self.set_duty_cycle_fraction(u16::from(percent), 100)
    }
}

impl<T: SetDutyCycle + ?Sized> SetDutyCycle for &mut T {
    #[inline]
    fn max_duty_cycle(&self) -> u16 {
        T::max_duty_cycle(self)
    }

    #[inline]
    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        T::set_duty_cycle(self, duty)
    }

    #[inline]
    fn set_duty_cycle_fully_off(&mut self) -> Result<(), Self::Error> {
        T::set_duty_cycle_fully_off(self)
    }

    #[inline]
    fn set_duty_cycle_fully_on(&mut self) -> Result<(), Self::Error> {
        T::set_duty_cycle_fully_on(self)
    }

    #[inline]
    fn set_duty_cycle_fraction(&mut self, num: u16, denom: u16) -> Result<(), Self::Error> {
        T::set_duty_cycle_fraction(self, num, denom)
    }

    #[inline]
    fn set_duty_cycle_percent(&mut self, percent: u8) -> Result<(), Self::Error> {
        T::set_duty_cycle_percent(self, percent)
    }
}
