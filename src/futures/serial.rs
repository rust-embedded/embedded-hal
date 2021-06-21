//! Serial interface

use core::future::Future;

/// Read half of a serial interface
///
/// Some serial interfaces support different data sizes (8 bits, 9 bits, etc.);
/// This can be encoded in this trait via the `Word` type parameter.
pub trait Read<Word> {
    /// Read error
    type Error;

    /// The future associated with the `read` method.
    type ReadFuture<'a>: Future<Output=Result<Word, Self::Error>> + 'a
    where
        Self: 'a;

    /// Reads a single word from the serial interface
    fn read<'a>(&'a mut self) -> Self::ReadFuture<'a>;
}

/// Write half of a serial interface
pub trait Write<Word> {
    /// Write error
    type Error;

    /// The future associated with the `write` method.
    type WriteFuture<'a>: Future<Output=Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// The future associated with the `flush` method.
    type FlushFuture<'a>: Future<Output=Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Writes a single word to the serial interface
    fn write<'a>(&'a mut self, word: Word) -> Self::WriteFuture<'a>;

    /// Ensures that none of the previously written words are still buffered
    fn flush<'a>(&'a mut self) -> Self::FlushFuture<'a>;
}
