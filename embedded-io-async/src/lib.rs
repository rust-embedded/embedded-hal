#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
#![allow(async_fn_in_trait)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod impls;

pub use embedded_io::{
    Error, ErrorKind, ErrorType, ReadExactError, ReadReady, SeekFrom, WriteReady,
};

/// Async reader.
///
/// This trait is the `embedded-io-async` equivalent of [`std::io::Read`].
pub trait Read: ErrorType {
    /// Read some bytes from this source into the specified buffer, returning how many bytes were read.
    ///
    /// If no bytes are currently available to read:
    /// - The method waits until at least one byte becomes available;
    /// - Once at least one (or more) bytes become available, a non-zero amount of those is copied to the
    ///   beginning of `buf`, and the amount is returned, *without waiting any further for more bytes to
    ///   become available*.
    ///
    /// If bytes are available to read:
    /// - A non-zero amount of bytes is read to the beginning of `buf`, and the amount is returned immediately,
    ///   *without waiting for more bytes to become available*;
    ///
    /// Note that once some bytes are available to read, it is *not* guaranteed that all available bytes are returned.
    /// It is possible for the implementation to read an amount of bytes less than `buf.len()` while there are more
    /// bytes immediately available.
    ///
    /// This waiting behavior is important for the cases where `Read` represents the "read" leg of a pipe-like
    /// protocol (a socket, a pipe, a serial line etc.). The semantics is that the caller - by passing a non-empty
    /// buffer - does expect _some_ data (one or more bytes) - but _not necessarily `buf.len()` or more bytes_ -
    /// to become available, before the peer represented by `Read` would stop sending bytes due to
    /// application-specific reasons (as in the peer waiting for a response to the data it had sent so far).
    ///
    /// If the reader is at end-of-file (EOF), `Ok(0)` is returned. There is no guarantee that a reader at EOF
    /// will always be so in the future, for example a reader can stop being at EOF if another process appends
    /// more bytes to the underlying file.
    ///
    /// If `buf.len() == 0`, `read` returns without waiting, with either `Ok(0)` or an error.
    /// The `Ok(0)` doesn't indicate EOF, unlike when called with a non-empty buffer.
    ///
    /// # Cancel Safety
    ///
    /// **This method is NOT guaranteed to be cancel-safe.**
    ///
    /// If a `read()` future is dropped before it completes, bytes that have already been transferred
    /// from the hardware FIFO or DMA buffer may be silently discarded — no error is returned, and
    /// no indication is given to the caller that data was lost. The next call to `read()` will return
    /// bytes received *after* the cancellation point, not the discarded bytes.
    ///
    /// Implementations that are able to guarantee cancel safety (i.e. that dropping a pending future
    /// leaves the stream state unchanged — no bytes consumed) SHOULD document this explicitly and
    /// MAY implement the `CancelSafeRead` marker trait when it becomes available.
    ///
    /// Implementations that cannot guarantee cancel safety (e.g. those that use DMA to write
    /// directly into the caller's buffer) MUST document this limitation.
    ///
    /// Callers that require cancel-safe reads SHOULD use a pattern that ensures the future runs to
    /// completion (e.g. a dedicated read task) rather than relying on cancel safety.
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;

    /// Read the exact number of bytes required to fill `buf`.
    ///
    /// This function calls `read()` in a loop until exactly `buf.len()` bytes have
    /// been read, waiting if needed.
    ///
    /// This function is not side-effect-free on cancel (AKA "cancel-safe"), i.e. if you cancel (drop) a returned
    /// future that hasn't completed yet, some bytes might have already been read, which will get lost.
    async fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<(), ReadExactError<Self::Error>> {
        while !buf.is_empty() {
            match self.read(buf).await {
                Ok(0) => break,
                Ok(n) => buf = &mut buf[n..],
                Err(e) => return Err(ReadExactError::Other(e)),
            }
        }
        if buf.is_empty() {
            Ok(())
        } else {
            Err(ReadExactError::UnexpectedEof)
        }
    }
}

/// Async buffered reader.
///
/// This trait is the `embedded-io-async` equivalent of [`std::io::BufRead`].
pub trait BufRead: Read {
    /// Return the contents of the internal buffer, filling it with more data from the inner reader if it is empty.
    ///
    /// If no bytes are currently available to read, this function waits until at least one byte is available.
    ///
    /// If the reader is at end-of-file (EOF), an empty slice is returned. There is no guarantee that a reader at EOF
    /// will always be so in the future, for example a reader can stop being at EOF if another process appends
    /// more bytes to the underlying file.
    async fn fill_buf(&mut self) -> Result<&[u8], Self::Error>;

    /// Tell this buffer that `amt` bytes have been consumed from the buffer, so they should no longer be returned in calls to `fill_buf`.
    fn consume(&mut self, amt: usize);
}

/// Async writer.
///
/// This trait is the `embedded-io-async` equivalent of [`std::io::Write`].
pub trait Write: ErrorType {
    /// Write a buffer into this writer, returning how many bytes were written.
    ///
    /// If the writer is not currently ready to accept more bytes (for example, its buffer is full),
    /// this function waits until it is ready to accept least one byte.
    ///
    /// If it's ready to accept bytes, a non-zero amount of bytes is written from the beginning of `buf`, and the amount
    /// is returned. It is not guaranteed that *all* available buffer space is filled, i.e. it is possible for the
    /// implementation to write an amount of bytes less than `buf.len()` while the writer continues to be
    /// ready to accept more bytes immediately.
    ///
    /// Implementations should never return `Ok(0)` when `buf.len() != 0`. Situations where the writer is not
    /// able to accept more bytes and likely never will are better indicated with errors.
    ///
    /// If `buf.len() == 0`, `write` returns without waiting, with either `Ok(0)` or an error.
    /// The `Ok(0)` doesn't indicate an error.
    ///
    /// # Cancel Safety
    ///
    /// **This method is NOT guaranteed to be cancel-safe.**
    ///
    /// If a `write()` future is dropped before it completes, bytes may have already been submitted
    /// to the hardware without the caller's knowledge — no error is returned. The number of bytes
    /// actually written is unknown.
    ///
    /// Implementations that are able to guarantee cancel safety (i.e. that dropping a pending future
    /// leaves the stream state unchanged — no bytes written) SHOULD document this explicitly.
    ///
    /// Implementations that cannot guarantee cancel safety (e.g. those that use DMA directly from
    /// the caller's buffer) MUST document this limitation.
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error>;

    /// Flush this output stream, ensuring that all intermediately buffered contents reach their destination.
    async fn flush(&mut self) -> Result<(), Self::Error>;

    /// Write an entire buffer into this writer.
    ///
    /// This function calls `write()` in a loop until exactly `buf.len()` bytes have
    /// been written, waiting if needed.
    ///
    /// This function is not side-effect-free on cancel (AKA "cancel-safe"), i.e. if you cancel (drop) a returned
    /// future that hasn't completed yet, some bytes might have already been written.
    async fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        let mut buf = buf;
        while !buf.is_empty() {
            match self.write(buf).await {
                Ok(0) => panic!("write() returned Ok(0)"),
                Ok(n) => buf = &buf[n..],
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

/// Async seek within streams.
///
/// This trait is the `embedded-io-async` equivalent of [`std::io::Seek`].
pub trait Seek: ErrorType {
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
