use crate::{BufRead, Io, Read, Seek, Write};
use alloc::boxed::Box;

#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
impl<T: ?Sized + Io> Io for Box<T> {
    type Error = T::Error;
}

#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
impl<T: ?Sized + Read> Read for Box<T> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        T::read(self, buf)
    }
}

#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
impl<T: ?Sized + BufRead> BufRead for Box<T> {
    fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
        T::fill_buf(self)
    }

    fn consume(&mut self, amt: usize) {
        T::consume(self, amt)
    }
}

#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
impl<T: ?Sized + Write> Write for Box<T> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        T::write(self, buf)
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        T::flush(self)
    }
}

#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
impl<T: ?Sized + Seek> Seek for Box<T> {
    #[inline]
    fn seek(&mut self, pos: crate::SeekFrom) -> Result<u64, Self::Error> {
        T::seek(self, pos)
    }
}
