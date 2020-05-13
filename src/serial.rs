//! Serial interface

use nb;

/// Read half of a serial interface
///
/// Some serial interfaces support different data sizes (8 bits, 9 bits, etc.);
/// This can be encoded in this trait via the `Word` type parameter.
pub trait Read<Word> {
    /// Read error
    type Error;

    /// Reads a single word from the serial interface
    fn try_read(&mut self) -> nb::Result<Word, Self::Error>;
}

/// Write half of a serial interface
pub trait Write<Word> {
    /// Write error
    type Error;

    /// Writes a single word to the serial interface
    fn try_write(&mut self, word: Word) -> nb::Result<(), Self::Error>;

    /// Ensures that none of the previously written words are still buffered
    fn try_flush(&mut self) -> nb::Result<(), Self::Error>;
}

/// Enable changing the baudrate after initiation of serial interface
pub trait ConfigureBaud {
    /// Baudrate type
    type BaudRate;
    /// Set baud error
    type Error;
    /// Change baudrate
    fn set_baudrate(&mut self, baudrate: Self::BaudRate) -> Result<(), Self::Error>;
}