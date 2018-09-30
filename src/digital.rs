//! Digital I/O

/// Single digital push-pull output pin. (Infallible version)
///
/// *This version of the trait is now deprecated. Please enable the
/// `"use-fallible-digital-traits"` feature when building embedded-hal to use the new version.
/// In the release after next one, this version will only be available when activating the
/// `"use-infallible-digital-traits"` and it will be removed in the release after that one.*
#[deprecated]
#[cfg(not(feature = "use-fallible-digital-traits"))]
pub trait OutputPin {
    /// Drives the pin low
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be low, e.g. due to external
    /// electrical sources
    fn set_low(&mut self);

    /// Drives the pin high
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be high, e.g. due to external
    /// electrical sources
    fn set_high(&mut self);
}

/// Single digital push-pull output pin
/// (Fallible version. This will become the default after the next release)
///
/// *This trait is available if embedded-hal is built with the `"use-fallible-digital-traits"` feature.*
#[cfg(feature = "use-fallible-digital-traits")]
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
/// *This version of the trait is now deprecated. Please enable the
/// `"use-fallible-digital-traits"` feature when building embedded-hal to use the new version.
/// In the release after next one, this version will only be available when activating the
/// `"use-infallible-digital-traits"` and it will be removed in the release after that one.*
#[deprecated]
#[cfg(all(feature = "unproven", not(feature = "use-fallible-digital-traits")))]
pub trait StatefulOutputPin {
    /// Is the pin in drive high mode?
    ///
    /// *NOTE* this does *not* read the electrical state of the pin
    fn is_set_high(&self) -> bool;

    /// Is the pin in drive low mode?
    ///
    /// *NOTE* this does *not* read the electrical state of the pin
    fn is_set_low(&self) -> bool;
}

/// Push-pull output pin that can read its output state
/// (Fallible version. This will become the default after the next release)
///
/// *This trait is available if embedded-hal is built with the `"unproven"` and
/// `"use-fallible-digital-traits"` features.*
#[cfg(all(feature = "unproven", feature = "use-fallible-digital-traits"))]
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
/// use embedded_hal::digital::{OutputPin, StatefulOutputPin, ToggleableOutputPin};
/// use embedded_hal::digital::toggleable;
///
/// /// A virtual output pin that exists purely in software
/// struct MyPin {
///     state: bool
/// }
///
/// #[cfg(not(feature = "use-fallible-digital-traits"))]
/// impl OutputPin for MyPin {
///    fn set_low(&mut self){
///        self.state = false;
///    }
///    fn set_high(&mut self){
///        self.state = true;
///    }
/// }
///
/// #[cfg(feature = "use-fallible-digital-traits")]
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
/// #[cfg(not(feature = "use-fallible-digital-traits"))]
/// impl StatefulOutputPin for MyPin {
///    fn is_set_low(&self) -> bool {
///        !self.state
///    }
///    fn is_set_high(&self) -> bool {
///        self.state
///    }
/// }
///
/// #[cfg(feature = "use-fallible-digital-traits")]
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
/// #[cfg(not(feature = "use-fallible-digital-traits"))]
/// assert!(pin.is_set_high());
/// #[cfg(feature = "use-fallible-digital-traits")]
/// assert!(pin.is_set_high().unwrap());
/// pin.toggle().unwrap();
/// #[cfg(not(feature = "use-fallible-digital-traits"))]
/// assert!(pin.is_set_low());
/// #[cfg(feature = "use-fallible-digital-traits")]
/// assert!(pin.is_set_low().unwrap());
/// ```
#[cfg(feature = "unproven")]
#[allow(deprecated)]
pub mod toggleable {
    use super::{OutputPin, StatefulOutputPin, ToggleableOutputPin};

    /// Software-driven `toggle()` implementation.
    ///
    /// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
    pub trait Default: OutputPin + StatefulOutputPin {}

    #[cfg(not(feature = "use-fallible-digital-traits"))]
    use void::Void;

    #[cfg(not(feature = "use-fallible-digital-traits"))]
    impl<P> ToggleableOutputPin for P
    where
        P: Default,
    {
        type Error = Void;

        /// Toggle pin output
        fn toggle(&mut self) -> Result<(), Self::Error> {
            if self.is_set_low() {
                self.set_high();
            } else {
                self.set_low();
            }
            Ok(())
        }
    }

    #[cfg(feature = "use-fallible-digital-traits")]
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
