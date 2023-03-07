//! Serial interface

pub use embedded_hal::serial::{Error, ErrorKind, ErrorType};

/// Read an exact amount of words from a serial interface
///
/// Some serial interfaces support different data sizes (8 bits, 9 bits, etc.);
/// This can be encoded in this trait via the `Word` type parameter.
pub trait ReadExact<Word: 'static + Copy = u8>: ErrorType {
    /// Read an exact amount of words.
    ///
    /// This does not return until exactly `read.len()` words have been read.
    async fn read_exact(&mut self, read: &mut [Word]) -> Result<(), Self::Error>;
}

impl<T: ReadExact<Word>, Word: 'static + Copy> ReadExact<Word> for &mut T {
    async fn read_exact(&mut self, read: &mut [Word]) -> Result<(), Self::Error> {
        T::read_exact(self, read).await
    }
}

/// Read words from a serial interface, until the line becomes idle.
///
/// Some serial interfaces support different data sizes (8 bits, 9 bits, etc.);
/// This can be encoded in this trait via the `Word` type parameter.
pub trait ReadUntilIdle<Word: 'static + Copy = u8>: ErrorType {
    /// Read words until the line becomes idle.
    ///
    /// Returns the amount of words received.
    ///
    /// This returns at the earliest of either:
    /// - at least 1 word has been received, and then the line becomes idle
    /// - exactly `read.len()` words have been read (the buffer is full)
    ///
    /// The serial line is considered idle after a timeout of it being constantly
    /// at high level. The exact timeout is implementation-defined, but it should be
    /// short, around 1 or 2 words' worth of time.
    async fn read_until_idle(&mut self, read: &mut [Word]) -> Result<usize, Self::Error>;
}

impl<T: ReadUntilIdle<Word>, Word: 'static + Copy> ReadUntilIdle<Word> for &mut T {
    async fn read_until_idle(&mut self, read: &mut [Word]) -> Result<usize, Self::Error> {
        T::read_until_idle(self, read).await
    }
}

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
