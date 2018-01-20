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

/// Single digital input pin
pub trait InputPin {
    /// Checks if the pin is being driven low
    fn is_high(&self) -> bool;

    /// Checks if the pin is being driven high
    fn is_low(&self) -> bool;
}

/// A pin that can switch between input and output modes at runtime
pub trait IoPin {
    /// Pin configured in input mode
    type Input: InputPin;

    /// Pin configured in output mode
    type Output: OutputPin;

    /// Puts the pin in input mode and performs the operations in the closure `f`
    fn as_input<R, F>(&mut self, f: F) -> R
    where
        F: FnOnce(&Self::Input) -> R;

    /// Puts the pin in output mode and performs the operations in the closure `f`
    fn as_output<R, F>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Self::Output) -> R;
}
