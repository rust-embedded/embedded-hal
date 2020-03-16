//! Digital I/O

/// Single digital push-pull output pin
pub trait OutputPin {
    /// Error type
    type Error;

    /// Drives the pin low
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be low, e.g. due to external
    /// electrical sources
    fn try_set_low(&mut self) -> Result<(), Self::Error>;

    /// Drives the pin high
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be high, e.g. due to external
    /// electrical sources
    fn try_set_high(&mut self) -> Result<(), Self::Error>;
}

/// Push-pull output pin that can read its output state
pub trait StatefulOutputPin: OutputPin {
    /// Is the pin in drive high mode?
    ///
    /// *NOTE* this does *not* read the electrical state of the pin
    fn try_is_set_high(&self) -> Result<bool, Self::Error>;

    /// Is the pin in drive low mode?
    ///
    /// *NOTE* this does *not* read the electrical state of the pin
    fn try_is_set_low(&self) -> Result<bool, Self::Error>;
}

/// Output pin that can be toggled
///
/// See [toggleable](toggleable) to use a software implementation if
/// both [OutputPin](trait.OutputPin.html) and
/// [StatefulOutputPin](trait.StatefulOutputPin.html) are
/// implemented. Otherwise, implement this using hardware mechanisms.
pub trait ToggleableOutputPin {
    /// Error type
    type Error;

    /// Toggle pin output.
    fn try_toggle(&mut self) -> Result<(), Self::Error>;
}

/// If you can read **and** write the output state, a pin is
/// toggleable by software.
///
/// ```
/// use embedded_hal::digital::{OutputPin, StatefulOutputPin, ToggleableOutputPin};
/// use embedded_hal::digital::toggleable;
/// use core::convert::Infallible;
///
/// /// A virtual output pin that exists purely in software
/// struct MyPin {
///     state: bool
/// }
///
/// impl OutputPin for MyPin {
///    type Error = Infallible;
///
///    fn try_set_low(&mut self) -> Result<(), Self::Error> {
///        self.state = false;
///        Ok(())
///    }
///    fn try_set_high(&mut self) -> Result<(), Self::Error> {
///        self.state = true;
///        Ok(())
///    }
/// }
///
/// impl StatefulOutputPin for MyPin {
///    fn try_is_set_low(&self) -> Result<bool, Self::Error> {
///        Ok(!self.state)
///    }
///    fn try_is_set_high(&self) -> Result<bool, Self::Error> {
///        Ok(self.state)
///    }
/// }
///
/// /// Opt-in to the software implementation.
/// impl toggleable::Default for MyPin {}
///
/// let mut pin = MyPin { state: false };
/// pin.try_toggle().unwrap();
/// assert!(pin.try_is_set_high().unwrap());
/// pin.try_toggle().unwrap();
/// assert!(pin.try_is_set_low().unwrap());
/// ```
pub mod toggleable {
    use super::{OutputPin, StatefulOutputPin, ToggleableOutputPin};

    /// Software-driven `toggle()` implementation.
    pub trait Default: OutputPin + StatefulOutputPin {}

    impl<P> ToggleableOutputPin for P
    where
        P: Default,
    {
        type Error = P::Error;

        /// Toggle pin output
        fn try_toggle(&mut self) -> Result<(), Self::Error> {
            if self.try_is_set_low()? {
                self.try_set_high()
            } else {
                self.try_set_low()
            }
        }
    }
}

/// Single digital input pin
pub trait InputPin {
    /// Error type
    type Error;

    /// Is the input pin high?
    fn try_is_high(&self) -> Result<bool, Self::Error>;

    /// Is the input pin low?
    fn try_is_low(&self) -> Result<bool, Self::Error>;
}
