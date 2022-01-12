//! Blocking SPI API

use super::ErrorType;

/// Blocking transfer with separate buffers
pub trait Transfer<Word = u8>: ErrorType {
    /// Writes and reads simultaneously. `write` is written to the slave on MOSI and
    /// words received on MISO are stored in `read`.
    ///
    /// It is allowed for `read` and `write` to have different lengths, even zero length.
    /// The transfer runs for `max(read.len(), write.len())` words. If `read` is shorter,
    /// incoming words after `read` has been filled will be discarded. If `write` is shorter,
    /// the value of words sent in MOSI after all `write` has been sent is implementation-defined,
    /// typically `0x00`, `0xFF`, or configurable.
    fn transfer(&mut self, read: &mut [Word], write: &[Word]) -> Result<(), Self::Error>;
}

impl<T: Transfer<Word>, Word: Copy> Transfer<Word> for &mut T {
    fn transfer(&mut self, read: &mut [Word], write: &[Word]) -> Result<(), Self::Error> {
        T::transfer(self, read, write)
    }
}

/// Blocking transfer with single buffer (in-place)
pub trait TransferInplace<Word: Copy = u8>: ErrorType {
    /// Writes and reads simultaneously. The contents of `words` are
    /// written to the slave, and the received words are stored into the same
    /// `words` buffer, overwriting it.
    fn transfer_inplace(&mut self, words: &mut [Word]) -> Result<(), Self::Error>;
}

impl<T: TransferInplace<Word>, Word: Copy> TransferInplace<Word> for &mut T {
    fn transfer_inplace(&mut self, words: &mut [Word]) -> Result<(), Self::Error> {
        T::transfer_inplace(self, words)
    }
}

/// Blocking read
pub trait Read<Word: Copy = u8>: ErrorType {
    /// Reads `words` from the slave.
    ///
    /// The word value sent on MOSI during reading is implementation-defined,
    /// typically `0x00`, `0xFF`, or configurable.
    fn read(&mut self, words: &mut [Word]) -> Result<(), Self::Error>;
}

impl<T: Read<Word>, Word: Copy> Read<Word> for &mut T {
    fn read(&mut self, words: &mut [Word]) -> Result<(), Self::Error> {
        T::read(self, words)
    }
}

/// Blocking write
pub trait Write<Word: Copy = u8>: ErrorType {
    /// Writes `words` to the slave, ignoring all the incoming words
    fn write(&mut self, words: &[Word]) -> Result<(), Self::Error>;
}

impl<T: Write<Word>, Word: Copy> Write<Word> for &mut T {
    fn write(&mut self, words: &[Word]) -> Result<(), Self::Error> {
        T::write(self, words)
    }
}

/// Blocking write (iterator version)
pub trait WriteIter<Word: Copy = u8>: ErrorType {
    /// Writes `words` to the slave, ignoring all the incoming words
    fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
    where
        WI: IntoIterator<Item = Word>;
}

impl<T: WriteIter<Word>, Word: Copy> WriteIter<Word> for &mut T {
    fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
    where
        WI: IntoIterator<Item = Word>,
    {
        T::write_iter(self, words)
    }
}

/// Operation for transactional SPI trait
///
/// This allows composition of SPI operations into a single bus transaction
#[derive(Debug, PartialEq)]
pub enum Operation<'a, Word: 'static + Copy = u8> {
    /// Read data into the provided buffer.
    Read(&'a mut [Word]),
    /// Write data from the provided buffer, discarding read data
    Write(&'a [Word]),
    /// Write data out while reading data into the provided buffer
    Transfer(&'a mut [Word], &'a [Word]),
    /// Write data out while reading data into the provided buffer
    TransferInplace(&'a mut [Word]),
}

/// Transactional trait allows multiple actions to be executed
/// as part of a single SPI transaction
pub trait Transactional<Word: 'static + Copy = u8>: ErrorType {
    /// Execute the provided transactions
    fn exec<'a>(&mut self, operations: &mut [Operation<'a, Word>]) -> Result<(), Self::Error>;
}

impl<T: Transactional<Word>, Word: 'static + Copy> Transactional<Word> for &mut T {
    fn exec<'a>(&mut self, operations: &mut [Operation<'a, Word>]) -> Result<(), Self::Error> {
        T::exec(self, operations)
    }
}
