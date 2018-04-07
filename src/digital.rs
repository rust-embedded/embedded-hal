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
trait StatefulOutputPin: OutputPin {
    /// Is the pin set to high?
    fn is_set_high(&self) -> bool;

    /// Is the pin set to low?
    fn is_set_low(&self) -> bool;

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
