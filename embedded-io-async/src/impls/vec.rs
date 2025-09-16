use alloc::vec::Vec;

use crate::Write;

#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
impl Write for Vec<u8> {
    #[inline]
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.extend_from_slice(buf);
        Ok(buf.len())
    }

    #[inline]
    async fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.write(buf).await?;
        Ok(())
    }
}
