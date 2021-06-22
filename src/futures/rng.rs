//! Random Number Generator Interface

use core::{future::Future, mem::MaybeUninit};

/// Nonblocking stream of random bytes.
pub trait Read {
    /// An enumeration of RNG errors.
    ///
    /// For infallible implementations, will be `Infallible`
    type Error;

    /// The future associated with the `read` method.
    type ReadFuture<'a>: Future<Output=Result<&'a [u8], Self::Error>> + 'a
    where
        Self: 'a;

    /// Get a number of bytes from the RNG. The returned buffer is the initialized `buf`.
    fn read<'a>(&'a mut self, buf: &'a mut [MaybeUninit<u8>]) -> Self::ReadFuture<'a>;
}
