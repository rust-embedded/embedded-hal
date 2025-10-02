use crate::{BufRead, ErrorType, Read, ReadReady, Seek, Write, WriteReady};
use alloc::boxed::Box;

#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
#[deny(
    clippy::missing_trait_methods,
    reason = "Methods should be forwarded to the underlying type"
)]
impl<T: ?Sized + ErrorType> ErrorType for Box<T> {
    type Error = T::Error;
}

#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
#[deny(
    clippy::missing_trait_methods,
    reason = "Methods should be forwarded to the underlying type"
)]
impl<T: ?Sized + Read> Read for Box<T> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        T::read(self, buf)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), crate::ReadExactError<Self::Error>> {
        T::read_exact(self, buf)
    }
}

#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
#[deny(
    clippy::missing_trait_methods,
    reason = "Methods should be forwarded to the underlying type"
)]
impl<T: ?Sized + BufRead> BufRead for Box<T> {
    fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
        T::fill_buf(self)
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        T::consume(self, amt);
    }
}

#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
#[deny(
    clippy::missing_trait_methods,
    reason = "Methods should be forwarded to the underlying type"
)]
impl<T: ?Sized + Write> Write for Box<T> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        T::write(self, buf)
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        T::write_all(self, buf)
    }

    #[inline]
    fn write_fmt(
        &mut self,
        fmt: core::fmt::Arguments<'_>,
    ) -> Result<(), crate::WriteFmtError<Self::Error>> {
        T::write_fmt(self, fmt)
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        T::flush(self)
    }
}

#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
#[deny(
    clippy::missing_trait_methods,
    reason = "Methods should be forwarded to the underlying type"
)]
impl<T: ?Sized + Seek> Seek for Box<T> {
    #[inline]
    fn seek(&mut self, pos: crate::SeekFrom) -> Result<u64, Self::Error> {
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

#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
#[deny(
    clippy::missing_trait_methods,
    reason = "Methods should be forwarded to the underlying type"
)]
impl<T: ?Sized + ReadReady> ReadReady for Box<T> {
    #[inline]
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        T::read_ready(self)
    }
}

#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
#[deny(
    clippy::missing_trait_methods,
    reason = "Methods should be forwarded to the underlying type"
)]
impl<T: ?Sized + WriteReady> WriteReady for Box<T> {
    #[inline]
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        T::write_ready(self)
    }
}
