//! Trait for digital to analog conversion

/// Represents a single DAC channel.
pub trait DAC<WORD> {
    /// Error type returned by DAC methods
    type Error;

    /// Set the output of the DAC
    fn try_set_output(&mut self, value: WORD) -> Result<(), Self::Error>;
}
