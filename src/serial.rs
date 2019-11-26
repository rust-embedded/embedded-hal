//! Serial interface

use core::task::Poll;

/// Read half of a serial interface
///
/// Some serial interfaces support different data sizes (8 bits, 9 bits, etc.);
/// This can be encoded in this trait via the `Word` type parameter.
pub trait Read<Word> {
    /// Read error
    type Error;

    /// Reads a single word from the serial interface
    fn read(&mut self) -> Poll<Result<Word, Self::Error>>;
}

/// Write half of a serial interface
pub trait Write<Word> {
    /// Write error
    type Error;

    /// Writes a single word to the serial interface
    fn write(&mut self, word: Word) -> Poll<Result<(), Self::Error>>;

    /// Ensures that none of the previously written words are still buffered
    fn flush(&mut self) -> Poll<Result<(), Self::Error>>;
}
