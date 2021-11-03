//! Serial Peripheral Interface

use super::{SpiWord, U8};

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
/// `W` types to allow operation in both modes.
pub trait FullDuplex<W: SpiWord = U8> {
    /// An enumeration of SPI errors
    type Error: crate::spi::Error;

    /// Reads the word stored in the shift register
    ///
    /// **NOTE** A word must be sent to the slave before attempting to call this
    /// method.
    fn read(&mut self) -> nb::Result<W::Data, Self::Error>;

    /// Writes a word to the slave
    fn write(&mut self, word: W::Data) -> nb::Result<(), Self::Error>;
}

impl<T: FullDuplex<W>, W> FullDuplex<W> for &mut T
where
    W: SpiWord,
{
    type Error = T::Error;

    fn read(&mut self) -> nb::Result<W::Data, Self::Error> {
        T::read(self)
    }

    fn write(&mut self, word: W::Data) -> nb::Result<(), Self::Error> {
        T::write(self, word)
    }
}
