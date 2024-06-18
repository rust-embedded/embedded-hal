//! Adapters to/from `embedded_storage_async` traits.

use embedded_io_async::{Read, Seek, SeekFrom, Write};
use embedded_storage_async::{ReadStorage, Storage};

pub use crate::embedded_storage::StorageIOError;

/// Adapter from `embedded_storage_async` traits.
#[derive(Clone)]
pub struct FromEmbeddedStorage<T: ?Sized> {
    position: u32,
    inner: T,
}

impl<T> FromEmbeddedStorage<T> {
    /// Create a new adapter.
    pub fn new(inner: T) -> Self {
        Self { position: 0, inner }
    }

    /// Consume the adapter, returning the inner object.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: ?Sized> FromEmbeddedStorage<T> {
    /// Borrow the inner object.
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Mutably borrow the inner object.
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: ?Sized> embedded_io_async::ErrorType for FromEmbeddedStorage<T> {
    type Error = StorageIOError;
}

impl<T: ReadStorage<Error = E> + ?Sized, E: Into<StorageIOError>> Read for FromEmbeddedStorage<T> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.inner
            .read(self.position, buf)
            .await
            .map_err(|e| e.into())?;
        self.position += buf.len() as u32;
        Ok(buf.len())
    }
}

impl<T: Storage<Error = E> + ?Sized, E: Into<StorageIOError>> Write for FromEmbeddedStorage<T> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.inner
            .write(self.position, buf)
            .await
            .map_err(|e| e.into())?;
        self.position += buf.len() as u32;
        Ok(buf.len())
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<T: ReadStorage<Error = E> + ?Sized, E: Into<StorageIOError>> Seek for FromEmbeddedStorage<T> {
    async fn seek(&mut self, pos: SeekFrom) -> Result<u64, Self::Error> {
        let new_position = match pos {
            SeekFrom::Start(pos) => pos as i64,
            SeekFrom::End(offset) => self.inner.capacity() as i64 + offset,
            SeekFrom::Current(offset) => self.position as i64 + offset,
        };
        self.position = new_position as u32;
        Ok(self.position as u64)
    }
}

/// Adapter to `embedded_storage_async` traits.
#[derive(Clone)]
pub struct ToEmbeddedStorage<T: ?Sized> {
    capacity: usize,
    inner: T,
}

impl<T: Seek> ToEmbeddedStorage<T> {
    /// Create a new adapter.
    pub async fn new(mut inner: T) -> Self {
        let capacity = inner.seek(SeekFrom::End(0)).await.unwrap() as usize;
        Self { inner, capacity }
    }

    /// Consume the adapter, returning the inner object.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: ?Sized> ToEmbeddedStorage<T> {
    /// Borrow the inner object.
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Mutably borrow the inner object.
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: Read + Seek + ?Sized> ReadStorage for ToEmbeddedStorage<T> {
    type Error = T::Error;

    async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        self.inner.seek(SeekFrom::Start(offset as u64)).await?;
        let mut read = 0;
        while read < bytes.len() {
            read += self.inner.read(&mut bytes[read..]).await?;
        }
        Ok(())
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T: Read + Write + Seek + ?Sized> Storage for ToEmbeddedStorage<T> {
    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        self.inner.seek(SeekFrom::Start(offset as u64)).await?;
        let mut written = 0;
        while written < bytes.len() {
            written += self.inner.write(&bytes[written..]).await?;
        }
        Ok(())
    }
}
