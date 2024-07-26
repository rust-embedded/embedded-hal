//! Adapters to the `core::fmt::Write`.

/// Adapter to the `core::fmt::Write` trait.
#[derive(Clone, Default, PartialEq, Debug)]
pub struct ToFmt<T: ?Sized> {
    inner: T,
}

impl<T> ToFmt<T> {
    /// Create a new adapter.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Consume the adapter, returning the inner object.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: ?Sized> ToFmt<T> {
    /// Borrow the inner object.
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Mutably borrow the inner object.
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: embedded_io::Write + ?Sized> core::fmt::Write for ToFmt<T> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.inner.write_all(s.as_bytes()).or(Err(core::fmt::Error))
    }

    // Use fmt::Write default impls for
    // * write_fmt(): better here than e-io::Write::write_fmt
    //   since we don't need to bother with saving the Error
    // * write_char(): would be the same
}
