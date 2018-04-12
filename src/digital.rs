//! Digital I/O

/// Single digital output pin
pub trait OutputPin {
    /// Sets the pin low
    fn set_low(&mut self);

    /// Sets the pin high
    fn set_high(&mut self);
}

/// Output pin that can read its output state
#[cfg(feature = "unproven")]
pub trait StatefulOutputPin {
    /// Is the pin set to high?
    fn is_set_high(&self) -> bool;

    /// Is the pin set to low?
    fn is_set_low(&self) -> bool;
}

/// Output pin that can be toggled
///
/// See [toggleable](toggleable) to use a software implementation if
/// both [OutputPin](trait.OutputPin.html) and
/// [StatefulOutputPin](trait.StatefulOutputPin.html) are
/// implemented. Otherwise, implement this using hardware mechanisms.
#[cfg(feature = "unproven")]
pub trait ToggleableOutputPin {
    /// Toggle pin output.
    fn toggle(&mut self);
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
/// impl OutputPin for MyPin {
///    fn set_low(&mut self) {
///        self.state = false;
///    }
///    fn set_high(&mut self) {
///        self.state = true;
///    }
/// }
///
/// impl StatefulOutputPin for MyPin {
///    fn is_set_low(&self) -> bool {
///        !self.state
///    }
///    fn is_set_high(&self) -> bool {
///        self.state
///    }
/// }
///
/// /// Opt-in to the software implementation.
/// impl toggleable::Default for MyPin {}
///
/// let mut pin = MyPin { state: false };
/// pin.toggle();
/// assert!(pin.is_set_high());
/// pin.toggle();
/// assert!(pin.is_set_low());
/// ```
#[cfg(feature = "unproven")]
pub mod toggleable {
    use super::{OutputPin, StatefulOutputPin, ToggleableOutputPin};

    /// Software-driven `toggle()` implementation.
    pub trait Default: OutputPin + StatefulOutputPin {}

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
}

/// Single digital input pin
#[cfg(feature = "unproven")]
pub trait InputPin {
    /// Is the input pin high?
    fn is_high(&self) -> bool;

    /// Is the input pin low?
    fn is_low(&self) -> bool;
}
