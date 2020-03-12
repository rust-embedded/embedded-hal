//! Random Number Generator Interface

use nb;

/// Nonblocking stream of random bytes.
pub trait Read {
    /// An enumeration of RNG errors.
    ///
    /// For infallible implementations, will be `Infallible`
    type Error;

    /// Get a number of bytes from the RNG.
    fn try_read(&mut self, buf: &mut [u8]) -> nb::Result<usize, Self::Error>;
}
