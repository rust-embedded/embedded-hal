//! Random Number Generator Interface

use core::future::Future;

/// Nonblocking stream of random bytes.
pub trait Read {
    /// An enumeration of RNG errors.
    ///
    /// For infallible implementations, will be `Infallible`
    type Error;

    /// The future associated with the `read` method.
    type ReadFuture<'a>: Future<Output=Result<usize, Self::Error>> + 'a
    where
        Self: 'a;

    /// Get a number of bytes from the RNG.
    fn read<'a>(&'a mut self, buf: &'a mut [u8]) -> Self::ReadFuture<'a>;
}
