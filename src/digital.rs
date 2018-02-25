//! Digital I/O

/// Single digital output pin
pub trait OutputPin {
    /// Is the output pin high?
    fn is_high(&self) -> bool {
        !self.is_low()
    }

    /// Is the output pin low?
    fn is_low(&self) -> bool;

    /// Sets the pin low
    fn set_low(&mut self);

    /// Sets the pin high
    fn set_high(&mut self);

    /// Sets the pin to state
    fn set_state(&mut self, state: bool) {
        if state {
            self.set_high();
        } else {
            self.set_low();
        }
    }
    
    /// Toggles the pin state
    fn toggle(&mut self) {
        if self.is_low() {
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
