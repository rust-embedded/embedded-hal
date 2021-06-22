//! Serial Peripheral Interface

use core::future::Future;

/// Full duplex (master mode)
///
/// # Notes
///
/// - It's the task of the user of this interface to manage the slave select lines
///
/// - Due to how full duplex SPI works each `read` call must be preceded by a `write` call.
///
/// - `read` calls only return the data received with the last `write` call.
/// Previously received data is discarded
///
/// - Data is only guaranteed to be clocked out when the `read` call succeeds.
/// The slave select line shouldn't be released before that.
///
/// - Some SPIs can work with 8-bit *and* 16-bit words. You can overload this trait with different
/// `Word` types to allow operation in both modes.
pub trait FullDuplex<Word> {
    /// An enumeration of SPI errors
    type Error;

    /// The future associated with the `read` method.
    type ReadFuture<'a>: Future<Output=Result<Word, Self::Error>> + 'a
    where
        Self: 'a;

    /// The future associated with the `write` method.
    type WriteFuture<'a>: Future<Output=Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Reads the word stored in the shift register
    ///
    /// **NOTE** A word must be sent to the slave before attempting to call this
    /// method.
    fn read<'a>(&'a mut self) -> Self::ReadFuture<'a>;

    /// Writes a word to the slave
    fn write<'a>(&'a mut self, word: Word) -> Self::WriteFuture<'a>;
}

/// Async transfer
pub trait Transfer<Word: 'static> {
    /// Error type
    type Error;

    /// Associated future for the `transfer` method.
    type TransferFuture<'a>: Future<Output = Result<&'a [Word], Self::Error>> + 'a
    where
        Self: 'a;

    /// Writes `words` to the slave. Returns the `words` received from the slave
    fn transfer<'a>(&'a mut self, words: &'a mut [Word]) -> Self::TransferFuture<'a>;
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

/// Async transfer
pub mod transfer {
    use core::future::Future;

    /// Default implementation of `futures::spi::Transfer<W>` for implementers of
    /// `futures::spi::FullDuplex<W>`
    pub trait Default<Word>: super::FullDuplex<Word> {}

    impl<Word, S> super::Transfer<Word> for S
    where
        S: Default<Word>,
        Word: Clone + 'static,
    {
        type Error = S::Error;

        type TransferFuture<'a> where Self: 'a = impl Future<Output = Result<&'a [Word], S::Error>> + 'a;

        fn transfer<'w>(&'w mut self, words: &'w mut [Word]) -> Self::TransferFuture<'w> {
            async move {
                for word in words.iter_mut() {
                    self.write(word.clone()).await?;
                    *word = self.read().await?;
                }

                Ok(words as &'w [Word])
            }
        }
    }
}

/// Blocking write
pub mod write {
    use core::future::Future;

    /// Default implementation of `futures::spi::Write<W>` for implementers
    /// of `futures::spi::FullDuplex<W>`
    pub trait Default<W>: super::FullDuplex<W> {}

    impl<W, S> super::Write<W> for S
    where
        S: Default<W>,
        W: Clone + 'static,
    {
        type Error = S::Error;

        type WriteFuture<'a> where Self: 'a = impl Future<Output = Result<(), S::Error>> + 'a;

        fn write<'a>(&'a mut self, words: &'a [W]) -> Self::WriteFuture<'a> {
            async move {
                for word in words {
                    self.write(word.clone()).await?;
                    self.read().await?;
                }

                Ok(())
            }
        }
    }
}

// /// Operation for transactional SPI trait
// ///
// /// This allows composition of SPI operations into a single bus transaction
// #[derive(Debug, PartialEq)]
// pub enum Operation<'a, W: 'static> {
//     /// Write data from the provided buffer, discarding read data
//     Write(&'a [W]),
//     /// Write data out while reading data into the provided buffer
//     Transfer(&'a mut [W]),
// }

// /// Transactional trait allows multiple actions to be executed
// /// as part of a single SPI transaction
// pub trait Transactional<W: 'static> {
//     /// Associated error type
//     type Error;
//     /// Future associated with the `exec` method.
//     type ExecFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
//     where
//         Self: 'a;

//     /// Execute the provided transactions
//     fn exec<'a>(&'a mut self, operations: &'a mut [Operation<'a, W>]) -> Self::ExecFuture<'a>;
// }

// /// Blocking transactional impl over spi::Write and spi::Transfer
// pub mod transactional {
//     use core::future::Future;

//     use super::{Operation, Transfer, Write};

//     /// Default implementation of `blocking::spi::Transactional<W>` for implementers of
//     /// `spi::Write<W>` and `spi::Transfer<W>`
//     pub trait Default<W: 'static>: Write<W> + Transfer<W> {}

//     impl<W: 'static, E, S> super::Transactional<W> for S
//     where
//         S: self::Default<W> + Write<W, Error = E> + Transfer<W, Error = E>,
//         W: Copy + Clone,
//     {
//         type Error = E;
//         type ExecFuture<'a> where Self: 'a = impl Future<Output=Result<(), E>> + 'a;

//         fn exec<'a>(&'a mut self, operations: &'a mut [super::Operation<'a, W>]) -> Self::ExecFuture<'a> {
//             async move {
//                 for op in operations {
//                     match op {
//                         Operation::Write(w) => self.write(w).await?,
//                         Operation::Transfer(t) => self.transfer(t).await.map(|_| ())?,
//                     }
//                 }

//                 Ok(())
//             }
//         }
//     }
// }
