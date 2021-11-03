//! Blocking SPI API

use super::ErrorType;

/// Blocking read-only SPI
pub trait Read<W = u8>: ErrorType {
    /// Reads `words` from the slave.
    ///
    /// The word value sent on MOSI during reading is implementation-defined,
    /// typically `0x00`, `0xFF`, or configurable.
    fn read(&mut self, words: &mut [W]) -> Result<(), Self::Error>;

    /// Reads all slices in `words` from the slave as part of a single SPI transaction.
    ///
    /// The word value sent on MOSI during reading is implementation-defined,
    /// typically `0x00`, `0xFF`, or configurable.
    fn read_batch(&mut self, words: &mut [&mut [W]]) -> Result<(), Self::Error>;
}

impl<T: Read<W>, W> Read<W> for &mut T {
    fn read(&mut self, words: &mut [W]) -> Result<(), Self::Error> {
        T::read(self, words)
    }

    fn read_batch(&mut self, words: &mut [&mut [W]]) -> Result<(), Self::Error> {
        T::read_batch(self, words)
    }
}

/// Blocking write-only SPI
pub trait Write<W = u8>: ErrorType {
    /// Writes `words` to the slave, ignoring all the incoming words
    fn write(&mut self, words: &[W]) -> Result<(), Self::Error>;

    /// Writes all slices in `words` to the slave as part of a single SPI transaction, ignoring all the incoming words
    fn write_batch(&mut self, words: &[&[W]]) -> Result<(), Self::Error>;

    /// Writes `words` to the slave, ignoring all the incoming words
    fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
    where
        WI: IntoIterator<Item = W>;
}

impl<T: Write<W>, W> Write<W> for &mut T {
    fn write(&mut self, words: &[W]) -> Result<(), Self::Error> {
        T::write(self, words)
    }

    fn write_batch(&mut self, words: &[&[W]]) -> Result<(), Self::Error> {
        T::write_batch(self, words)
    }

    fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
    where
        WI: IntoIterator<Item = W>,
    {
        T::write_iter(self, words)
    }
}

/// Operation for transactional SPI trait
///
/// This allows composition of SPI operations into a single bus transaction
#[derive(Debug, PartialEq)]
pub enum Operation<'a, W: 'static = u8> {
    /// Read data into the provided buffer.
    Read(&'a mut [W]),
    /// Write data from the provided buffer, discarding read data
    Write(&'a [W]),
    /// Write data out while reading data into the provided buffer
    Transfer(&'a mut [W], &'a [W]),
    /// Write data out while reading data into the provided buffer
    TransferInplace(&'a mut [W]),
}

/// Blocking read-write SPI
pub trait ReadWrite<W = u8>: Read<W> + Write<W> {
    /// Writes and reads simultaneously. `write` is written to the slave on MOSI and
    /// words received on MISO are stored in `read`.
    ///
    /// It is allowed for `read` and `write` to have different lengths, even zero length.
    /// The transfer runs for `max(read.len(), write.len())` words. If `read` is shorter,
    /// incoming words after `read` has been filled will be discarded. If `write` is shorter,
    /// the value of words sent in MOSI after all `write` has been sent is implementation-defined,
    /// typically `0x00`, `0xFF`, or configurable.
    fn transfer(&mut self, read: &mut [W], write: &[W]) -> Result<(), Self::Error>;

    /// Writes and reads simultaneously. The contents of `words` are
    /// written to the slave, and the received words are stored into the same
    /// `words` buffer, overwriting it.
    fn transfer_inplace(&mut self, words: &mut [W]) -> Result<(), Self::Error>;

    /// Execute multiple actions as part of a single SPI transaction
    fn exec<'a>(&mut self, operations: &mut [Operation<'a, W>]) -> Result<(), Self::Error>;
}

impl<T: ReadWrite<W>, W> ReadWrite<W> for &mut T {
    fn transfer(&mut self, read: &mut [W], write: &[W]) -> Result<(), Self::Error> {
        T::transfer(self, read, write)
    }

    fn transfer_inplace(&mut self, words: &mut [W]) -> Result<(), Self::Error> {
        T::transfer_inplace(self, words)
    }

    fn exec<'a>(&mut self, operations: &mut [Operation<'a, W>]) -> Result<(), Self::Error> {
        T::exec(self, operations)
    }
}
