//! Serial Peripheral Interface

use core::future::Future;

/// Async transfer
pub trait Transfer<W: 'static> {
    /// Error type
    type Error;

    /// Associated future for the `transfer` method.
    type TransferFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Writes and reads simultaneously. `write` is written to the slave on MOSI and
    /// words received on MISO are stored in `read`.
    ///
    /// It is allowed for `read` and `write` to have different lengths, even zero length.
    /// The transfer runs for `max(read.len(), write.len())` words. If `read` is shorter,
    /// incoming words after `read` has been filled will be discarded. If `write` is shorter,
    /// the value of words sent in MOSI after all `write` has been sent is implementation defined,
    /// typically `0x00`, `0xFF`, or configurable.
    fn transfer<'a>(&'a mut self, write: &'a [W], read: &'a mut [W]) -> Self::TransferFuture<'a>;
}

/// Async transfer in place.
pub trait TransferInPlace<W: 'static> {
    /// Error type
    type Error;

    /// Associated future for the `transfer_inplace` method.
    type TransferInPlaceFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Writes `words` to the slave from the `readwrite` buffer and reads words into the same buffer.
    /// This method uses a single `readwrite` buffer.
    ///
    /// The returned buffer is the initialized `readwrite` buffer.
    fn transfer_inplace<'a>(&'a mut self, words: &'a mut [W]) -> Self::TransferInPlaceFuture<'a>;
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
    fn write<'a>(&'a mut self, write: &'a [W]) -> Self::WriteFuture<'a>;
}

/// Async read
pub trait Read<W: 'static> {
    /// Error type
    type Error;

    /// Associated future for the `read` method.
    type ReadFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Reads words from the slave without specifying any data to write.
    /// The SPI hardware will send data, though what data it sends is not defined
    /// by this trait. Some hardware can configure what values (e.g. 0x00, 0xFF), some cannot.
    ///
    /// The returned buffer is the initialized `words` buffer.
    fn read<'a>(&'a mut self, read: &'a mut [W]) -> Self::ReadFuture<'a>;
}
