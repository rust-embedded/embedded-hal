#![deny(
    clippy::missing_trait_methods,
    reason = "Methods should be forwarded to the underlying type"
)]

use embedded_io::{ReadExactError, SeekFrom};

use crate::{BufRead, Read, Seek, Write};

impl<T: ?Sized + Read> Read for &mut T {
    #[inline]
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        T::read(self, buf).await
    }

    #[inline]
    async fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), ReadExactError<Self::Error>> {
        T::read_exact(self, buf).await
    }
}

impl<T: ?Sized + BufRead> BufRead for &mut T {
    #[inline]
    async fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
        T::fill_buf(self).await
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        T::consume(self, amt);
    }
}

impl<T: ?Sized + Write> Write for &mut T {
    #[inline]
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        T::write(self, buf).await
    }

    #[inline]
    async fn flush(&mut self) -> Result<(), Self::Error> {
        T::flush(self).await
    }

    #[inline]
    async fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        T::write_all(self, buf).await
    }
}

impl<T: ?Sized + Seek> Seek for &mut T {
    #[inline]
    async fn seek(&mut self, pos: SeekFrom) -> Result<u64, Self::Error> {
        T::seek(self, pos).await
    }

    #[inline]
    async fn rewind(&mut self) -> Result<(), Self::Error> {
        T::rewind(self).await
    }

    #[inline]
    async fn stream_position(&mut self) -> Result<u64, Self::Error> {
        T::stream_position(self).await
    }
}
