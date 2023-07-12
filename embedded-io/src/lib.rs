#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use core::fmt;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
pub mod adapters;

mod impls;

/// Enumeration of possible methods to seek within an I/O object.
///
/// Semantics are the same as [`std::io::SeekFrom`], check its documentation for details.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SeekFrom {
    /// Sets the offset to the provided number of bytes.
    Start(u64),
    /// Sets the offset to the size of this object plus the specified number of bytes.
    End(i64),
    /// Sets the offset to the current position plus the specified number of bytes.
    Current(i64),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
/// Possible kinds of errors.
pub enum ErrorKind {
    /// Unspecified error kind.
    Other,
}

/// Error trait.
///
/// This trait allows generic code to do limited inspecting of errors,
/// to react differently to different kinds.
pub trait Error: core::fmt::Debug {
    /// Get the kind of this error.
    fn kind(&self) -> ErrorKind;
}

impl Error for core::convert::Infallible {
    fn kind(&self) -> ErrorKind {
        match *self {}
    }
}

impl Error for ErrorKind {
    fn kind(&self) -> ErrorKind {
        *self
    }
}

/// Base trait for all IO traits, defining the error type.
///
/// All IO operations of all traits return the error defined in this trait.
///
/// Having a shared trait instead of having every trait define its own
/// `Error` associated type enforces all impls on the same type use the same error.
/// This is very convenient when writing generic code, it means you have to
/// handle a single error type `T::Error`, instead of `<T as Read>::Error` and `<T as Write>::Error`
/// which might be different types.
pub trait ErrorType {
    /// Error type of all the IO operations on this type.
    type Error: Error;
}

impl<T: ?Sized + crate::ErrorType> crate::ErrorType for &mut T {
    type Error = T::Error;
}

/// Error returned by [`Read::read_exact`]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ReadExactError<E> {
    /// An EOF error was encountered before reading the exact amount of requested bytes.
    UnexpectedEof,
    /// Error returned by the inner Read.
    Other(E),
}

impl<E> From<E> for ReadExactError<E> {
    fn from(err: E) -> Self {
        Self::Other(err)
    }
}

impl<E: fmt::Debug> fmt::Display for ReadExactError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl<E: fmt::Debug> std::error::Error for ReadExactError<E> {}

/// Error returned by [`Write::write_fmt`]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WriteFmtError<E> {
    /// [`Write::write`] wrote zero bytes
    WriteZero,
    /// An error was encountered while formatting.
    FmtError,
    /// Error returned by the inner Write.
    Other(E),
}

impl<E> From<E> for WriteFmtError<E> {
    fn from(err: E) -> Self {
        Self::Other(err)
    }
}

impl<E: fmt::Debug> fmt::Display for WriteFmtError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl<E: fmt::Debug> std::error::Error for WriteFmtError<E> {}

/// Error returned by [`Write::write_all`]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WriteAllError<E> {
    /// [`Write::write`] wrote zero bytes
    WriteZero,
    /// Error returned by the inner Write.
    Other(E),
}

impl<E> From<E> for WriteAllError<E> {
    fn from(err: E) -> Self {
        Self::Other(err)
    }
}

impl<E: fmt::Debug> fmt::Display for WriteAllError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl<E: fmt::Debug> std::error::Error for WriteAllError<E> {}

/// Blocking reader.
///
/// Semantics are the same as [`std::io::Read`], check its documentation for details.
pub trait Read: crate::ErrorType {
    /// Pull some bytes from this source into the specified buffer, returning how many bytes were read.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;

    /// Read the exact number of bytes required to fill `buf`.
    fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<(), ReadExactError<Self::Error>> {
        while !buf.is_empty() {
            match self.read(buf) {
                Ok(0) => break,
                Ok(n) => buf = &mut buf[n..],
                Err(e) => return Err(ReadExactError::Other(e)),
            }
        }
        if !buf.is_empty() {
            Err(ReadExactError::UnexpectedEof)
        } else {
            Ok(())
        }
    }
}

/// Blocking buffered reader.
///
/// Semantics are the same as [`std::io::BufRead`], check its documentation for details.
pub trait BufRead: crate::ErrorType {
    /// Return the contents of the internal buffer, filling it with more data from the inner reader if it is empty.
    fn fill_buf(&mut self) -> Result<&[u8], Self::Error>;

    /// Tell this buffer that `amt` bytes have been consumed from the buffer, so they should no longer be returned in calls to `fill_buf`.
    fn consume(&mut self, amt: usize);
}

/// Blocking writer.
///
/// Semantics are the same as [`std::io::Write`], check its documentation for details.
pub trait Write: crate::ErrorType {
    /// Write a buffer into this writer, returning how many bytes were written.
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error>;

    /// Flush this output stream, ensuring that all intermediately buffered contents reach their destination.
    fn flush(&mut self) -> Result<(), Self::Error>;

    /// Write an entire buffer into this writer.
    fn write_all(&mut self, mut buf: &[u8]) -> Result<(), WriteAllError<Self::Error>> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => return Err(WriteAllError::WriteZero),
                Ok(n) => buf = &buf[n..],
                Err(e) => return Err(WriteAllError::Other(e)),
            }
        }
        Ok(())
    }

    /// Write a formatted string into this writer, returning any error encountered.
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> Result<(), WriteFmtError<Self::Error>> {
        // Create a shim which translates a Write to a fmt::Write and saves
        // off I/O errors. instead of discarding them
        struct Adapter<'a, T: Write + ?Sized + 'a> {
            inner: &'a mut T,
            error: Result<(), WriteAllError<T::Error>>,
        }

        impl<T: Write + ?Sized> fmt::Write for Adapter<'_, T> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                match self.inner.write_all(s.as_bytes()) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.error = Err(e);
                        Err(fmt::Error)
                    }
                }
            }
        }

        let mut output = Adapter {
            inner: self,
            error: Ok(()),
        };
        match fmt::write(&mut output, fmt) {
            Ok(()) => Ok(()),
            Err(..) => match output.error {
                // check if the error came from the underlying `Write` or not
                Err(e) => match e {
                    WriteAllError::WriteZero => Err(WriteFmtError::WriteZero),
                    WriteAllError::Other(e) => Err(WriteFmtError::Other(e)),
                },
                Ok(()) => Err(WriteFmtError::FmtError),
            },
        }
    }
}

/// Blocking seek within streams.
///
/// Semantics are the same as [`std::io::Seek`], check its documentation for details.
pub trait Seek: crate::ErrorType {
    /// Seek to an offset, in bytes, in a stream.
    fn seek(&mut self, pos: crate::SeekFrom) -> Result<u64, Self::Error>;

    /// Rewind to the beginning of a stream.
    fn rewind(&mut self) -> Result<(), Self::Error> {
        self.seek(crate::SeekFrom::Start(0))?;
        Ok(())
    }

    /// Returns the current seek position from the start of the stream.
    fn stream_position(&mut self) -> Result<u64, Self::Error> {
        self.seek(crate::SeekFrom::Current(0))
    }
}

/// Get whether a reader is ready.
///
/// This allows using a [`Read`] or [`BufRead`] in a nonblocking fashion, i.e. trying to read
/// only when it is ready.
pub trait ReadReady: crate::ErrorType {
    /// Get whether the reader is ready for immediately reading.
    ///
    /// This usually means that there is either some bytes have been received and are buffered and ready to be read,
    /// or that the reader is at EOF.
    ///
    /// If this returns `true`, it's guaranteed that the next call to [`Read::read`] or [`BufRead::fill_buf`] will not block.
    fn read_ready(&mut self) -> Result<bool, Self::Error>;
}

/// Get whether a writer is ready.
///
/// This allows using a [`Write`] in a nonblocking fashion, i.e. trying to write
/// only when it is ready.
pub trait WriteReady: crate::ErrorType {
    /// Get whether the writer is ready for immediately writing.
    ///
    /// This usually means that there is free space in the internal transmit buffer.
    ///
    /// If this returns `true`, it's guaranteed that the next call to [`Write::write`] will not block.
    fn write_ready(&mut self) -> Result<bool, Self::Error>;
}

impl<T: ?Sized + Read> Read for &mut T {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        T::read(self, buf)
    }
}

impl<T: ?Sized + BufRead> BufRead for &mut T {
    fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
        T::fill_buf(self)
    }

    fn consume(&mut self, amt: usize) {
        T::consume(self, amt)
    }
}

impl<T: ?Sized + Write> Write for &mut T {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        T::write(self, buf)
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        T::flush(self)
    }
}

impl<T: ?Sized + Seek> Seek for &mut T {
    #[inline]
    fn seek(&mut self, pos: crate::SeekFrom) -> Result<u64, Self::Error> {
        T::seek(self, pos)
    }
}

impl<T: ?Sized + ReadReady> ReadReady for &mut T {
    #[inline]
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        T::read_ready(self)
    }
}

impl<T: ?Sized + WriteReady> WriteReady for &mut T {
    #[inline]
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        T::write_ready(self)
    }
}
