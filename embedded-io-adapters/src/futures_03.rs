//! Adapters to/from `futures::io` traits.

// MSRV is 1.60 if you don't enable async, 1.80 if you do.
// Cargo.toml has 1.60, which makes Clippy complain that `poll_fn` was introduced
// in 1.64. So, just silence it for this file.
#![allow(clippy::incompatible_msrv)]

use core::future::poll_fn;
use core::pin::Pin;

use futures::AsyncBufReadExt;

/// Adapter from `futures::io` traits.
#[derive(Clone)]
pub struct FromFutures<T: ?Sized> {
    inner: T,
}

impl<T> FromFutures<T> {
    /// Create a new adapter.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Consume the adapter, returning the inner object.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: ?Sized> FromFutures<T> {
    /// Borrow the inner object.
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Mutably borrow the inner object.
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: ?Sized> embedded_io::ErrorType for FromFutures<T> {
    type Error = std::io::Error;
}

impl<T: futures::io::AsyncRead + Unpin + ?Sized> embedded_io_async::Read for FromFutures<T> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        poll_fn(|cx| Pin::new(&mut self.inner).poll_read(cx, buf)).await
    }
}

impl<T: futures::io::AsyncBufRead + Unpin + ?Sized> embedded_io_async::BufRead for FromFutures<T> {
    async fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
        self.inner.fill_buf().await
    }

    fn consume(&mut self, amt: usize) {
        Pin::new(&mut self.inner).consume(amt);
    }
}

impl<T: futures::io::AsyncWrite + Unpin + ?Sized> embedded_io_async::Write for FromFutures<T> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        match poll_fn(|cx| Pin::new(&mut self.inner).poll_write(cx, buf)).await {
            Ok(0) if !buf.is_empty() => Err(std::io::ErrorKind::WriteZero.into()),
            Ok(n) => Ok(n),
            Err(e) => Err(e),
        }
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        poll_fn(|cx| Pin::new(&mut self.inner).poll_flush(cx)).await
    }
}

impl<T: futures::io::AsyncSeek + Unpin + ?Sized> embedded_io_async::Seek for FromFutures<T> {
    async fn seek(&mut self, pos: embedded_io::SeekFrom) -> Result<u64, Self::Error> {
        poll_fn(move |cx| Pin::new(&mut self.inner).poll_seek(cx, pos.into())).await
    }
}

// TODO: ToFutures.
// It's a bit tricky because futures::io is "stateless", while we're "stateful" (we
// return futures that borrow Self and get polled for the duration of the operation.)
// It can probably done by storing the futures in Self, with unsafe Pin hacks because
// we're a self-referential struct
