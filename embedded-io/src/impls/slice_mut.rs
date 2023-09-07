use crate::{ErrorType, Write};
use core::mem;

impl ErrorType for &mut [u8] {
    type Error = core::convert::Infallible;
}

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
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let amt = core::cmp::min(buf.len(), self.len());
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

#[cfg(test)]
mod test {
    use crate::Write;

    #[test]
    fn basic_length() {
        let mut buf = [0u8; 1024];
        let len = write!(&mut buf[..], "Hello!").unwrap();
        assert!(len == "Hello!".as_bytes().len());
    }

    #[test]
    fn format_length() {
        let mut buf = [0u8; 1024];
        let len = write!(&mut buf[..], "Hello, {}!", "World").unwrap();
        assert!(len == "Hello, World!".as_bytes().len());
    }
}
