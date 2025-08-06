use core::mem::{self, MaybeUninit};
use embedded_io::SliceWriteError;

use crate::Write;

/// Write is implemented for `&mut [u8]` by copying into the slice, overwriting
/// its data.
///
/// Note that writing updates the slice to point to the yet unwritten part.
/// The slice will be empty when it has been completely overwritten.
///
/// If the number of bytes to be written exceeds the size of the slice, write operations will
/// return short writes: ultimately, `Ok(0)`; in this situation, `write_all` returns an error of
/// kind `ErrorKind::WriteZero`.
impl Write for &mut [u8] {
    #[inline]
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let amt = core::cmp::min(buf.len(), self.len());
        if !buf.is_empty() && amt == 0 {
            return Err(SliceWriteError::Full);
        }
        let (a, b) = mem::take(self).split_at_mut(amt);
        a.copy_from_slice(buf.split_at(amt).0);
        *self = b;
        Ok(amt)
    }
}

/// Write is implemented for `&mut [MaybeUninit<u8>]` by copying into the slice, initializing
/// & overwriting its data.
///
/// Note that writing updates the slice to point to the yet unwritten part.
/// The slice will be empty when it has been completely overwritten.
///
/// If the number of bytes to be written exceeds the size of the slice, write operations will
/// return short writes: ultimately, `Ok(0)`; in this situation, `write_all` returns an error of
/// kind `ErrorKind::WriteZero`.
impl Write for &mut [MaybeUninit<u8>] {
    #[inline]
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let amt = core::cmp::min(buf.len(), self.len());
        if !buf.is_empty() && amt == 0 {
            return Err(SliceWriteError::Full);
        }
        let (a, b) = mem::take(self).split_at_mut(amt);
        buf.split_at(amt)
            .0
            .iter()
            .enumerate()
            .for_each(|(index, byte)| {
                a[index].write(*byte);
            });
        *self = b;
        Ok(amt)
    }
}
