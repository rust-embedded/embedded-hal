//! Blocking SPI API

/// Blocking transfer
pub trait Transfer<W> {
    /// Error type
    type Error;

    /// Sends `words` to the slave. Returns the `words` received from the slave
    fn try_transfer<'w>(&mut self, words: &'w mut [W]) -> Result<&'w [W], Self::Error>;
}

/// Blocking write
pub trait Write<W> {
    /// Error type
    type Error;

    /// Sends `words` to the slave, ignoring all the incoming words
    fn try_write(&mut self, words: &[W]) -> Result<(), Self::Error>;
}

/// Blocking write (iterator version)
pub trait WriteIter<W> {
    /// Error type
    type Error;

    /// Sends `words` to the slave, ignoring all the incoming words
    fn try_write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
    where
        WI: IntoIterator<Item = W>;
}

/// Blocking transfer
pub mod transfer {
    /// Default implementation of `blocking::spi::Transfer<W>` for implementers of
    /// `spi::FullDuplex<W>`
    pub trait Default<W>: crate::spi::FullDuplex<W> {}

    impl<W, S> crate::blocking::spi::Transfer<W> for S
    where
        S: Default<W>,
        W: Clone,
    {
        type Error = S::Error;

        fn try_transfer<'w>(&mut self, words: &'w mut [W]) -> Result<&'w [W], S::Error> {
            for word in words.iter_mut() {
                nb::block!(self.try_send(word.clone()))?;
                *word = nb::block!(self.try_read())?;
            }

            Ok(words)
        }
    }
}

/// Blocking write
pub mod write {
    /// Default implementation of `blocking::spi::Write<W>` for implementers of `spi::FullDuplex<W>`
    pub trait Default<W>: crate::spi::FullDuplex<W> {}

    impl<W, S> crate::blocking::spi::Write<W> for S
    where
        S: Default<W>,
        W: Clone,
    {
        type Error = S::Error;

        fn try_write(&mut self, words: &[W]) -> Result<(), S::Error> {
            for word in words {
                nb::block!(self.try_send(word.clone()))?;
                nb::block!(self.try_read())?;
            }

            Ok(())
        }
    }
}

/// Blocking write (iterator version)
pub mod write_iter {
    /// Default implementation of `blocking::spi::WriteIter<W>` for implementers of
    /// `spi::FullDuplex<W>`
    pub trait Default<W>: crate::spi::FullDuplex<W> {}

    impl<W, S> crate::blocking::spi::WriteIter<W> for S
    where
        S: Default<W>,
        W: Clone,
    {
        type Error = S::Error;

        fn try_write_iter<WI>(&mut self, words: WI) -> Result<(), S::Error>
        where
            WI: IntoIterator<Item = W>,
        {
            for word in words.into_iter() {
                nb::block!(self.try_send(word.clone()))?;
                nb::block!(self.try_read())?;
            }

            Ok(())
        }
    }
}

/// Operation for transactional SPI trait
///
/// This allows composition of SPI operations into a single bus transaction
#[derive(Debug, PartialEq)]
pub enum Operation<'a, W: 'static> {
    /// Write data from the provided buffer, discarding read data
    Write(&'a [W]),
    /// Write data out while reading data into the provided buffer
    Transfer(&'a mut [W]),
}

/// Transactional trait allows multiple actions to be executed
/// as part of a single SPI transaction
pub trait Transactional<W: 'static> {
    /// Associated error type
    type Error;

    /// Execute the provided transactions
    fn try_exec<'a>(&mut self, operations: &mut [Operation<'a, W>]) -> Result<(), Self::Error>;
}

/// Blocking transactional impl over spi::Write and spi::Transfer
pub mod transactional {
    use super::{Operation, Transfer, Write};

    /// Default implementation of `blocking::spi::Transactional<W>` for implementers of
    /// `spi::Write<W>` and `spi::Transfer<W>`
    pub trait Default<W>: Write<W> + Transfer<W> {}

    impl<W: 'static, E, S> super::Transactional<W> for S
    where
        S: self::Default<W> + Write<W, Error = E> + Transfer<W, Error = E>,
        W: Copy + Clone,
    {
        type Error = E;

        fn try_exec<'a>(&mut self, operations: &mut [super::Operation<'a, W>]) -> Result<(), E> {
            for op in operations {
                match op {
                    Operation::Write(w) => self.try_write(w)?,
                    Operation::Transfer(t) => self.try_transfer(t).map(|_| ())?,
                }
            }

            Ok(())
        }
    }
}
