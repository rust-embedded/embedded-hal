#![deny(
    clippy::missing_trait_methods,
    reason = "Methods should be forwarded to the underlying type"
)]

use core::fmt;

use crate::{
    BufRead, Read, ReadExactError, ReadReady, Seek, SeekFrom, Write, WriteFmtError, WriteReady,
};

impl<T: ?Sized + Read> Read for &mut T {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        T::read(self, buf)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), ReadExactError<Self::Error>> {
        T::read_exact(self, buf)
    }
}

impl<T: ?Sized + BufRead> BufRead for &mut T {
    #[inline]
    fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
        T::fill_buf(self)
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        T::consume(self, amt);
    }
}

impl<T: ?Sized + Write> Write for &mut T {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        T::write(self, buf)
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        T::flush(self)
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        T::write_all(self, buf)
    }

    #[inline]
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> Result<(), WriteFmtError<Self::Error>> {
        T::write_fmt(self, fmt)
    }
}

impl<T: ?Sized + Seek> Seek for &mut T {
    #[inline]
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Self::Error> {
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

impl<T: ?Sized + ReadReady> ReadReady for &mut T {
    #[inline]
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        T::read_ready(self)
    }
}

impl<T: ?Sized + WriteReady> WriteReady for &mut T {
    #[inline]
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        T::write_ready(self)
    }
}
