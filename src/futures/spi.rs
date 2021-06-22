//! Serial Peripheral Interface

use core::{future::Future, mem::MaybeUninit};

/// Async read + write
pub trait ReadWrite<Word: 'static> {
    /// Error type
    type Error;

    /// Associated future for the `transfer` method.
    type ReadWriteFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Writes `words` to the slave from the `write` buffer. Puts the words returned in the `read` buffer.
    /// This method uses separate `write` and `read` buffers.
    fn readwrite<'a>(&'a mut self, write: &'a [Word], read: &'a mut [MaybeUninit<Word>]) -> Self::ReadWriteFuture<'a>;
}

/// Async read + write in place.
pub trait ReadWriteInPlace<Word: 'static> {
    /// Error type
    type Error;

    /// Associated future for the `transfer` method.
    type ReadWriteInPlaceFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Writes `words` to the slave from the `readwrite` buffer and reads words into the same buffer.
    /// This method uses a single `readwrite` buffer.
    fn readwrite_inplace<'a>(&'a mut self, readwrite: &'a mut [MaybeUninit<Word>]) -> Self::ReadWriteInPlaceFuture<'a>;
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

/// Async read
pub trait Read<W> {
    /// Error type
    type Error;

    /// Associated future for the `read` method.
    type ReadFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Reads words from the slave without specifying any data to write.
    /// The SPI hardware will send data, though what data it sends is not defined
    /// by this trait. Some hardware can configure what values (e.g. all zeroes, all ones), some cannot.
    fn read<'a>(&'a mut self, words: &'a mut [MaybeUninit<W>]) -> Self::ReadFuture<'a>;
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
