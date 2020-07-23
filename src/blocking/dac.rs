//! Blocking DAC trait for single channel digital to analog conversion

/// A single DAC channel. Word is the type used to represent a single sample, this would typically
/// be u8, u16 or u32.
/// Note that not all bits will always be used. A 12 bit DAC for example will probably use u16 here
/// <DISCUSSION> should we prescribe to use the most significant bits here for compat between word
///   sizes?
pub trait DAC<WORD> {
    /// Error type returned by DAC methods
    type Error;

    /// Set the output of the DAC
    fn try_set_output(&mut self, value: WORD) -> Result<(), Self::Error>;
}
