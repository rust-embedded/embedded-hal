//! Serial Peripheral Interface

use super::ErrorType;

/// Full duplex (master mode)
///
/// # Notes
///
/// - It's the task of the user of this interface to manage the slave select lines
///
/// - Due to how full duplex SPI works each `read` call must be preceded by a `write` call.
///
/// - `read` calls only return the data received with the last `write` call.
/// Previously received data is discarded
///
/// - Data is only guaranteed to be clocked out when the `read` call succeeds.
/// The slave select line shouldn't be released before that.
///
/// - Some SPIs can work with 8-bit *and* 16-bit words. You can overload this trait with different
/// `Word` types to allow operation in both modes.
pub trait FullDuplex<Word: Copy = u8>: ErrorType {
    /// Reads the word stored in the shift register
    ///
    /// **NOTE** A word must be sent to the slave before attempting to call this
    /// method.
    fn read(&mut self) -> nb::Result<Word, Self::Error>;

    /// Writes a word to the slave
    fn write(&mut self, word: Word) -> nb::Result<(), Self::Error>;
}

impl<T: FullDuplex<Word>, Word: Copy> FullDuplex<Word> for &mut T {
    fn read(&mut self) -> nb::Result<Word, Self::Error> {
        T::read(self)
    }

    fn write(&mut self, word: Word) -> nb::Result<(), Self::Error> {
        T::write(self, word)
    }
}
