//! Adapters to/from `tokio::io` traits.

use core::future::poll_fn;
use core::pin::Pin;
use core::task::Poll;

/// Adapter from `tokio::io` traits.
#[derive(Clone)]
pub struct FromTokio<T: ?Sized> {
    inner: T,
}

impl<T> FromTokio<T> {
    /// Create a new adapter.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Consume the adapter, returning the inner object.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: ?Sized> FromTokio<T> {
    /// Borrow the inner object.
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Mutably borrow the inner object.
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: ?Sized> embedded_io::ErrorType for FromTokio<T> {
    type Error = std::io::Error;
}

impl<T: tokio::io::AsyncRead + Unpin + ?Sized> embedded_io_async::Read for FromTokio<T> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        poll_fn(|cx| {
            let mut buf = tokio::io::ReadBuf::new(buf);
            match Pin::new(&mut self.inner).poll_read(cx, &mut buf) {
                Poll::Ready(r) => match r {
                    Ok(()) => Poll::Ready(Ok(buf.filled().len())),
                    Err(e) => Poll::Ready(Err(e)),
                },
                Poll::Pending => Poll::Pending,
            }
        })
        .await
    }
}

impl<T: tokio::io::AsyncWrite + Unpin + ?Sized> embedded_io_async::Write for FromTokio<T> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        poll_fn(|cx| Pin::new(&mut self.inner).poll_write(cx, buf)).await
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        poll_fn(|cx| Pin::new(&mut self.inner).poll_flush(cx)).await
    }
}

impl<T: tokio::io::AsyncSeek + Unpin + ?Sized> embedded_io_async::Seek for FromTokio<T> {
    async fn seek(&mut self, pos: embedded_io::SeekFrom) -> Result<u64, Self::Error> {
        // Note: `start_seek` can return an error if there is another seek in progress.
        // Therefor it is recommended to call `poll_complete` before any call to `start_seek`.
        poll_fn(|cx| Pin::new(&mut self.inner).poll_complete(cx)).await?;
        Pin::new(&mut self.inner).start_seek(pos.into())?;
        poll_fn(|cx| Pin::new(&mut self.inner).poll_complete(cx)).await
    }
}

// TODO: ToTokio.
// It's a bit tricky because tokio::io is "stateless", while we're "stateful" (we
// return futures that borrow Self and get polled for the duration of the operation.)
// It can probably done by storing the futures in Self, with unsafe Pin hacks because
// we're a self-referential struct
