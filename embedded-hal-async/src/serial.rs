//! Serial interface

pub use embedded_hal::serial::{Error, ErrorKind, ErrorType};

/// Write half of a serial interface
pub trait Write<Word: 'static + Copy = u8>: ErrorType {
    /// Writes a slice, blocking until everything has been written.
    ///
    /// An implementation can choose to buffer the write, returning `Ok(())`
    /// after the complete slice has been written to a buffer, but before all
    /// words have been sent via the serial interface. To make sure that
    /// everything has been sent, call [`flush`](Write::flush) after this function returns.
    async fn write(&mut self, words: &[Word]) -> Result<(), Self::Error>;

    /// Ensures that none of the previously written data is still buffered
    async fn flush(&mut self) -> Result<(), Self::Error>;
}

impl<T: Write<Word>, Word: 'static + Copy> Write<Word> for &mut T {
    async fn write(&mut self, words: &[Word]) -> Result<(), Self::Error> {
        T::write(self, words).await
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        T::flush(self).await
    }
}
