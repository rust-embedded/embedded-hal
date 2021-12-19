//! Serial Peripheral Interface

use core::future::Future;

pub use embedded_hal::spi::blocking::Operation;
pub use embedded_hal::spi::{
    Error, ErrorKind, ErrorType, Mode, Phase, Polarity, MODE_0, MODE_1, MODE_2, MODE_3,
};

/// Read-only SPI
pub trait Read<W: 'static = u8>: ErrorType {
    /// Associated future for the `read` method.
    type ReadFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Reads `words` from the slave.
    ///
    /// The word value sent on MOSI during reading is implementation-defined,
    /// typically `0x00`, `0xFF`, or configurable.
    fn read<'a>(&'a mut self, words: &'a mut [W]) -> Self::ReadFuture<'a>;

    /// Associated future for the `read_batch` method.
    type ReadBatchFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Reads all slices in `words` from the slave as part of a single SPI transaction.
    ///
    /// The word value sent on MOSI during reading is implementation-defined,
    /// typically `0x00`, `0xFF`, or configurable.
    fn read_batch<'a>(&'a mut self, words: &'a mut [&'a mut [W]]) -> Self::ReadBatchFuture<'a>;
}

impl<T: Read<W>, W: 'static> Read<W> for &mut T {
    type ReadFuture<'a>
    where
        Self: 'a,
    = T::ReadFuture<'a>;

    fn read<'a>(&'a mut self, words: &'a mut [W]) -> Self::ReadFuture<'a> {
        T::read(self, words)
    }

    type ReadBatchFuture<'a>
    where
        Self: 'a,
    = T::ReadBatchFuture<'a>;

    fn read_batch<'a>(&'a mut self, words: &'a mut [&'a mut [W]]) -> Self::ReadBatchFuture<'a> {
        T::read_batch(self, words)
    }
}

/// Write-only SPI
pub trait Write<W: 'static = u8>: ErrorType {
    /// Associated future for the `write` method.
    type WriteFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Writes `words` to the slave, ignoring all the incoming words
    fn write<'a>(&'a mut self, words: &'a [W]) -> Self::WriteFuture<'a>;

    /// Associated future for the `write_batch` method.
    type WriteBatchFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Writes all slices in `words` to the slave as part of a single SPI transaction, ignoring all the incoming words
    fn write_batch<'a>(&'a mut self, words: &'a [&'a [W]]) -> Self::WriteBatchFuture<'a>;
}

impl<T: Write<W>, W: 'static> Write<W> for &mut T {
    type WriteFuture<'a>
    where
        Self: 'a,
    = T::WriteFuture<'a>;

    fn write<'a>(&'a mut self, words: &'a [W]) -> Self::WriteFuture<'a> {
        T::write(self, words)
    }

    type WriteBatchFuture<'a>
    where
        Self: 'a,
    = T::WriteBatchFuture<'a>;

    fn write_batch<'a>(&'a mut self, words: &'a [&'a [W]]) -> Self::WriteBatchFuture<'a> {
        T::write_batch(self, words)
    }
}

/// Read-write SPI
pub trait ReadWrite<W: 'static = u8>: Read<W> + Write<W> {
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
    /// the value of words sent in MOSI after all `write` has been sent is implementation-defined,
    /// typically `0x00`, `0xFF`, or configurable.
    fn transfer<'a>(&'a mut self, read: &'a mut [W], write: &'a [W]) -> Self::TransferFuture<'a>;

    /// Associated future for the `transfer_in_place` method.
    type TransferInPlaceFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Writes and reads simultaneously. The contents of `words` are
    /// written to the slave, and the received words are stored into the same
    /// `words` buffer, overwriting it.
    fn transfer_in_place<'a>(&'a mut self, words: &'a mut [W]) -> Self::TransferInPlaceFuture<'a>;

    /// Associated future for the `batch` method.
    type BatchFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Execute multiple actions as part of a single SPI transaction
    fn batch<'a>(&'a mut self, operations: &'a mut [Operation<'a, W>]) -> Self::BatchFuture<'a>;
}

impl<T: ReadWrite<W>, W: 'static> ReadWrite<W> for &mut T {
    type TransferFuture<'a>
    where
        Self: 'a,
    = T::TransferFuture<'a>;

    fn transfer<'a>(&'a mut self, read: &'a mut [W], write: &'a [W]) -> Self::TransferFuture<'a> {
        T::transfer(self, read, write)
    }

    type TransferInPlaceFuture<'a>
    where
        Self: 'a,
    = T::TransferInPlaceFuture<'a>;

    fn transfer_in_place<'a>(&'a mut self, words: &'a mut [W]) -> Self::TransferInPlaceFuture<'a> {
        T::transfer_in_place(self, words)
    }

    type BatchFuture<'a>
    where
        Self: 'a,
    = T::BatchFuture<'a>;

    fn batch<'a>(&'a mut self, operations: &'a mut [Operation<'a, W>]) -> Self::BatchFuture<'a> {
        T::batch(self, operations)
    }
}
