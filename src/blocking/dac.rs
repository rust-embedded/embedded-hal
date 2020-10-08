//! Digital-to-analog conversion traits

/// Trait representing a single-channel digital-to-analog converter (DAC).
pub trait SingleChannelDac<Word> {
    /// Error type returned by DAC methods
    type Error;

    /// Output a constant signal, given a bit word.
    fn try_set_value(&mut self, value: Word) -> Result<(), Self::Error>;
}
