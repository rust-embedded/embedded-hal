#![feature(async_fn_in_trait, impl_trait_projections)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "alloc")]
extern crate alloc;

mod impls;

pub use embedded_io::{
    Error, ErrorKind, Io, ReadExactError, ReadReady, SeekFrom, WriteAllError, WriteReady,
};

/// Async reader.
///
/// Semantics are the same as [`std::io::Read`], check its documentation for details.
pub trait Read: Io {
    /// Pull some bytes from this source into the specified buffer, returning how many bytes were read.
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;

    /// Read the exact number of bytes required to fill `buf`.
    async fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<(), ReadExactError<Self::Error>> {
        while !buf.is_empty() {
            match self.read(buf).await {
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

/// Async buffered reader.
///
/// Semantics are the same as [`std::io::BufRead`], check its documentation for details.
pub trait BufRead: Io {
    /// Return the contents of the internal buffer, filling it with more data from the inner reader if it is empty.
    async fn fill_buf(&mut self) -> Result<&[u8], Self::Error>;

    /// Tell this buffer that `amt` bytes have been consumed from the buffer, so they should no longer be returned in calls to `fill_buf`.
    fn consume(&mut self, amt: usize);
}

/// Async writer.
///
/// Semantics are the same as [`std::io::Write`], check its documentation for details.
pub trait Write: Io {
    /// Write a buffer into this writer, returning how many bytes were written.
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error>;

    /// Flush this output stream, ensuring that all intermediately buffered contents reach their destination.
    async fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Write an entire buffer into this writer.
    async fn write_all(&mut self, buf: &[u8]) -> Result<(), WriteAllError<Self::Error>> {
        let mut buf = buf;
        while !buf.is_empty() {
            match self.write(buf).await {
                Ok(0) => return Err(WriteAllError::WriteZero),
                Ok(n) => buf = &buf[n..],
                Err(e) => return Err(WriteAllError::Other(e)),
            }
        }
        Ok(())
    }
}

/// Async seek within streams.
///
/// Semantics are the same as [`std::io::Seek`], check its documentation for details.
pub trait Seek: Io {
    /// Seek to an offset, in bytes, in a stream.
    async fn seek(&mut self, pos: SeekFrom) -> Result<u64, Self::Error>;

    /// Rewind to the beginning of a stream.
    async fn rewind(&mut self) -> Result<(), Self::Error> {
        self.seek(SeekFrom::Start(0)).await?;
        Ok(())
    }

    /// Returns the current seek position from the start of the stream.
    async fn stream_position(&mut self) -> Result<u64, Self::Error> {
        self.seek(SeekFrom::Current(0)).await
    }
}

impl<T: ?Sized + Read> Read for &mut T {
    #[inline]
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        T::read(self, buf).await
    }
}

impl<T: ?Sized + BufRead> BufRead for &mut T {
    async fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
        T::fill_buf(self).await
    }

    fn consume(&mut self, amt: usize) {
        T::consume(self, amt)
    }
}

impl<T: ?Sized + Write> Write for &mut T {
    #[inline]
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        T::write(self, buf).await
    }

    #[inline]
    async fn flush(&mut self) -> Result<(), Self::Error> {
        T::flush(self).await
    }
}

impl<T: ?Sized + Seek> Seek for &mut T {
    #[inline]
    async fn seek(&mut self, pos: SeekFrom) -> Result<u64, Self::Error> {
        T::seek(self, pos).await
    }
}
