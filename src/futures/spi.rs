//! Serial Peripheral Interface

use core::{future::Future, mem::MaybeUninit};

/// Async read + write
pub trait ReadWrite<Word: 'static> {
    /// Error type
    type Error;

    /// Associated future for the `transfer` method.
    type ReadWriteFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Writes `words` to the slave from the `write` buffer. Puts the words returned in the `read` buffer.
    /// This method uses separate `write` and `read` buffers.
    fn readwrite<'a>(&'a mut self, write: &'a [Word], read: &'a mut [MaybeUninit<Word>]) -> Self::ReadWriteFuture<'a>;
}

/// Async read + write in place.
pub trait ReadWriteInPlace<Word: 'static> {
    /// Error type
    type Error;

    /// Associated future for the `transfer` method.
    type ReadWriteInPlaceFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Writes `words` to the slave from the `readwrite` buffer and reads words into the same buffer.
    /// This method uses a single `readwrite` buffer.
    fn readwrite_inplace<'a>(&'a mut self, readwrite: &'a mut [MaybeUninit<Word>]) -> Self::ReadWriteInPlaceFuture<'a>;
}

/// Async write
pub trait Write<W> {
    /// Error type
    type Error;

    /// Associated future for the `write` method.
    type WriteFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Writes `words` to the slave, ignoring all the incoming words
    fn write<'a>(&'a mut self, words: &'a [W]) -> Self::WriteFuture<'a>;
}

/// Async read
pub trait Read<W> {
    /// Error type
    type Error;

    /// Associated future for the `read` method.
    type ReadFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Reads words from the slave without specifying any data to write.
    /// The SPI hardware will send data, though what data it sends is not defined
    /// by this trait. Some hardware can configure what values (e.g. all zeroes, all ones), some cannot.
    fn read<'a>(&'a mut self, words: &'a mut [MaybeUninit<W>]) -> Self::ReadFuture<'a>;
}
