//! Serial interface

use nb;

/// Read half of a serial interface
///
/// Some serial interfaces support different data sizes (8 bits, 9 bits, etc.);
/// This can be encoded in this trait via the `Word` type parameter.
pub trait Read<Word> {
    /// Read error
    type Error;

    /// Reads a single word from the serial interface
    fn read(&mut self) -> nb::Result<Word, Self::Error>;
}

/// Write half of a serial interface
pub trait Write<Word> {
    /// Write error
    type Error;

    /// Writes a single word to the serial interface
    fn write(&mut self, word: Word) -> nb::Result<(), Self::Error>;

    /// Ensures that none of the previously written words are still buffered
    fn flush(&mut self) -> nb::Result<(), Self::Error>;
}

#[cfg(feature = "unproven")]
/// TODO
pub mod io {
    use nb;
    use super::{Read, Write};

    /// TODO
    pub struct Reader<R> where R: Read<u8> {
        reader: R,
    }

    /// TODO
    pub struct Writer<W> where W: Write<u8> {
        writer: W,
    }

    /// TODO
    pub fn reader<R>(reader: R) -> Reader<R> where R: Read<u8> {
        Reader { reader }
    }

    /// TODO
    pub fn writer<W>(writer: W) -> Writer<W> where W: Write<u8> {
        Writer { writer }
    }

    impl<R> nb::io::Read for Reader<R> where R: Read<u8> {
        type Error = R::Error;

        fn read(&mut self, buf: &mut [u8]) -> nb::Result<usize, Self::Error> {
            let mut count = 0;
            while count < buf.len() {
                match self.reader.read() {
                    Ok(byte) => {
                        buf[count] = byte;
                        count += 1;
                    }
                    Err(nb::Error::WouldBlock) => {
                        if count > 0 {
                            return Ok(count);
                        } else {
                            return Err(nb::Error::WouldBlock);
                        }
                    }
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
            return Ok(count);
        }
    }

    impl<W> nb::io::Write for Writer<W> where W: Write<u8> {
        type Error = W::Error;

        fn write(&mut self, buf: &[u8]) -> nb::Result<usize, Self::Error> {
            let mut count = 0;
            while count < buf.len() {
                match self.writer.write(buf[count]) {
                    Ok(()) => {
                        count += 1;
                    }
                    Err(nb::Error::WouldBlock) => {
                        if count > 0 {
                            return Ok(count);
                        } else {
                            return Err(nb::Error::WouldBlock);
                        }
                    }
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
            return Ok(count);
        }

        fn flush(&mut self) -> nb::Result<(), Self::Error> {
            self.writer.flush()
        }
    }
}
