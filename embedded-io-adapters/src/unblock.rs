//! Adapters to/from `futures::io` traits.

// MSRV is 1.60 if you don't enable async, 1.80 if you do.
// Cargo.toml has 1.60, which makes Clippy complain that `poll_fn` was introduced
// in 1.64. So, just silence it for this file.
#![allow(clippy::incompatible_msrv)]

use std::sync::{Arc, Mutex};

use blocking::unblock;

/// Adapter from `embedded_io` traits to `embedded_io_async` traits.
///
/// This is not suitable for use in embedded environments, but it can be useful for quickly
/// iterating on driver code from your desktop without constantly re-flashing development boards.
///
/// This is quite inefficient, because it does IO operations on a threadpool, and does
/// an awful lot of copying. No attempt has been made to optimize this.
///
/// If you have access to a port implementing std::io::Read + std::io::Write and either
/// std::os::unix::io::AsRawFd or std::os::windows::io::AsRawSocket, you should attempt to use
/// `async_io::Async` followed by `embedded_io_adapters::futures_03::FromFutures` instead.
///
/// If you only need `embedded_io_async::Read` or `embedded_io_async::Write`, you can use
/// `UnblockRead` or `UnblockWrite`. In practice, most of the time you should just use this adapter.
///
/// The ergonomics of this are a bit worse than the other adapters because we need to avoid
/// overlapping impls of embedded_io::ErrorType.
pub struct Unblock<T: Send + Sync> {
    read: UnblockRead<T>,
    write: UnblockWrite<T>,
}

impl<T: Send + Sync + 'static> Unblock<T> {
    /// Create a new adapter.
    pub fn new(port: T) -> Self {
        let inner = Arc::new(Mutex::new(port));
        Self {
            read: UnblockRead {
                inner: inner.clone(),
            },
            write: UnblockWrite { inner },
        }
    }
}

impl<T: embedded_io::Read + embedded_io::Write + Send + Sync> embedded_io::ErrorType
    for Unblock<T>
{
    type Error = T::Error;
}

impl<T: embedded_io::Read + embedded_io::Write + Send + Sync + 'static> embedded_io_async::Read
    for Unblock<T>
where
    T::Error: Send + 'static,
{
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, T::Error> {
        self.read.read(buf).await
    }
}

impl<T: embedded_io::Read + embedded_io::Write + Send + Sync + 'static> embedded_io_async::Write
    for Unblock<T>
where
    T::Error: Send + 'static,
{
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.write.write(buf).await
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.write.flush().await
    }
}

/// Use this if you have a port that only implements `embedded_io::Read`. Otherwise, use `Unblock`.
///
/// The ergonomics of this are a bit worse than the other adapters because we need to avoid
/// overlapping impls of embedded_io::ErrorType.
pub struct UnblockRead<T: Send + Sync> {
    inner: Arc<Mutex<T>>,
}

impl<T: Send + Sync + 'static> UnblockRead<T> {
    /// Create a new adapter.
    pub fn new(port: T) -> Self {
        Self {
            inner: Arc::new(Mutex::new(port)),
        }
    }
}

impl<T: embedded_io::Read + Send + Sync + 'static> embedded_io_async::Read for UnblockRead<T>
where
    T::Error: Send + 'static,
{
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, T::Error> {
        let max_len = buf.len();
        let inner = self.inner.clone();

        let result = unblock(move || {
            let mut inner_buf: Vec<_> = std::iter::repeat(0u8).take(max_len).collect();
            match inner.lock().unwrap().read(&mut inner_buf) {
                Ok(count) => {
                    inner_buf.resize(count, 0);
                    Ok(inner_buf)
                }
                Err(e) => Err(e),
            }
        })
        .await;

        match result {
            Ok(inner_buf) => {
                buf[..inner_buf.len()].copy_from_slice(&inner_buf);
                Ok(inner_buf.len())
            }
            Err(e) => Err(e),
        }
    }
}

impl<T: embedded_io::Read + Send + Sync> embedded_io::ErrorType for UnblockRead<T> {
    type Error = T::Error;
}

/// Use this if you have a port that only implements `embedded_io::Write`. Otherwise, use `Unblock`.
///
/// The ergonomics of this are a bit worse than the other adapters because we need to avoid
/// overlapping impls of embedded_io::ErrorType.
pub struct UnblockWrite<T: Send + Sync> {
    inner: Arc<Mutex<T>>,
}

impl<T: Send + Sync + 'static> UnblockWrite<T> {
    /// Create a new adapter.
    pub fn new(port: T) -> Self {
        Self {
            inner: Arc::new(Mutex::new(port)),
        }
    }
}

impl<T: embedded_io::Write + Send + Sync> embedded_io::ErrorType for UnblockWrite<T> {
    type Error = T::Error;
}

impl<T: embedded_io::Write + Send + Sync + 'static> embedded_io_async::Write for UnblockWrite<T>
where
    T::Error: Send + 'static,
{
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let inner = self.inner.clone();
        let inner_buf = Vec::from(buf);

        unblock(move || {
            let inner_buf = inner_buf;
            inner.lock().unwrap().write(&inner_buf)
        })
        .await
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        let inner = self.inner.clone();
        unblock(move || inner.lock().unwrap().flush()).await
    }
}
