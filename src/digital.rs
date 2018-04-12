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
trait StatefulOutputPin {
    /// Is the pin set to high?
    fn is_set_high(&self) -> bool;

    /// Is the pin set to low?
    fn is_set_low(&self) -> bool;
}

/// Output pin that can be toggled
#[cfg(feature = "unproven")]
trait ToggleableOutputPin {
    /// Toggle pin output
    fn toggle(&mut self);
}

/// If you can read **and** write the output state, a pin is
/// toggleable by software. You may override the `toggle()` method
/// with a hardware implementation.
#[cfg(feature = "unproven")]
impl<PIN: OutputPin + StatefulOutputPin> ToggleableOutputPin for PIN {
    /// Toggle pin output
    fn toggle(&mut self) {
        if self.is_set_low() {
            self.set_high();
        } else {
            self.set_low();
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
