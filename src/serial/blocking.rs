//! Blocking serial API

/// Write half of a serial interface (blocking variant)
pub trait Write<Word: Copy = u8>: super::ErrorType {
    /// Writes a slice, blocking until everything has been written
    ///
    /// An implementation can choose to buffer the write, returning `Ok(())`
    /// after the complete slice has been written to a buffer, but before all
    /// words have been sent via the serial interface. To make sure that
    /// everything has been sent, call [`flush`] after this function returns.
    ///
    /// [`flush`]: #tymethod.flush
    fn write(&mut self, buffer: &[Word]) -> Result<(), Self::Error>;

    /// Block until the serial interface has sent all buffered words
    fn flush(&mut self) -> Result<(), Self::Error>;
}

impl<T: Write<Word>, Word: Copy> Write<Word> for &mut T {
    fn write(&mut self, buffer: &[Word]) -> Result<(), Self::Error> {
        T::write(self, buffer)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        T::flush(self)
    }
}
