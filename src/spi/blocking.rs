//! Blocking SPI API

/// Blocking transfer
pub trait Transfer<W = u8> {
    /// Error type
    type Error: crate::spi::Error;

    /// Writes and reads simultaneously. The contents of `words` are
    /// written to the slave, and the received words are stored into the same
    /// `words` buffer, overwriting it.
    fn transfer(&mut self, words: &mut [W]) -> Result<(), Self::Error>;
}

impl<T: Transfer<W>, W> Transfer<W> for &mut T {
    type Error = T::Error;

    fn transfer(&mut self, words: &mut [W]) -> Result<(), Self::Error> {
        T::transfer(self, words)
    }
}

/// Blocking write
pub trait Write<W = u8> {
    /// Error type
    type Error: crate::spi::Error;

    /// Writes `words` to the slave, ignoring all the incoming words
    fn write(&mut self, words: &[W]) -> Result<(), Self::Error>;
}

impl<T: Write<W>, W> Write<W> for &mut T {
    type Error = T::Error;

    fn write(&mut self, words: &[W]) -> Result<(), Self::Error> {
        T::write(self, words)
    }
}

/// Blocking write (iterator version)
pub trait WriteIter<W = u8> {
    /// Error type
    type Error: crate::spi::Error;

    /// Writes `words` to the slave, ignoring all the incoming words
    fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
    where
        WI: IntoIterator<Item = W>;
}

impl<T: WriteIter<W>, W> WriteIter<W> for &mut T {
    type Error = T::Error;

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
    /// Write data from the provided buffer, discarding read data
    Write(&'a [W]),
    /// Write data out while reading data into the provided buffer
    Transfer(&'a mut [W]),
}

/// Transactional trait allows multiple actions to be executed
/// as part of a single SPI transaction
pub trait Transactional<W: 'static = u8> {
    /// Associated error type
    type Error: crate::spi::Error;

    /// Execute the provided transactions
    fn exec<'a>(&mut self, operations: &mut [Operation<'a, W>]) -> Result<(), Self::Error>;
}

impl<T: Transactional<W>, W: 'static> Transactional<W> for &mut T {
    type Error = T::Error;

    fn exec<'a>(&mut self, operations: &mut [Operation<'a, W>]) -> Result<(), Self::Error> {
        T::exec(self, operations)
    }
}
