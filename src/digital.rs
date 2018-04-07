//! Digital I/O

/// Single digital output pin
pub trait OutputPin {
    /// Sets the pin low
    fn set_low(&mut self);

    /// Sets the pin high
    fn set_high(&mut self);
}

/// Single digital input pin
#[cfg(feature = "unproven")]
pub trait InputPin {
    /// Is the input pin high?
    fn is_high(&self) -> bool;

    /// Is the input pin low?
    fn is_low(&self) -> bool;
}
