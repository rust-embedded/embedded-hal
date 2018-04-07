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
#[cfg(feature = "unproven")]
pub trait InputPin {
    /// Is the input pin high?
    fn is_high(&self) -> bool;

    /// Is the input pin low?
    fn is_low(&self) -> bool;
}

/// Pins that can switch between input and output modes at runtime
#[cfg(feature = "unproven")]
pub trait IoPin {
    /// Used by [`into_input()`](#tymethod.into_input)
    ///
    /// In addition to being an [`InputPin`](trait.InputPin.html), the
    /// target type must implement `IoPin` so that the mode can be
    /// changed again.
    type Input: InputPin + IoPin<Input = Self::Input, Output = Self::Output>;

    /// Used by [`into_output()`](#tymethod.into_output)
    ///
    /// In addition to being an [`OutputPin`](trait.OutputPin.html),
    /// the target type must implement `IoPin` so that the mode can be
    /// changed again.
    type Output: OutputPin + IoPin<Input = Self::Input, Output = Self::Output>;

    /// Configure as [`InputPin`](trait.InputPin.html)
    fn into_input(self) -> Self::Input;

    /// Configure as [`OutputPin`](trait.OutputPin.html)
    fn into_output(self) -> Self::Output;
}
