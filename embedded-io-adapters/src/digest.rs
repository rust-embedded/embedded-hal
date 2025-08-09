//! Adapters to/from `digest::Digest` traits e.g. sha2::Sha256

use core::convert::Infallible;
use digest::Update;
use embedded_io::{ErrorType, Write};

/// Adapter from `digest::Digest` traits.
#[derive(Clone)]
pub struct FromDigest<T: ?Sized> {
    inner: T,
}

impl<T> FromDigest<T> {
    /// Create a new adapter.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Consume the adapter, returning the inner object.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: ?Sized> FromDigest<T> {
    /// Borrow the inner object.
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Mutably borrow the inner object.
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T> ErrorType for FromDigest<T> {
    type Error = Infallible;
}

impl<T: Update> Write for FromDigest<T> {
    fn write(&mut self, data: &[u8]) -> Result<usize, <Self as ErrorType>::Error> {
        T::update(&mut self.inner, data);
        Ok(data.len())
    }
    fn flush(&mut self) -> Result<(), <Self as ErrorType>::Error> {
        Ok(())
    }
}

impl<T: Default + ?Sized> Default for FromDigest<T> {
    fn default() -> Self {
        Self {
            inner: T::default(),
        }
    }
}

/// Adapter to `digest::Digest` traits.
#[derive(Clone)]
pub struct ToDigest<T: ?Sized> {
    inner: T,
}

impl<T> ToDigest<T> {
    /// Create a new adapter.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Consume the adapter, returning the inner object.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: ?Sized> ToDigest<T> {
    /// Borrow the inner object.
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Mutably borrow the inner object.
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: Default + ?Sized> Default for ToDigest<T> {
    fn default() -> Self {
        Self {
            inner: T::default(),
        }
    }
}

impl<T: ErrorType<Error = Infallible> + Write> Update for ToDigest<T> {
    fn update(&mut self, data: &[u8]) {
        match self.inner.write_all(data) {
            Ok(()) => {}
            Err(_) => unreachable!(),
        }
        match self.inner.flush() {
            Ok(()) => {}
            Err(_) => unreachable!(),
        }
    }
}
