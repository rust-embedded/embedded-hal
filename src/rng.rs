//! Random Number Generator Interface

#[cfg(feature = "unproven")]
use core::task::Poll;

/// Nonblocking stream of random bytes.
#[cfg(feature = "unproven")]
// reason: No implementation or users yet
pub trait Read {
    /// An enumeration of RNG errors.
    ///
    /// For infallible implementations, will be `Infallible`
    type Error;

    /// Get a number of bytes from the RNG.
    fn read(&mut self, buf: &mut [u8]) -> Poll<Result<usize, Self::Error>>;
}
