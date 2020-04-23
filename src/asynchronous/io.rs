//! Common IO primitives.
//!
//! These primitives are closely mirroring the definitions in
//! [`tokio-io`](https://docs.rs/tokio-io).  A big difference is that these definitions are not tied
//! to `std::io::Error`, but instead allow for custom error types, and also don't require
//! allocation.

use core::fmt;
use core::pin;
use core::task;

pub mod flush;
pub mod read;
pub mod read_exact;
pub mod shutdown;
pub mod write;
pub mod write_all;

/// Reads bytes from a source.
pub trait Read: fmt::Debug {
    /// The type of error that can occur during a read operation.
    type Error: ReadError;

    /// Attempts to read into the provided buffer.
    ///
    /// On success, returns the number of bytes read.
    fn poll_read(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buffer: &mut [u8],
    ) -> task::Poll<Result<usize, Self::Error>>;
}

/// An error that might arise from read operations.
pub trait ReadError: fmt::Debug {
    /// Creates an error that indicates an EOF (end-of-file) condition.
    ///
    /// This condition is to be used when there is not enough data available to satisfy a read
    /// operation.
    fn eof() -> Self;
}

/// Writes bytes to a source.
pub trait Write: fmt::Debug {
    /// The type of error that can occur during a write operation.
    type Error: WriteError;

    /// Attempts to write the contents of the provided buffer.
    ///
    /// On success, returns the number of bytes written.
    fn poll_write(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        bytes: &[u8],
    ) -> task::Poll<Result<usize, Self::Error>>;

    /// Attempts to flush the object, ensuring that any buffered data reach their destination.
    fn poll_flush(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), Self::Error>>;

    /// Initiates or attempts to shut down this writer, returning success when the I/O connection
    /// has completely shut down.
    fn poll_shutdown(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), Self::Error>>;
}

impl<A: ?Sized + Write + Unpin> Write for &mut A {
    type Error = A::Error;

    fn poll_write(
        mut self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        bytes: &[u8],
    ) -> task::Poll<Result<usize, Self::Error>> {
        pin::Pin::new(&mut **self).poll_write(cx, bytes)
    }

    fn poll_flush(
        mut self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), Self::Error>> {
        pin::Pin::new(&mut **self).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), Self::Error>> {
        pin::Pin::new(&mut **self).poll_shutdown(cx)
    }
}

/// An error that might arise from write operations.
pub trait WriteError: fmt::Debug {
    /// Creates an error that indicates a zero-write condition.
    ///
    /// This condition is to be used when a write operation requires more bytes to be written, but
    /// an attempt to write returned zero bytes, indicating that there's no more room for bytes
    /// to be written.
    fn write_zero() -> Self;
}

/// Utility methods for types implementing [`Read`].
pub trait ReadExt: Read {
    /// Reads data into the specified buffer, returning the number of bytes written.
    fn read<'a>(&'a mut self, buffer: &'a mut [u8]) -> read::Read<'a, Self>
    where
        Self: Unpin,
    {
        read::read(self, buffer)
    }

    /// Reads data into the specified buffer, until the buffer is filled.
    fn read_exact<'a>(&'a mut self, buffer: &'a mut [u8]) -> read_exact::ReadExact<'a, Self>
    where
        Self: Unpin,
    {
        read_exact::read_exact(self, buffer)
    }
}

impl<A> ReadExt for A where A: Read {}

/// Utility methods for types implementing [`Write`].
pub trait WriteExt: Write {
    /// Writes data from the specified buffer, returning the number of bytes written.
    fn write<'a>(&'a mut self, bytes: &'a [u8]) -> write::Write<'a, Self>
    where
        Self: Unpin,
    {
        write::write(self, bytes)
    }

    /// Writes data from the specified buffer until all bytes are written.
    fn write_all<'a>(&'a mut self, bytes: &'a [u8]) -> write_all::WriteAll<'a, Self>
    where
        Self: Unpin,
    {
        write_all::write_all(self, bytes)
    }

    /// Shuts down this writer.
    fn shutdown(&mut self) -> shutdown::Shutdown<Self>
    where
        Self: Unpin,
    {
        shutdown::shutdown(self)
    }
}

impl<A> WriteExt for A where A: Write {}
