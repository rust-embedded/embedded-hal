//! Serial Peripheral Interface

use core::future::Future;

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
