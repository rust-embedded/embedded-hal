//! Blocking I2S API

/// Blocking read
pub trait Read<W> {
    /// Error type
    type Error;

    /// Reads enough bytes from the slave to fill `left_words` and `right_words`.
    fn try_read<'w>(
        &mut self,
        left_words: &'w mut [W],
        right_words: &'w mut [W],
    ) -> Result<(), Self::Error>;
}

/// Blocking write
pub trait Write<W> {
    /// Error type
    type Error;

    /// Sends `left_words` and `right_words` to the slave.
    fn try_write<'w>(
        &mut self,
        left_words: &'w [W],
        right_words: &'w [W],
    ) -> Result<(), Self::Error>;
}

/// Blocking write (iterator version)
pub trait WriteIter<W> {
    /// Error type
    type Error;

    /// Sends `left_words` and `right_words` to the slave.
    fn try_write<LW, RW>(&mut self, left_words: LW, right_words: RW) -> Result<(), Self::Error>
    where
        LW: IntoIterator<Item = W>,
        RW: IntoIterator<Item = W>;
}
