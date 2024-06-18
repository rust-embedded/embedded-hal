//! Adapters to/from `embedded_storage` traits.

// needed to prevent defmt macros from breaking, since they emit code that does `defmt::blahblah`.
#[cfg(feature = "defmt-03")]
use defmt_03 as defmt;

use embedded_io::{Error, ErrorKind, Read, Seek, SeekFrom, Write};
use embedded_storage::nor_flash::{NorFlashError, NorFlashErrorKind};
use embedded_storage::{ReadStorage, Storage};

/// Adapter from `embedded_storage` traits.
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

impl<T: ?Sized> embedded_io::ErrorType for FromEmbeddedStorage<T> {
    type Error = StorageIOError;
}

impl<T: ReadStorage<Error = E> + ?Sized, E: Into<StorageIOError>> embedded_io::Read
    for FromEmbeddedStorage<T>
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.inner.read(self.position, buf).map_err(|e| e.into())?;
        self.position += buf.len() as u32;
        Ok(buf.len())
    }
}

impl<T: Storage<Error = E> + ?Sized, E: Into<StorageIOError>> embedded_io::Write
    for FromEmbeddedStorage<T>
{
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.inner.write(self.position, buf).map_err(|e| e.into())?;
        self.position += buf.len() as u32;
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<T: ReadStorage<Error = E> + ?Sized, E: Into<StorageIOError>> embedded_io::Seek
    for FromEmbeddedStorage<T>
{
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Self::Error> {
        let new_position = match pos {
            SeekFrom::Start(pos) => pos as i64,
            SeekFrom::End(offset) => self.inner.capacity() as i64 + offset,
            SeekFrom::Current(offset) => self.position as i64 + offset,
        };
        self.position = new_position as u32;
        Ok(self.position as u64)
    }
}

/// Adapter to `embedded_storage` traits.
#[derive(Clone)]
pub struct ToEmbeddedStorage<T: ?Sized> {
    capacity: usize,
    inner: T,
}

impl<T: Seek> ToEmbeddedStorage<T> {
    /// Create a new adapter.
    pub fn new(mut inner: T) -> Self {
        let capacity = inner.seek(SeekFrom::End(0)).unwrap() as usize;
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

    fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        self.inner.seek(SeekFrom::Start(offset as u64))?;
        let mut read = 0;
        while read < bytes.len() {
            read += self.inner.read(&mut bytes[read..])?;
        }
        Ok(())
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T: Read + Write + Seek + ?Sized> Storage for ToEmbeddedStorage<T> {
    fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        self.inner.seek(SeekFrom::Start(offset as u64))?;
        let mut written = 0;
        while written < bytes.len() {
            written += self.inner.write(&bytes[written..])?;
        }
        Ok(())
    }
}

/// An error type that is implementing embedded_io::Error and that the concret
/// Storage::Error type should be able to convert to, to allow error compatibility
/// with the embedded_io layer.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct StorageIOError {
    kind: ErrorKind,
}

impl StorageIOError {
    /// Create a new StorageIOError.
    pub fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }
}

impl Error for StorageIOError {
    fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl<E: NorFlashError> From<E> for StorageIOError {
    fn from(value: E) -> Self {
        match value.kind() {
            NorFlashErrorKind::NotAligned => Self::new(ErrorKind::InvalidInput),
            NorFlashErrorKind::OutOfBounds => Self::new(ErrorKind::InvalidInput),
            _ => Self::new(ErrorKind::Other),
        }
    }
}
