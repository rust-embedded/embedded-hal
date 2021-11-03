//! Blocking SPI API
//!
//! In some cases it's possible to implement these blocking traits on top of one of the core HAL
//! traits. To save boilerplate when that's the case a `Default` marker trait may be provided.
//! Implementing that marker trait will opt in your type into a blanket implementation.

use super::{SpiWord, U8};

/// Blocking transfer
pub trait Transfer<W: SpiWord = U8> {
    /// Error type
    type Error: crate::spi::Error;

    /// Writes and reads simultaneously. The contents of `words` are
    /// written to the slave, and the received words are stored into the same
    /// `words` buffer, overwriting it.
    fn transfer(&mut self, words: &mut [W::Data]) -> Result<(), Self::Error>;
}

impl<T: Transfer<W>, W> Transfer<W> for &mut T
where
    W: SpiWord,
{
    type Error = T::Error;

    fn transfer(&mut self, words: &mut [W::Data]) -> Result<(), Self::Error> {
        T::transfer(self, words)
    }
}

/// Blocking write
pub trait Write<W: SpiWord = U8> {
    /// Error type
    type Error: crate::spi::Error;

    /// Writes `words` to the slave, ignoring all the incoming words
    fn write(&mut self, words: &[W::Data]) -> Result<(), Self::Error>;
}

impl<T: Write<W>, W> Write<W> for &mut T
where
    W: SpiWord,
{
    type Error = T::Error;

    fn write(&mut self, words: &[W::Data]) -> Result<(), Self::Error> {
        T::write(self, words)
    }
}

/// Blocking write (iterator version)
pub trait WriteIter<W: SpiWord = U8> {
    /// Error type
    type Error: crate::spi::Error;

    /// Writes `words` to the slave, ignoring all the incoming words
    fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
    where
        WI: IntoIterator<Item = W::Data>;
}

impl<T: WriteIter<W>, W> WriteIter<W> for &mut T
where
    W: SpiWord,
{
    type Error = T::Error;

    fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
    where
        WI: IntoIterator<Item = W::Data>,
    {
        T::write_iter(self, words)
    }
}

/// Operation for transactional SPI trait
///
/// This allows composition of SPI operations into a single bus transaction
#[derive(Debug, PartialEq)]
pub enum Operation<'a, W = U8>
where
    W: SpiWord,
    W::Data: 'static,
{
    /// Write data from the provided buffer, discarding read data
    Write(&'a [W::Data]),
    /// Write data out while reading data into the provided buffer
    Transfer(&'a mut [W::Data]),
}

/// Transactional trait allows multiple actions to be executed
/// as part of a single SPI transaction
pub trait Transactional<W = U8>
where
    W: SpiWord,
    W::Data: 'static,
{
    /// Associated error type
    type Error: crate::spi::Error;

    /// Execute the provided transactions
    fn exec<'a>(&mut self, operations: &mut [Operation<'a, W>]) -> Result<(), Self::Error>;
}

impl<T: Transactional<W>, W> Transactional<W> for &mut T
where
    W: SpiWord,
    W::Data: 'static,
{
    type Error = T::Error;

    fn exec<'a>(&mut self, operations: &mut [Operation<'a, W>]) -> Result<(), Self::Error> {
        T::exec(self, operations)
    }
}
