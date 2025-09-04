//! Adapted from std.

use core::convert::Infallible;

use alloc::collections::vec_deque::VecDeque;

use crate::{BufRead, ErrorType, Read, ReadExactError, Write};

#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
impl ErrorType for VecDeque<u8> {
    type Error = Infallible;
}

/// Read is implemented for `VecDeque<u8>` by consuming bytes from the front of the `VecDeque`.
#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
impl Read for VecDeque<u8> {
    /// Fill `buf` with the contents of the "front" slice as returned by
    /// [`as_slices`][`VecDeque::as_slices`]. If the contained byte slices of the `VecDeque` are
    /// discontiguous, multiple calls to `read` will be needed to read the entire content.
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let (ref mut front, _) = self.as_slices();
        let n = Read::read(front, buf)?;
        self.drain(..n);
        Ok(n)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), ReadExactError<Self::Error>> {
        let (front, back) = self.as_slices();

        // Use only the front buffer if it is big enough to fill `buf`, else use
        // the back buffer too.
        match buf.split_at_mut_checked(front.len()) {
            None => buf.copy_from_slice(&front[..buf.len()]),
            Some((buf_front, buf_back)) => match back.split_at_checked(buf_back.len()) {
                Some((back, _)) => {
                    buf_front.copy_from_slice(front);
                    buf_back.copy_from_slice(back);
                }
                None => {
                    self.clear();
                    return Err(ReadExactError::UnexpectedEof);
                }
            },
        }

        self.drain(..buf.len());
        Ok(())
    }
}

/// BufRead is implemented for `VecDeque<u8>` by reading bytes from the front of the `VecDeque`.
#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
impl BufRead for VecDeque<u8> {
    /// Returns the contents of the "front" slice as returned by
    /// [`as_slices`][`VecDeque::as_slices`]. If the contained byte slices of the `VecDeque` are
    /// discontiguous, multiple calls to `fill_buf` will be needed to read the entire content.
    #[inline]
    fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
        let (front, _) = self.as_slices();
        Ok(front)
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        self.drain(..amt);
    }
}

/// Write is implemented for `VecDeque<u8>` by appending to the `VecDeque`, growing it as needed.
#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
impl Write for VecDeque<u8> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.extend(buf);
        Ok(buf.len())
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.extend(buf);
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
