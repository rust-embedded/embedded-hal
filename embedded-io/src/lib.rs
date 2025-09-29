#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use core::fmt;

#[cfg(feature = "alloc")]
extern crate alloc;

mod impls;

/// Enumeration of possible methods to seek within an I/O object.
///
/// This is the `embedded-io` equivalent of [`std::io::SeekFrom`].
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SeekFrom {
    /// Sets the offset to the provided number of bytes.
    Start(u64),
    /// Sets the offset to the size of this object plus the specified number of bytes.
    End(i64),
    /// Sets the offset to the current position plus the specified number of bytes.
    Current(i64),
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl From<SeekFrom> for std::io::SeekFrom {
    fn from(pos: SeekFrom) -> Self {
        match pos {
            SeekFrom::Start(n) => std::io::SeekFrom::Start(n),
            SeekFrom::End(n) => std::io::SeekFrom::End(n),
            SeekFrom::Current(n) => std::io::SeekFrom::Current(n),
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl From<std::io::SeekFrom> for SeekFrom {
    fn from(pos: std::io::SeekFrom) -> SeekFrom {
        match pos {
            std::io::SeekFrom::Start(n) => SeekFrom::Start(n),
            std::io::SeekFrom::End(n) => SeekFrom::End(n),
            std::io::SeekFrom::Current(n) => SeekFrom::Current(n),
        }
    }
}

/// Possible kinds of errors.
///
/// This list is intended to grow over time and it is not recommended to
/// exhaustively match against it. In application code, use `match` for the `ErrorKind`
/// values you are expecting; use `_` to match "all other errors".
///
/// This is the `embedded-io` equivalent of [`std::io::ErrorKind`], except with the following changes:
///
/// - `WouldBlock` is removed, since `embedded-io` traits are always blocking. See the [crate-level documentation](crate) for details.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum ErrorKind {
    /// Unspecified error kind.
    Other,

    /// An entity was not found, often a file.
    NotFound,
    /// The operation lacked the necessary privileges to complete.
    PermissionDenied,
    /// The connection was refused by the remote server.
    ConnectionRefused,
    /// The connection was reset by the remote server.
    ConnectionReset,
    /// The connection was aborted (terminated) by the remote server.
    ConnectionAborted,
    /// The network operation failed because it was not connected yet.
    NotConnected,
    /// A socket address could not be bound because the address is already in
    /// use elsewhere.
    AddrInUse,
    /// A nonexistent interface was requested or the requested address was not
    /// local.
    AddrNotAvailable,
    /// The operation failed because a pipe was closed.
    BrokenPipe,
    /// An entity already exists, often a file.
    AlreadyExists,
    /// A parameter was incorrect.
    InvalidInput,
    /// Data not valid for the operation were encountered.
    ///
    /// Unlike [`InvalidInput`], this typically means that the operation
    /// parameters were valid, however the error was caused by malformed
    /// input data.
    ///
    /// For example, a function that reads a file into a string will error with
    /// `InvalidData` if the file's contents are not valid UTF-8.
    ///
    /// [`InvalidInput`]: ErrorKind::InvalidInput
    InvalidData,
    /// The I/O operation's timeout expired, causing it to be canceled.
    TimedOut,
    /// This operation was interrupted.
    ///
    /// Interrupted operations can typically be retried.
    Interrupted,
    /// This operation is unsupported on this platform.
    ///
    /// This means that the operation can never succeed.
    Unsupported,
    /// An operation could not be completed, because it failed
    /// to allocate enough memory.
    OutOfMemory,
    /// An attempted write could not write any data.
    WriteZero,
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl From<ErrorKind> for std::io::ErrorKind {
    fn from(value: ErrorKind) -> Self {
        match value {
            ErrorKind::NotFound => std::io::ErrorKind::NotFound,
            ErrorKind::PermissionDenied => std::io::ErrorKind::PermissionDenied,
            ErrorKind::ConnectionRefused => std::io::ErrorKind::ConnectionRefused,
            ErrorKind::ConnectionReset => std::io::ErrorKind::ConnectionReset,
            ErrorKind::ConnectionAborted => std::io::ErrorKind::ConnectionAborted,
            ErrorKind::NotConnected => std::io::ErrorKind::NotConnected,
            ErrorKind::AddrInUse => std::io::ErrorKind::AddrInUse,
            ErrorKind::AddrNotAvailable => std::io::ErrorKind::AddrNotAvailable,
            ErrorKind::BrokenPipe => std::io::ErrorKind::BrokenPipe,
            ErrorKind::AlreadyExists => std::io::ErrorKind::AlreadyExists,
            ErrorKind::InvalidInput => std::io::ErrorKind::InvalidInput,
            ErrorKind::InvalidData => std::io::ErrorKind::InvalidData,
            ErrorKind::TimedOut => std::io::ErrorKind::TimedOut,
            ErrorKind::Interrupted => std::io::ErrorKind::Interrupted,
            ErrorKind::Unsupported => std::io::ErrorKind::Unsupported,
            ErrorKind::OutOfMemory => std::io::ErrorKind::OutOfMemory,
            ErrorKind::WriteZero => std::io::ErrorKind::WriteZero,
            _ => std::io::ErrorKind::Other,
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl From<std::io::ErrorKind> for ErrorKind {
    fn from(value: std::io::ErrorKind) -> Self {
        match value {
            std::io::ErrorKind::NotFound => ErrorKind::NotFound,
            std::io::ErrorKind::PermissionDenied => ErrorKind::PermissionDenied,
            std::io::ErrorKind::ConnectionRefused => ErrorKind::ConnectionRefused,
            std::io::ErrorKind::ConnectionReset => ErrorKind::ConnectionReset,
            std::io::ErrorKind::ConnectionAborted => ErrorKind::ConnectionAborted,
            std::io::ErrorKind::NotConnected => ErrorKind::NotConnected,
            std::io::ErrorKind::AddrInUse => ErrorKind::AddrInUse,
            std::io::ErrorKind::AddrNotAvailable => ErrorKind::AddrNotAvailable,
            std::io::ErrorKind::BrokenPipe => ErrorKind::BrokenPipe,
            std::io::ErrorKind::AlreadyExists => ErrorKind::AlreadyExists,
            std::io::ErrorKind::InvalidInput => ErrorKind::InvalidInput,
            std::io::ErrorKind::InvalidData => ErrorKind::InvalidData,
            std::io::ErrorKind::TimedOut => ErrorKind::TimedOut,
            std::io::ErrorKind::Interrupted => ErrorKind::Interrupted,
            std::io::ErrorKind::Unsupported => ErrorKind::Unsupported,
            std::io::ErrorKind::OutOfMemory => ErrorKind::OutOfMemory,
            std::io::ErrorKind::WriteZero => ErrorKind::WriteZero,
            _ => ErrorKind::Other,
        }
    }
}

/// Error trait.
///
/// This trait allows generic code to do limited inspecting of errors,
/// to react differently to different kinds.
pub trait Error: core::error::Error {
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

impl core::error::Error for ErrorKind {}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl Error for std::io::Error {
    fn kind(&self) -> ErrorKind {
        self.kind().into()
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

impl<T: ?Sized + ErrorType> ErrorType for &mut T {
    type Error = T::Error;
}

/// Error returned by [`Read::read_exact`]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl From<ReadExactError<std::io::Error>> for std::io::Error {
    fn from(err: ReadExactError<std::io::Error>) -> Self {
        match err {
            ReadExactError::UnexpectedEof => std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "UnexpectedEof".to_owned(),
            ),
            ReadExactError::Other(e) => std::io::Error::new(e.kind(), format!("{e:?}")),
        }
    }
}

impl<E: fmt::Debug> fmt::Display for ReadExactError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl<E: fmt::Debug> core::error::Error for ReadExactError<E> {}

/// Errors that could be returned by `Write` on `&mut [u8]`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum SliceWriteError {
    /// The target slice was full and so could not receive any new data.
    Full,
}

/// Error returned by [`Write::write_fmt`]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WriteFmtError<E> {
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
        write!(f, "{self:?}")
    }
}

impl<E: fmt::Debug> core::error::Error for WriteFmtError<E> {}

/// Blocking reader.
///
/// This trait is the `embedded-io` equivalent of [`std::io::Read`].
pub trait Read: ErrorType {
    /// Read some bytes from this source into the specified buffer, returning how many bytes were read.
    ///
    /// If no bytes are currently available to read:
    /// - The method blocks until at least one byte becomes available;
    /// - Once at least one (or more) bytes become available, a non-zero amount of those is copied to the
    ///   beginning of `buf`, and the amount is returned, *without waiting or blocking any further for
    ///   more bytes to become available*.
    ///
    /// If bytes are available to read:
    /// - A non-zero amount of bytes is read to the beginning of `buf`, and the amount is returned immediately,
    ///   *without blocking and waiting for more bytes to become available*;
    ///
    /// Note that once some bytes are available to read, it is *not* guaranteed that all available bytes are returned.
    /// It is possible for the implementation to read an amount of bytes less than `buf.len()` while there are more
    /// bytes immediately available.
    ///
    /// This blocking behavior is important for the cases where `Read` represents the "read" leg of a pipe-like
    /// protocol (a socket, a pipe, a serial line etc.). The semantics is that the caller - by passing a non-empty
    /// buffer - does expect _some_ data (one or more bytes) - but _not necessarily `buf.len()` or more bytes_ -
    /// to become available, before the peer represented by `Read` would stop sending bytes due to
    /// application-specific reasons (as in the peer waiting for a response to the data it had sent so far).
    ///
    /// If the reader is at end-of-file (EOF), `Ok(0)` is returned. There is no guarantee that a reader at EOF
    /// will always be so in the future, for example a reader can stop being at EOF if another process appends
    /// more bytes to the underlying file.
    ///
    /// If `buf.len() == 0`, `read` returns without blocking, with either `Ok(0)` or an error.
    /// The `Ok(0)` doesn't indicate EOF, unlike when called with a non-empty buffer.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;

    /// Read the exact number of bytes required to fill `buf`.
    ///
    /// This function calls `read()` in a loop until exactly `buf.len()` bytes have
    /// been read, blocking if needed.
    ///
    /// If you are using [`ReadReady`] to avoid blocking, you should not use this function.
    /// `ReadReady::read_ready()` returning true only guarantees the first call to `read()` will
    /// not block, so this function may still block in subsequent calls.
    fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<(), ReadExactError<Self::Error>> {
        while !buf.is_empty() {
            match self.read(buf) {
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

/// The [BufReader] adds buffering to any reader, analogous to [`std::io::BufReader`]
///
/// This [BufReader] allocates it's own internal buffer of size [N].
///
/// # Examples
///
/// ```
/// use embedded_io::BufReader;
///
/// fn main()-> Result<(),>
/// {
///     let reader = [0,1,2,3];
///     let mut buf_reader: BufReader<4,&[u8]> = BufReader::new(&reader);
///     
///     let current_buff = buf_reader.fill_buff()?;
///
///     buf_reader.consume(4);
///     
/// }
///
/// ```
pub struct BufReader<const N: usize, R: ?Sized> {
    buff: [u8; N],
    pos: usize,
    inner: R,
}

impl<const N: usize, R: ?Sized> BufReader<N, R> {
    /// Gets a reference to the underlying reader.
    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    /// Gets a mutable reference to the underlying reader.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }

    /// Returns a reference to the internally buffered data.
    ///
    /// Unlike `fill_buff` this will not attempt to fill the buffer it if is empty.
    pub fn buffer(&self) -> &[u8] {
        &self.buff
    }

    /// Returns the number of bytes the internal buffer can hold at once.
    pub fn capacity(&self) -> usize {
        N
    }

    /// Unwraps this [BufReader<N,R>], returning the underlying reader.
    pub fn into_inner(self) -> R
    where
        R: Sized,
    {
        self.inner
    }
}

impl<const N: usize, R: Read> BufReader<N, R> {
    /// Creates a new [BufReader<N,R>] with a buffer capacity of `N`.
    pub fn new(reader: R) -> Self {
        Self {
            buff: [0u8; N],
            pos: 0,
            inner: reader,
        }
    }
}

impl<const N: usize, R: Read> ErrorType for BufReader<N, R> {
    type Error = R::Error;
}

impl<const N: usize, R: Read> BufRead for BufReader<N, R> {
    fn consume(&mut self, amt: usize) {
        // remove amt bytes from the front of the buffer
        // imagine the buffer is [0,1,2,3,4]
        // consume(2)
        // the buffer is now [2,3,4]
        self.buff.copy_within(amt..self.pos, 0);
        self.pos -= amt;
    }

    fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
        // fill the inner buffer
        let read_count = self.inner.read(&mut self.buff[self.pos..])?;
        self.pos += read_count;

        Ok(&self.buff[..self.pos])
    }
}

impl<const N: usize, R: Read> Read for BufReader<N, R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let mut rem = self.fill_buf()?;
        let nread = rem.read(buf).unwrap(); // infallible

        self.consume(nread);
        Ok(nread)
    }
}

/// Blocking buffered reader.
///
/// This trait is the `embedded-io` equivalent of [`std::io::BufRead`].
pub trait BufRead: Read {
    /// Return the contents of the internal buffer, filling it with more data from the inner reader if it is empty.
    ///
    /// If no bytes are currently available to read, this function blocks until at least one byte is available.
    ///
    /// If the reader is at end-of-file (EOF), an empty slice is returned. There is no guarantee that a reader at EOF
    /// will always be so in the future, for example a reader can stop being at EOF if another process appends
    /// more bytes to the underlying file.
    fn fill_buf(&mut self) -> Result<&[u8], Self::Error>;

    /// Tell this buffer that `amt` bytes have been consumed from the buffer, so they should no longer be returned in calls to `fill_buf`.
    fn consume(&mut self, amt: usize);
}

/// Blocking writer.
///
/// This trait is the `embedded-io` equivalent of [`std::io::Write`].
pub trait Write: ErrorType {
    /// Write a buffer into this writer, returning how many bytes were written.
    ///
    /// If the writer is not currently ready to accept more bytes (for example, its buffer is full),
    /// this function blocks until it is ready to accept least one byte.
    ///
    /// If it's ready to accept bytes, a non-zero amount of bytes is written from the beginning of `buf`, and the amount
    /// is returned. It is not guaranteed that *all* available buffer space is filled, i.e. it is possible for the
    /// implementation to write an amount of bytes less than `buf.len()` while the writer continues to be
    /// ready to accept more bytes immediately.
    ///
    /// Implementations must not return `Ok(0)` unless `buf` is empty. Situations where the
    /// writer is not able to accept more bytes must instead be indicated with an error,
    /// where the `ErrorKind` is `WriteZero`.
    ///
    /// If `buf` is empty, `write` returns without blocking, with either `Ok(0)` or an error.
    /// `Ok(0)` doesn't indicate an error.
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error>;

    /// Flush this output stream, blocking until all intermediately buffered contents reach their destination.
    fn flush(&mut self) -> Result<(), Self::Error>;

    /// Write an entire buffer into this writer.
    ///
    /// This function calls `write()` in a loop until exactly `buf.len()` bytes have
    /// been written, blocking if needed.
    ///
    /// If you are using [`WriteReady`] to avoid blocking, you should not use this function.
    /// `WriteReady::write_ready()` returning true only guarantees the first call to `write()` will
    /// not block, so this function may still block in subsequent calls.
    ///
    /// This function will panic if `write()` returns `Ok(0)`.
    fn write_all(&mut self, mut buf: &[u8]) -> Result<(), Self::Error> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => panic!("write() returned Ok(0)"),
                Ok(n) => buf = &buf[n..],
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    /// Write a formatted string into this writer, returning any error encountered.
    ///
    /// This function calls `write()` in a loop until the entire formatted string has
    /// been written, blocking if needed.
    ///
    /// If you are using [`WriteReady`] to avoid blocking, you should not use this function.
    /// `WriteReady::write_ready()` returning true only guarantees the first call to `write()` will
    /// not block, so this function may still block in subsequent calls.
    ///
    /// Unlike [`Write::write`], the number of bytes written is not returned. However, in the case of
    /// writing to an `&mut [u8]` its possible to calculate the number of bytes written by subtracting
    /// the length of the slice after the write, from the initial length of the slice.
    ///
    /// ```rust
    /// # use embedded_io::Write;
    /// let mut buf: &mut [u8] = &mut [0u8; 256];
    /// let start = buf.len();
    /// let len = write!(buf, "{}", "Test").and_then(|_| Ok(start - buf.len()));
    /// ```
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> Result<(), WriteFmtError<Self::Error>> {
        // Create a shim which translates a Write to a fmt::Write and saves
        // off I/O errors. instead of discarding them
        struct Adapter<'a, T: Write + ?Sized + 'a> {
            inner: &'a mut T,
            error: Result<(), T::Error>,
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
                Err(e) => Err(WriteFmtError::Other(e)),
                Ok(()) => Err(WriteFmtError::FmtError),
            },
        }
    }
}

/// Blocking seek within streams.\
///
/// The `Seek` trait provides a cursor which can be moved within a stream of
/// bytes.
///
/// The stream typically has a fixed size, allowing seeking relative to either
/// end or the current offset.
///
/// This trait is the `embedded-io` equivalent of [`std::io::Seek`].
pub trait Seek: ErrorType {
    /// Seek to an offset, in bytes, in a stream.
    /// A seek beyond the end of a stream is allowed, but behavior is defined
    /// by the implementation.
    ///
    /// If the seek operation completed successfully,
    /// this method returns the new position from the start of the stream.
    /// That position can be used later with [`SeekFrom::Start`].
    ///
    /// # Errors
    ///
    /// Seeking can fail, for example because it might involve flushing a buffer.
    ///
    /// Seeking to a negative offset is considered an error.
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Self::Error>;

    /// Rewind to the beginning of a stream.
    ///
    /// This is a convenience method, equivalent to `seek(SeekFrom::Start(0))`.
    ///
    /// # Errors
    ///
    /// Rewinding can fail, for example because it might involve flushing a buffer.
    fn rewind(&mut self) -> Result<(), Self::Error> {
        self.seek(SeekFrom::Start(0))?;
        Ok(())
    }

    /// Returns the current seek position from the start of the stream.
    ///
    /// This is equivalent to `self.seek(SeekFrom::Current(0))`.
    fn stream_position(&mut self) -> Result<u64, Self::Error> {
        self.seek(SeekFrom::Current(0))
    }

    /// Seeks relative to the current position.
    ///
    /// This is equivalent to `self.seek(SeekFrom::Current(offset))` but
    /// doesn't return the new position which can allow some implementations
    /// to perform more efficient seeks.
    fn seek_relative(&mut self, offset: i64) -> Result<(), Self::Error> {
        self.seek(SeekFrom::Current(offset))?;
        Ok(())
    }
}

/// Get whether a reader is ready.
///
/// This allows using a [`Read`] or [`BufRead`] in a nonblocking fashion, i.e. trying to read
/// only when it is ready.
pub trait ReadReady: Read {
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
pub trait WriteReady: Write {
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

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), ReadExactError<Self::Error>> {
        T::read_exact(self, buf)
    }
}

impl<T: ?Sized + BufRead> BufRead for &mut T {
    #[inline]
    fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
        T::fill_buf(self)
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        T::consume(self, amt);
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

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        T::write_all(self, buf)
    }
}

impl<T: ?Sized + Seek> Seek for &mut T {
    #[inline]
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Self::Error> {
        T::seek(self, pos)
    }

    #[inline]
    fn rewind(&mut self) -> Result<(), Self::Error> {
        T::rewind(self)
    }

    #[inline]
    fn stream_position(&mut self) -> Result<u64, Self::Error> {
        T::stream_position(self)
    }

    #[inline]
    fn seek_relative(&mut self, offset: i64) -> Result<(), Self::Error> {
        T::seek_relative(self, offset)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bufread_consume_removes_bytes() {
        let reader = [0, 1, 2, 3];

        let mut buf_read: BufReader<4, &[u8]> = BufReader::new(&reader);

        // read bytes
        let current_buff = buf_read.fill_buf().unwrap();

        assert_eq!(current_buff, [0, 1, 2, 3]);

        // consume bytes
        buf_read.consume(2);

        assert_eq!(buf_read.fill_buf().unwrap(), [2, 3]);
    }

    #[test]
    #[should_panic]
    fn bufread_panics_if_consume_more_than_n_bytes() {
        let reader = [0, 1, 2, 3];

        let mut buf_read: BufReader<4, &[u8]> = BufReader::new(&reader);

        buf_read.consume(5);
    }

    #[test]
    #[should_panic]
    fn bufread_panics_if_consume_more_bytes_than_filled() {
        let reader = [0, 1, 2, 3];

        let mut buf_read: BufReader<4, &[u8]> = BufReader::new(&reader);

        buf_read.consume(4);
    }
}
