//! Digital I/O
//!
//! Version 2 / fallible traits. Infallible implementations should set Error to `!`.

/// Single digital push-pull output pin
pub trait OutputPin {
    /// Error type
    type Error;

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
}

/// Push-pull output pin that can read its output state
///
/// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
#[cfg(feature = "unproven")]
pub trait StatefulOutputPin : OutputPin {
    /// Is the pin in drive high mode?
    ///
    /// *NOTE* this does *not* read the electrical state of the pin
    fn is_set_high(&self) -> Result<bool, Self::Error>;

    /// Is the pin in drive low mode?
    ///
    /// *NOTE* this does *not* read the electrical state of the pin
    fn is_set_low(&self) -> Result<bool, Self::Error>;
}

/// Output pin that can be toggled
///
/// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
///
/// See [toggleable](toggleable) to use a software implementation if
/// both [OutputPin](trait.OutputPin.html) and
/// [StatefulOutputPin](trait.StatefulOutputPin.html) are
/// implemented. Otherwise, implement this using hardware mechanisms.
#[cfg(feature = "unproven")]
pub trait ToggleableOutputPin {
    /// Error type
    type Error;

    /// Toggle pin output.
    fn toggle(&mut self) -> Result<(), Self::Error>;
}

/// If you can read **and** write the output state, a pin is
/// toggleable by software.
///
/// ```
/// use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin, ToggleableOutputPin};
/// use embedded_hal::digital::v2::toggleable;
///
/// /// A virtual output pin that exists purely in software
/// struct MyPin {
///     state: bool
/// }
///
/// impl OutputPin for MyPin {
///    type Error = void::Void;
///
///    fn set_low(&mut self) -> Result<(), Self::Error> {
///        self.state = false;
///        Ok(())
///    }
///    fn set_high(&mut self) -> Result<(), Self::Error> {
///        self.state = true;
///        Ok(())
///    }
/// }
///
/// impl StatefulOutputPin for MyPin {
///    fn is_set_low(&self) -> Result<bool, Self::Error> {
///        Ok(!self.state)
///    }
///    fn is_set_high(&self) -> Result<bool, Self::Error> {
///        Ok(self.state)
///    }
/// }
///
/// /// Opt-in to the software implementation.
/// impl toggleable::Default for MyPin {}
///
/// let mut pin = MyPin { state: false };
/// pin.toggle().unwrap();
/// assert!(pin.is_set_high().unwrap());
/// pin.toggle().unwrap();
/// assert!(pin.is_set_low().unwrap());
/// ```
#[cfg(feature = "unproven")]
pub mod toggleable {
    use super::{OutputPin, StatefulOutputPin, ToggleableOutputPin};

    /// Software-driven `toggle()` implementation.
    ///
    /// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
    pub trait Default: OutputPin + StatefulOutputPin {}

    impl<P> ToggleableOutputPin for P
    where
        P: Default,
    {
        type Error = P::Error;

        /// Toggle pin output
        fn toggle(&mut self) -> Result<(), Self::Error> {
            if self.is_set_low()? {
                self.set_high()
            } else {
                self.set_low()
            }
        }
    }
}

/// A digital output "port"
///
/// `Width` is the size of the port; it could be `u8` for an 8-bit parallel
/// port, `u16` for a 16-bit one, etc.
///
/// **NOTE** The "port" doesn't necessarily has to match a hardware GPIO port;
/// it could for instance be a 4-bit ports made up of non contiguous pins, say
/// `PA0`, `PA3`, `PA10` and `PA13`.
#[cfg(feature = "unproven")]
pub trait OutputPort<Width> {
    /// Error type
    type Error;
    /// Outputs `word` on the port pins
    ///
    /// # Contract
    ///
    /// The state of all the port pins will change atomically ("at the same time"). This usually
    /// means that state of all the pins will be changed in a single register operation.
    fn write(&mut self, word: Width) -> Result<(), Self::Error>;
}

/// Single digital input pin
///
/// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
#[cfg(feature = "unproven")]
pub trait InputPin {
    /// Error type
    type Error;

    /// Is the input pin high?
    fn is_high(&self) -> Result<bool, Self::Error>;

    /// Is the input pin low?
    fn is_low(&self) -> Result<bool, Self::Error>;
}
