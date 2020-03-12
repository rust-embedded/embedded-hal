//! Blocking hardware random number generator

/// Blocking read
pub trait Read {
    /// Error type
    type Error;

    /// Reads enough bytes from hardware random number generator to fill `buffer`
    ///
    /// If any error is encountered then this function immediately returns. The contents of buf are
    /// unspecified in this case.
    ///
    /// If this function returns an error, it is unspecified how many bytes it has read, but it
    /// will never read more than would be necessary to completely fill the buffer.
    fn try_read(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error>;
}
