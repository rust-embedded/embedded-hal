use crate::{BufRead, Read};

/// Read is implemented for `&[u8]` by copying from the slice.
///
/// Note that reading updates the slice to point to the yet unread part.
/// The slice will be empty when EOF is reached.
impl Read for &[u8] {
    #[inline]
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let amt = core::cmp::min(buf.len(), self.len());
        let (a, b) = self.split_at(amt);

        // First check if the amount of bytes we want to read is small:
        // `copy_from_slice` will generally expand to a call to `memcpy`, and
        // for a single byte the overhead is significant.
        if amt == 1 {
            buf[0] = a[0];
        } else {
            buf[..amt].copy_from_slice(a);
        }

        *self = b;
        Ok(amt)
    }

    async fn read_exact(
        &mut self,
        buf: &mut [u8],
    ) -> Result<(), embedded_io::ReadExactError<Self::Error>> {
        if self.len() < buf.len() {
            return Err(crate::ReadExactError::UnexpectedEof);
        }
        self.read(buf).await?;
        Ok(())
    }
}

impl BufRead for &[u8] {
    #[inline]
    async fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
        Ok(*self)
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        *self = &self[amt..];
    }
}
