//! Serial interface

use core::future::Future;
pub use embedded_hal::serial::{Error, ErrorKind};

/// Read half of a serial interface
///
/// Some serial interfaces support different data sizes (8 bits, 9 bits, etc.);
/// This can be encoded in this trait via the `Word` type parameter.
pub trait Read<Word: 'static = u8> {
    /// Read error
    type Error: Error;

    /// The future associated with the `read` method.
    type ReadFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Reads words from the serial interface into the supplied slice.
    fn read<'a>(&'a mut self, read: &'a mut [Word]) -> Self::ReadFuture<'a>;
}

impl<T: Read<Word>, Word: 'static> Read<Word> for &mut T {
    type Error = T::Error;
    type ReadFuture<'a>
    where
        Self: 'a,
    = T::ReadFuture<'a>;

    fn read<'a>(&'a mut self, read: &'a mut [Word]) -> Self::ReadFuture<'a> {
        T::read(self, read)
    }
}
/// Write half of a serial interface
pub trait Write<Word: 'static = u8> {
    /// Write error
    type Error: Error;

    /// The future associated with the `write` method.
    type WriteFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// The future associated with the `flush` method.
    type FlushFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Writes a single word to the serial interface
    fn write<'a>(&'a mut self, words: &'a [Word]) -> Self::WriteFuture<'a>;

    /// Ensures that none of the previously written words are still buffered
    fn flush<'a>(&'a mut self) -> Self::FlushFuture<'a>;
}

impl<T: Write<Word>, Word: 'static> Write<Word> for &mut T {
    type Error = T::Error;
    type WriteFuture<'a>
    where
        Self: 'a,
    = T::WriteFuture<'a>;
    type FlushFuture<'a>
    where
        Self: 'a,
    = T::FlushFuture<'a>;

    fn write<'a>(&'a mut self, words: &'a [Word]) -> Self::WriteFuture<'a> {
        T::write(self, words)
    }

    fn flush<'a>(&'a mut self) -> Self::FlushFuture<'a> {
        T::flush(self)
    }
}
