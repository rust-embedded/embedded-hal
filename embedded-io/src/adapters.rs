//! Adapters to/from `std::io` traits.
//!
//! To interoperate with `std::io`, wrap a type in one of these
//! adapters.
//!
//! There are no separate adapters for `Read`/`ReadBuf`/`Write` traits. Instead, a single
//! adapter implements the right traits based on what the inner type implements.
//! This allows using these adapters when using `Read+Write`, for example.

use crate::SeekFrom;

/// Adapter from `std::io` traits.
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[derive(Clone)]
pub struct FromStd<T: ?Sized> {
    inner: T,
}

impl<T> FromStd<T> {
    /// Create a new adapter.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Consume the adapter, returning the inner object.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: ?Sized> FromStd<T> {
    /// Borrow the inner object.
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Mutably borrow the inner object.
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: ?Sized> crate::ErrorType for FromStd<T> {
    type Error = std::io::Error;
}

impl<T: std::io::Read + ?Sized> crate::Read for FromStd<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.inner.read(buf)
    }
}

impl<T: std::io::BufRead + ?Sized> crate::BufRead for FromStd<T> {
    fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
        self.inner.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt)
    }
}

impl<T: std::io::Write + ?Sized> crate::Write for FromStd<T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.inner.write(buf)
    }
    fn flush(&mut self) -> Result<(), Self::Error> {
        self.inner.flush()
    }
}

impl<T: std::io::Seek + ?Sized> crate::Seek for FromStd<T> {
    fn seek(&mut self, pos: crate::SeekFrom) -> Result<u64, Self::Error> {
        self.inner.seek(pos.into())
    }
}

/// Adapter to `std::io` traits.
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub struct ToStd<T: ?Sized> {
    inner: T,
}

impl<T> ToStd<T> {
    /// Create a new adapter.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Consume the adapter, returning the inner object.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: ?Sized> ToStd<T> {
    /// Borrow the inner object.
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Mutably borrow the inner object.
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: crate::Read + ?Sized> std::io::Read for ToStd<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.inner.read(buf).map_err(to_std_error)
    }
}

impl<T: crate::Write + ?Sized> std::io::Write for ToStd<T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        self.inner.write(buf).map_err(to_std_error)
    }
    fn flush(&mut self) -> Result<(), std::io::Error> {
        self.inner.flush().map_err(to_std_error)
    }
}

impl<T: crate::Seek + ?Sized> std::io::Seek for ToStd<T> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> Result<u64, std::io::Error> {
        self.inner.seek(pos.into()).map_err(to_std_error)
    }
}

fn to_std_error<T: crate::Error>(err: T) -> std::io::Error {
    let kind = match err.kind() {
        crate::ErrorKind::NotFound => std::io::ErrorKind::NotFound,
        crate::ErrorKind::PermissionDenied => std::io::ErrorKind::PermissionDenied,
        crate::ErrorKind::ConnectionRefused => std::io::ErrorKind::ConnectionRefused,
        crate::ErrorKind::ConnectionReset => std::io::ErrorKind::ConnectionReset,
        crate::ErrorKind::ConnectionAborted => std::io::ErrorKind::ConnectionAborted,
        crate::ErrorKind::NotConnected => std::io::ErrorKind::NotConnected,
        crate::ErrorKind::AddrInUse => std::io::ErrorKind::AddrInUse,
        crate::ErrorKind::AddrNotAvailable => std::io::ErrorKind::AddrNotAvailable,
        crate::ErrorKind::BrokenPipe => std::io::ErrorKind::BrokenPipe,
        crate::ErrorKind::AlreadyExists => std::io::ErrorKind::AlreadyExists,
        crate::ErrorKind::InvalidInput => std::io::ErrorKind::InvalidInput,
        crate::ErrorKind::InvalidData => std::io::ErrorKind::InvalidData,
        crate::ErrorKind::TimedOut => std::io::ErrorKind::TimedOut,
        crate::ErrorKind::Interrupted => std::io::ErrorKind::Interrupted,
        crate::ErrorKind::Unsupported => std::io::ErrorKind::Unsupported,
        crate::ErrorKind::OutOfMemory => std::io::ErrorKind::OutOfMemory,
        _ => std::io::ErrorKind::Other,
    };
    std::io::Error::new(kind, format!("{:?}", err))
}

#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl crate::Error for std::io::Error {
    fn kind(&self) -> crate::ErrorKind {
        match self.kind() {
            std::io::ErrorKind::NotFound => crate::ErrorKind::NotFound,
            std::io::ErrorKind::PermissionDenied => crate::ErrorKind::PermissionDenied,
            std::io::ErrorKind::ConnectionRefused => crate::ErrorKind::ConnectionRefused,
            std::io::ErrorKind::ConnectionReset => crate::ErrorKind::ConnectionReset,
            std::io::ErrorKind::ConnectionAborted => crate::ErrorKind::ConnectionAborted,
            std::io::ErrorKind::NotConnected => crate::ErrorKind::NotConnected,
            std::io::ErrorKind::AddrInUse => crate::ErrorKind::AddrInUse,
            std::io::ErrorKind::AddrNotAvailable => crate::ErrorKind::AddrNotAvailable,
            std::io::ErrorKind::BrokenPipe => crate::ErrorKind::BrokenPipe,
            std::io::ErrorKind::AlreadyExists => crate::ErrorKind::AlreadyExists,
            std::io::ErrorKind::InvalidInput => crate::ErrorKind::InvalidInput,
            std::io::ErrorKind::InvalidData => crate::ErrorKind::InvalidData,
            std::io::ErrorKind::TimedOut => crate::ErrorKind::TimedOut,
            std::io::ErrorKind::Interrupted => crate::ErrorKind::Interrupted,
            std::io::ErrorKind::Unsupported => crate::ErrorKind::Unsupported,
            std::io::ErrorKind::OutOfMemory => crate::ErrorKind::OutOfMemory,
            _ => crate::ErrorKind::Other,
        }
    }
}

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
