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

    /// Configures the pin to operate in input mode
    fn as_input(&mut self);
    /// Configures the pin to operate in output mode
    fn as_output(&mut self);

    /// Sets the pin low
    fn set_low(&mut self) -> Result<(), Self::Error>;
    /// Sets the pin high
    fn set_high(&mut self) -> Result<(), Self::Error>;

    /// Checks if the pin is being driven low
    fn is_low(&self) -> Result<bool, Self::Error>;
    /// Checks if the pin is being driven high
    fn is_high(&self) -> Result<bool, Self::Error>;
}
