//! Serial interface

pub use embedded_hal::serial::{Error, ErrorKind, ErrorType};

/// Read half of a serial interface
///
/// Some serial interfaces support different data sizes (8 bits, 9 bits, etc.);
/// This can be encoded in this trait via the `Word` type parameter.
pub trait Read<Word: Copy = u8>: ErrorType {
    /// Reads a single word from the serial interface
    fn read(&mut self) -> nb::Result<Word, Self::Error>;
}

impl<T: Read<Word>, Word: Copy> Read<Word> for &mut T {
    fn read(&mut self) -> nb::Result<Word, Self::Error> {
        T::read(self)
    }
}

/// Write half of a serial interface
pub trait Write<Word: Copy = u8>: ErrorType {
    /// Writes a single word to the serial interface
    fn write(&mut self, word: Word) -> nb::Result<(), Self::Error>;

    /// Ensures that none of the previously written words are still buffered
    fn flush(&mut self) -> nb::Result<(), Self::Error>;
}

impl<T: Write<Word>, Word: Copy> Write<Word> for &mut T {
    fn write(&mut self, word: Word) -> nb::Result<(), Self::Error> {
        T::write(self, word)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        T::flush(self)
    }
}

/// Implementation of `core::fmt::Write` for the HAL's `serial::Write`.
///
/// TODO write example of usage

impl<Word, Error: embedded_hal::serial::Error> core::fmt::Write
    for dyn Write<Word, Error = Error> + '_
where
    Word: Copy + From<u8>,
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let _ = s
            .bytes()
            .map(|c| nb::block!(self.write(Word::from(c))))
            .last();
        Ok(())
    }
}
