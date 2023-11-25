use crate::{BufRead, Read, Seek, SeekFrom, Write};
use alloc::boxed::Box;

#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
impl<T: ?Sized + Read> Read for Box<T> {
    #[inline]
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        T::read(self, buf).await
    }
}

#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
impl<T: ?Sized + BufRead> BufRead for Box<T> {
    #[inline]
    async fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
        T::fill_buf(self).await
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        T::consume(self, amt);
    }
}

#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
impl<T: ?Sized + Write> Write for Box<T> {
    #[inline]
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        T::write(self, buf).await
    }

    #[inline]
    async fn flush(&mut self) -> Result<(), Self::Error> {
        T::flush(self).await
    }
}

#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
impl<T: ?Sized + Seek> Seek for Box<T> {
    #[inline]
    async fn seek(&mut self, pos: SeekFrom) -> Result<u64, Self::Error> {
        T::seek(self, pos).await
    }
}
