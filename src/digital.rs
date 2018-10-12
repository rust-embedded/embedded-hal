//! Digital I/O.
//!
//! The infallible versions of the traits below are now deprecated. Please enable the
//! `"use-fallible-digital-traits"` feature when building embedded-hal to use the new versions.
//! In the release after next one, the infallible versions will only be available when activating the
//! `"regress-infallible-digital-traits"` and they will be removed in the release after that one.*

/// Single digital push-pull output pin. *(Infallible version)*
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

/// Single digital push-pull output pin.
///
/// *Fallible version. This will become the default after the next release.*
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

/// Push-pull output pin that can read its output state. *(Infallible version)*
///
/// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
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
///
/// *Fallible version. This will become the default after the next release.*
///
/// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
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

/// Output pin that can be toggled.
///
/// *Fallible version. This will become the default after the next release.*
///
/// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
///
/// See [toggleable](toggleable) to use a software implementation if
/// both [OutputPin](trait.OutputPin.html) and
/// [StatefulOutputPin](trait.StatefulOutputPin.html) are
/// implemented. Otherwise, implement this using hardware mechanisms.
#[cfg(all(feature = "unproven", feature = "use-fallible-digital-traits"))]
pub trait ToggleableOutputPin {
    /// Error type
    type Error;

    /// Toggle pin output.
    fn toggle(&mut self) -> Result<(), Self::Error>;
}

/// Output pin that can be toggled. *(Infallible version)*
///
/// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
///
/// See [toggleable](toggleable) to use a software implementation if
/// both [OutputPin](trait.OutputPin.html) and
/// [StatefulOutputPin](trait.StatefulOutputPin.html) are
/// implemented. Otherwise, implement this using hardware mechanisms.
#[deprecated]
#[cfg(all(feature = "unproven", not(feature = "use-fallible-digital-traits")))]
pub trait ToggleableOutputPin {
    /// Toggle pin output.
    fn toggle(&mut self);
}

/// If you can read **and** write the output state, a pin is
/// toggleable by software.
///
/// ```
/// #[allow(deprecated)]
/// use embedded_hal::digital::{OutputPin, StatefulOutputPin, ToggleableOutputPin};
/// use embedded_hal::digital::toggleable;
///
/// /// A virtual output pin that exists purely in software
/// struct MyPin {
///     state: bool
/// }
///
/// #[cfg(not(feature = "use-fallible-digital-traits"))]
/// #[allow(deprecated)]
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
/// #[allow(deprecated)]
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
/// #[cfg(feature = "use-fallible-digital-traits")]
/// {
///     pin.toggle().unwrap();
///     assert!(pin.is_set_high().unwrap());
///     pin.toggle().unwrap();
///     assert!(pin.is_set_low().unwrap());
/// }
/// #[cfg(not(feature = "use-fallible-digital-traits"))]
/// {
///     // deprecated
///     pin.toggle();
///     assert!(pin.is_set_high());
///     pin.toggle();
///     assert!(pin.is_set_low());
/// }
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
    impl<P> ToggleableOutputPin for P
    where
        P: Default,
    {
        /// Toggle pin output
        fn toggle(&mut self) {
            if self.is_set_low() {
                self.set_high();
            } else {
                self.set_low();
            }
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

/// Single digital input pin.
///
/// *Fallible version. This will become the default after the next release.*
///
/// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
#[cfg(all(feature = "unproven", feature = "use-fallible-digital-traits"))]
pub trait InputPin {
    /// Error type
    type Error;

    /// Is the input pin high?
    fn is_high(&self) -> Result<bool, Self::Error>;

    /// Is the input pin low?
    fn is_low(&self) -> Result<bool, Self::Error>;
}

/// Single digital input pin. *(Infallible version)*
///
/// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
#[deprecated]
#[cfg(all(feature = "unproven", not(feature = "use-fallible-digital-traits")))]
pub trait InputPin {
    /// Is the input pin high?
    fn is_high(&self) -> bool;

    /// Is the input pin low?
    fn is_low(&self) -> bool;
}
