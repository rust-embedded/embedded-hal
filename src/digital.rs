//! Digital I/O

/// Single digital output pin
pub trait OutputPin {
    /// Is the output pin high?
    fn is_high(&self) -> bool;

    /// Is the output pin low?
    fn is_low(&self) -> bool;

    /// Sets the pin low
    fn set_low(&mut self);

    /// Sets the pin high
    fn set_high(&mut self);
}

/// A pin that can switch between input and output modes at runtime
pub trait IoPin {
    /// Signals that a method was used in the wrong mode
    type Error;

    /// Sets the pin low
    ///
    /// **NOTE** Automatically switches the pin to output mode
    fn set_low(&mut self);
    /// Sets the pin high
    ///
    /// **NOTE** Automatically switches the pin to output mode
    fn set_high(&mut self);

    /// Checks if the pin is being driven low
    ///
    /// **NOTE** Automatically switches the pin to input mode
    /// **NOTE** Takes `&mut self` because needs to modify the configuration register
    fn is_low(&mut self) -> bool;
    /// Checks if the pin is being driven high
    ///
    /// **NOTE** Automatically switches the pin to input mode
    /// **NOTE** Takes `&mut self` because needs to modify the configuration register
    fn is_high(&mut self) -> bool;
}
