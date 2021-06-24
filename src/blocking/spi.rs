//! Blocking SPI API

/// Blocking transfer with separate buffers
pub trait Transfer<W> {
    /// Error type
    type Error;

    /// Writes and reads simultaneously. `write` is written to the slave on MOSI and
    /// words received on MISO are stored in `read`.
    ///
    /// It is allowed for `read` and `write` to have different lengths, even zero length.
    /// The transfer runs for `max(read.len(), write.len())` words. If `read` is shorter,
    /// incoming words after `read` has been filled will be discarded. If `write` is shorter,
    /// the value of words sent in MOSI after all `write` has been sent is implementation defined,
    /// typically `0x00`, `0xFF`, or configurable.
    fn transfer(&mut self, read: &mut [W], write: &[W]) -> Result<(), Self::Error>;
}

/// Blocking transfer with single buffer
pub trait TransferInplace<W> {
    /// Error type
    type Error;

    /// Writes and reads simultaneously. The contents of `words` are
    /// written to the slave, and the received words are stored into the same
    /// `words` buffer, overwriting it.
    fn transfer_inplace(&mut self, words: &mut [W]) -> Result<(), Self::Error>;
}

/// Blocking write
pub trait Write<W> {
    /// Error type
    type Error;

    /// Writes `words` to the slave, ignoring all the incoming words
    fn write(&mut self, words: &[W]) -> Result<(), Self::Error>;
}

/// Blocking read
pub trait Read<W> {
    /// Error type
    type Error;

    /// Reads `words` to the slave. The word value sent on MOSI during
    /// reading is implementation defined, typically `0x00`, `0xFF`, or configurable.
    fn read(&mut self, words: &mut [W]) -> Result<(), Self::Error>;
}

/// Blocking write (iterator version)
pub trait WriteIter<W> {
    /// Error type
    type Error;

    /// Writes `words` to the slave, ignoring all the incoming words
    fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
    where
        WI: IntoIterator<Item = W>;
}

/// Blocking transfer with separate buffers
pub mod transfer {
    /// Default implementation of `blocking::spi::Transfer<W>` for implementers of
    /// `nonblocking::spi::FullDuplex<W>`
    ///
    /// If `read` is longer than `write`, `W::default()` (which is typically 0) is sent on MOSI
    /// to fill the remaining bytes.
    pub trait Default<W>: crate::nb::spi::FullDuplex<W> {}

    impl<W, S> crate::blocking::spi::Transfer<W> for S
    where
        S: Default<W>,
        W: Clone + core::default::Default,
    {
        type Error = S::Error;

        fn transfer(&mut self, read: &mut [W], write: &[W]) -> Result<(), S::Error> {
            for i in 0..core::cmp::max(read.len(), write.len()) {
                let word_out = if i < write.len() {
                    write[i].clone()
                } else {
                    W::default()
                };
                nb::block!(self.write(word_out.clone()))?;

                let word_in = nb::block!(self.read())?;
                if i < read.len() {
                    read[i] = word_in;
                }
            }

            Ok(())
        }
    }
}

/// Blocking simultaneous read+write with separate buffers
pub mod transfer_inplace {
    /// Default implementation of `blocking::spi::Transfer<W>` for implementers of
    /// `nonblocking::spi::FullDuplex<W>`
    pub trait Default<W>: crate::nb::spi::FullDuplex<W> {}

    impl<W, S> crate::blocking::spi::TransferInplace<W> for S
    where
        S: Default<W>,
        W: Clone,
    {
        type Error = S::Error;

        fn transfer_inplace(&mut self, words: &mut [W]) -> Result<(), S::Error> {
            for word in words.iter_mut() {
                nb::block!(self.write(word.clone()))?;
                *word = nb::block!(self.read())?;
            }

            Ok(())
        }
    }
}

/// Blocking write
pub mod write {
    /// Default implementation of `blocking::spi::Write<W>` for implementers
    /// of `nonblocking::spi::FullDuplex<W>`
    pub trait Default<W>: crate::nb::spi::FullDuplex<W> {}

    impl<W, S> crate::blocking::spi::Write<W> for S
    where
        S: Default<W>,
        W: Clone,
    {
        type Error = S::Error;

        fn write(&mut self, words: &[W]) -> Result<(), S::Error> {
            for word in words {
                nb::block!(self.write(word.clone()))?;
                nb::block!(self.read())?;
            }

            Ok(())
        }
    }
}

/// Blocking read
pub mod read {
    /// Default implementation of `blocking::spi::Read<W>` for implementers
    /// of `nonblocking::spi::FullDuplex<W>`
    ///
    /// During the read, `W::default()` (which is typically 0) is sent on MOSI.
    pub trait Default<W>: crate::nb::spi::FullDuplex<W> {}

    impl<W, S> crate::blocking::spi::Read<W> for S
    where
        S: Default<W>,
        W: core::default::Default,
    {
        type Error = S::Error;

        fn read(&mut self, words: &mut [W]) -> Result<(), S::Error> {
            for word in words.iter_mut() {
                nb::block!(self.write(W::default()))?;
                *word = nb::block!(self.read())?;
            }

            Ok(())
        }
    }
}

/// Blocking write (iterator version)
pub mod write_iter {
    /// Default implementation of `blocking::spi::WriteIter<W>` for implementers of
    /// `nonblocking::spi::FullDuplex<W>`
    pub trait Default<W>: crate::nb::spi::FullDuplex<W> {}

    impl<W, S> crate::blocking::spi::WriteIter<W> for S
    where
        S: Default<W>,
        W: Clone,
    {
        type Error = S::Error;

        fn write_iter<WI>(&mut self, words: WI) -> Result<(), S::Error>
        where
            WI: IntoIterator<Item = W>,
        {
            for word in words.into_iter() {
                nb::block!(self.write(word.clone()))?;
                nb::block!(self.read())?;
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
    fn exec<'a>(&mut self, operations: &mut [Operation<'a, W>]) -> Result<(), Self::Error>;
}

/// Blocking transactional impl over spi::Write and spi::Transfer
pub mod transactional {
    use super::{Operation, TransferInplace, Write};

    /// Default implementation of `blocking::spi::Transactional<W>` for implementers of
    /// `spi::Write<W>` and `spi::Transfer<W>`
    pub trait Default<W>: Write<W> + TransferInplace<W> {}

    impl<W: 'static, E, S> super::Transactional<W> for S
    where
        S: self::Default<W> + Write<W, Error = E> + TransferInplace<W, Error = E>,
        W: Copy + Clone,
    {
        type Error = E;

        fn exec<'a>(&mut self, operations: &mut [super::Operation<'a, W>]) -> Result<(), E> {
            for op in operations {
                match op {
                    Operation::Write(w) => self.write(w)?,
                    Operation::Transfer(t) => self.transfer_inplace(t).map(|_| ())?,
                }
            }

            Ok(())
        }
    }
}
