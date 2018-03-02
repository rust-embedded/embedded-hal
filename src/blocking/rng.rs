//! Blocking hardware random number generator

/// Blocking read
#[cfg(feature = "unproven")]
pub trait Read {
    /// Error type
    type Error;

    /// Reads enough bytes from hardware random number generator to fill `buffer`
    fn read(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error>;
}
