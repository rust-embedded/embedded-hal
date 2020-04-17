//! I2S - Inter-IC Sound Interface

use nb;

/// Full duplex
pub trait FullDuplex<Word> {
    /// Error type
    type Error;

    /// Reads the left word and right word available.
    ///
    /// The order is in the result is `(left_word, right_word)`
    fn try_read(&mut self) -> nb::Result<(Word, Word), Self::Error>;

    /// Sends a left word and a right word to the slave.
    fn try_send(&mut self, left_word: Word, right_word: Word) -> nb::Result<(), Self::Error>;
}
