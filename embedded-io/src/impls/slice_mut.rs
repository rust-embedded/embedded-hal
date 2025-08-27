use crate::{ErrorKind, ErrorType, SliceWriteError, Write, WriteReady};
use core::mem;

impl ErrorType for &mut [u8] {
    type Error = ErrorKind;
}

impl core::fmt::Display for SliceWriteError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl core::error::Error for SliceWriteError {}

/// Write is implemented for `&mut [u8]` by copying into the slice, overwriting
/// its data.
///
/// Note that writing updates the slice to point to the yet unwritten part.
/// The slice will be empty when it has been completely overwritten.
///
/// If the number of bytes to be written exceeds the size of the slice, write operations will
/// return short writes: ultimately, a `SliceWriteError::Full`.
impl Write for &mut [u8] {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let amt = core::cmp::min(buf.len(), self.len());
        if !buf.is_empty() && amt == 0 {
            return Err(ErrorKind::StorageFull);
        }
        let (a, b) = mem::take(self).split_at_mut(amt);
        a.copy_from_slice(&buf[..amt]);
        *self = b;
        Ok(amt)
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl WriteReady for &mut [u8] {
    #[inline]
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        Ok(true)
    }
}
