use core::cmp;

use crate::{BufRead, ErrorKind, ErrorType, Read, ReadReady, Seek, SeekFrom, Write, WriteReady};

/// A `Cursor` wraps an in-memory buffer and provides it with a [`Seek`] implementation.
///
/// `Cursor`s are used with in-memory buffers, anything implementing [`AsRef<[u8]>`],
/// to allow them to implement [`Read`] and/or [`Write`], allowing these buffers to be used
/// anywhere you might use a reader or writer that does actual I/O.
///
/// The standard library implements some I/O traits on various types which
/// are commonly used as a buffer, like `Cursor<Vec<u8>>` and `Cursor<&[u8]>`.
///
/// This is the `embedded-io` equivalent of [`std::io::Cursor`].
///
/// # Examples
///
/// We may want to write bytes to a [`Write`], but not consume the buffer:
///
/// ```rust
/// use embedded_io::{Write, Cursor};
///
/// let mut buf = [0u8; 10];
/// let mut cursor = Cursor::new(&mut buf[..]);
///
/// cursor.write_all(b"some bytes").unwrap();
/// // Internal buffer is not at capacity so writing more bytes fails.
/// cursor.write_all(b"some more bytes").unwrap_err();
///
/// assert_eq!(cursor.into_inner(), b"some bytes");
/// ```
///
/// [`std::io::Cursor`]: https://doc.rust-lang.org/std/io/struct.Cursor.html
#[derive(Debug, Default, Eq, PartialEq)]
pub struct Cursor<T> {
    inner: T,
    pos: u64,
}

impl<T> Cursor<T> {
    /// Creates a new cursor wrapping the provided underlying in-memory buffer.
    ///
    /// Cursor initial position is `0` even if the underlying buffer is not empty.
    #[inline]
    pub const fn new(inner: T) -> Cursor<T> {
        Cursor { inner, pos: 0 }
    }

    /// Consumes this cursor, returning the underlying value.
    #[inline]
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Gets a reference to the underlying value in this cursor.
    #[inline]
    pub const fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Gets a mutable reference to the underlying value in this cursor.
    ///
    /// Care should be taken to avoid modifying the internal I/O state of the
    /// underlying value as it may corrupt this cursor's position.
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Returns the current position of this cursor.
    #[inline]
    pub const fn position(&self) -> u64 {
        self.pos
    }

    /// Sets the position of this cursor.
    #[inline]
    pub fn set_position(&mut self, pos: u64) {
        self.pos = pos;
    }
}

impl<T> Cursor<T>
where
    T: AsRef<[u8]>,
{
    /// Returns the remaining slice.
    #[inline]
    pub fn remaining_slice(&self) -> &[u8] {
        let len = self.inner.as_ref().len();
        let pos = cmp::min(self.pos, len as u64) as usize;
        &self.inner.as_ref()[pos..]
    }

    /// Returns `true` if the remaining slice is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.remaining_slice().is_empty()
    }
}

impl<T> Clone for Cursor<T>
where
    T: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Cursor {
            inner: self.inner.clone(),
            pos: self.pos,
        }
    }

    #[inline]
    fn clone_from(&mut self, other: &Self) {
        self.inner.clone_from(&other.inner);
        self.pos = other.pos;
    }
}

impl<T> ErrorType for Cursor<T> {
    type Error = CursorError;
}

/// Errors that could be returned by `Cursor`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum CursorError {
    /// An invalid seek was attempted (e.g., seeking to a negative position).
    InvalidSeek,
    /// The cursor's buffer is full and cannot accept more data.
    Full,
}

impl crate::Error for CursorError {
    fn kind(&self) -> ErrorKind {
        match self {
            CursorError::InvalidSeek => ErrorKind::InvalidInput,
            CursorError::Full => ErrorKind::WriteZero,
        }
    }
}

impl core::fmt::Display for CursorError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CursorError::InvalidSeek => write!(f, "invalid seek to a negative position"),
            CursorError::Full => write!(f, "cursor buffer is full"),
        }
    }
}

impl core::error::Error for CursorError {}

impl<T> Read for Cursor<T>
where
    T: AsRef<[u8]>,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let slice = self.remaining_slice();
        let amt = cmp::min(buf.len(), slice.len());

        if amt == 1 {
            buf[0] = slice[0];
        } else {
            buf[..amt].copy_from_slice(&slice[..amt]);
        }

        self.pos += amt as u64;
        Ok(amt)
    }
}

impl<T> BufRead for Cursor<T>
where
    T: AsRef<[u8]>,
{
    fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
        Ok(self.remaining_slice())
    }

    fn consume(&mut self, amt: usize) {
        self.pos += amt as u64;
    }
}

impl<T> Seek for Cursor<T>
where
    T: AsRef<[u8]>,
{
    fn seek(&mut self, style: SeekFrom) -> Result<u64, Self::Error> {
        let (base_pos, offset) = match style {
            SeekFrom::Start(n) => {
                self.pos = n;
                return Ok(n);
            }
            SeekFrom::End(n) => (self.inner.as_ref().len() as u64, n),
            SeekFrom::Current(n) => (self.pos, n),
        };

        let Some(new_pos) = base_pos.checked_add_signed(offset) else {
            return Err(CursorError::InvalidSeek);
        };

        self.pos = new_pos;
        Ok(new_pos)
    }
}

impl<T> ReadReady for Cursor<T>
where
    T: AsRef<[u8]>,
{
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

// Write implementation for Cursor<&mut [u8]>.
impl Write for Cursor<&mut [u8]> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let pos = cmp::min(self.pos, self.inner.len() as u64) as usize;
        let remaining = &mut self.inner[pos..];
        let len = cmp::min(buf.len(), remaining.len());

        if !buf.is_empty() && len == 0 {
            return Err(CursorError::Full);
        }

        remaining[..len].copy_from_slice(&buf[..len]);
        self.pos += len as u64;
        Ok(len)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl WriteReady for Cursor<&mut [u8]> {
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

#[cfg(feature = "alloc")]
mod alloc_impl {
    use alloc::vec::Vec;

    use crate::{Write, WriteReady};

    use super::{cmp, Cursor};

    // Write implementation for Cursor<Vec<u8>>.
    #[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
    impl Write for Cursor<Vec<u8>> {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
            let pos = cmp::min(self.pos, self.inner.len() as u64) as usize;

            // If position is at the end, just append.
            if pos == self.inner.len() {
                self.inner.extend_from_slice(buf);
            } else {
                // Overwrite existing bytes first, then extend if needed.
                let overlap = cmp::min(buf.len(), self.inner.len() - pos);
                self.inner[pos..pos + overlap].copy_from_slice(&buf[..overlap]);

                if buf.len() > overlap {
                    self.inner.extend_from_slice(&buf[overlap..]);
                }
            }

            self.pos += buf.len() as u64;
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
    impl WriteReady for Cursor<Vec<u8>> {
        fn write_ready(&mut self) -> Result<bool, Self::Error> {
            Ok(true)
        }
    }

    // Write implementation for Cursor<&mut Vec<u8>>.
    #[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
    impl Write for Cursor<&mut Vec<u8>> {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
            let pos = cmp::min(self.pos, self.inner.len() as u64) as usize;

            // If position is at the end, just append.
            if pos == self.inner.len() {
                self.inner.extend_from_slice(buf);
            } else {
                // Overwrite existing bytes first, then extend if needed.
                let overlap = cmp::min(buf.len(), self.inner.len() - pos);
                self.inner[pos..pos + overlap].copy_from_slice(&buf[..overlap]);

                if buf.len() > overlap {
                    self.inner.extend_from_slice(&buf[overlap..]);
                }
            }

            self.pos += buf.len() as u64;
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
    impl WriteReady for Cursor<&mut Vec<u8>> {
        fn write_ready(&mut self) -> Result<bool, Self::Error> {
            Ok(true)
        }
    }
}

#[cfg(feature = "alloc")]
mod box_impl {
    use alloc::boxed::Box;

    use crate::{Write, WriteReady};

    use super::{cmp, Cursor, CursorError};

    // Write implementation for Cursor<Box<[u8]>>.
    #[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
    impl Write for Cursor<Box<[u8]>> {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
            let pos = cmp::min(self.pos, self.inner.len() as u64) as usize;
            let remaining = &mut self.inner[pos..];
            let amt = cmp::min(buf.len(), remaining.len());

            if !buf.is_empty() && amt == 0 {
                return Err(CursorError::Full);
            }

            remaining[..amt].copy_from_slice(&buf[..amt]);
            self.pos += amt as u64;
            Ok(amt)
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
    impl WriteReady for Cursor<Box<[u8]>> {
        fn write_ready(&mut self) -> Result<bool, Self::Error> {
            Ok(true)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_and_position() {
        let cursor = Cursor::new([1, 2, 3]);
        assert_eq!(cursor.position(), 0);
        assert_eq!(cursor.get_ref(), &[1, 2, 3]);
    }

    #[test]
    fn set_position() {
        let mut cursor = Cursor::new([1, 2, 3]);
        cursor.set_position(2);
        assert_eq!(cursor.position(), 2);
    }

    #[test]
    fn into_inner() {
        let cursor = Cursor::new([1, 2, 3]);
        assert_eq!(cursor.into_inner(), [1, 2, 3]);
    }

    #[test]
    fn get_mut() {
        let mut cursor = Cursor::new([1, 2, 3]);
        cursor.get_mut()[0] = 10;
        assert_eq!(cursor.get_ref(), &[10, 2, 3]);
    }

    #[test]
    fn remaining_slice() {
        let mut cursor = Cursor::new([1, 2, 3, 4, 5]);
        assert_eq!(cursor.remaining_slice(), &[1, 2, 3, 4, 5]);
        cursor.set_position(2);
        assert_eq!(cursor.remaining_slice(), &[3, 4, 5]);
        cursor.set_position(5);
        assert_eq!(cursor.remaining_slice(), &[]);
        // Position beyond end.
        cursor.set_position(100);
        assert_eq!(cursor.remaining_slice(), &[]);
    }

    #[test]
    fn is_empty() {
        let mut cursor = Cursor::new([1, 2, 3]);
        assert!(!cursor.is_empty());
        cursor.set_position(3);
        assert!(cursor.is_empty());
    }

    #[test]
    fn read_basic() {
        let mut cursor = Cursor::new([1, 2, 3, 4, 5]);
        let mut buf = [0; 3];
        assert_eq!(cursor.read(&mut buf).unwrap(), 3);
        assert_eq!(buf, [1, 2, 3]);
        assert_eq!(cursor.position(), 3);

        assert_eq!(cursor.read(&mut buf).unwrap(), 2);
        assert_eq!(buf[..2], [4, 5]);
        assert_eq!(cursor.position(), 5);

        // Reading at end returns 0.
        assert_eq!(cursor.read(&mut buf).unwrap(), 0);
    }

    #[test]
    fn read_single_byte() {
        let mut cursor = Cursor::new([42]);
        let mut buf = [0; 1];
        assert_eq!(cursor.read(&mut buf).unwrap(), 1);
        assert_eq!(buf[0], 42);
    }

    #[test]
    fn buf_read() {
        let mut cursor = Cursor::new([1, 2, 3, 4, 5]);
        assert_eq!(cursor.fill_buf().unwrap(), &[1, 2, 3, 4, 5]);
        cursor.consume(2);
        assert_eq!(cursor.position(), 2);
        assert_eq!(cursor.fill_buf().unwrap(), &[3, 4, 5]);
    }

    #[test]
    fn seek_start() {
        let mut cursor = Cursor::new([1, 2, 3, 4, 5]);
        assert_eq!(cursor.seek(SeekFrom::Start(3)).unwrap(), 3);
        assert_eq!(cursor.position(), 3);
    }

    #[test]
    fn seek_end() {
        let mut cursor = Cursor::new([1, 2, 3, 4, 5]);
        assert_eq!(cursor.seek(SeekFrom::End(-2)).unwrap(), 3);
        assert_eq!(cursor.position(), 3);

        assert_eq!(cursor.seek(SeekFrom::End(0)).unwrap(), 5);
        assert_eq!(cursor.position(), 5);
    }

    #[test]
    fn seek_current() {
        let mut cursor = Cursor::new([1, 2, 3, 4, 5]);
        cursor.set_position(2);
        assert_eq!(cursor.seek(SeekFrom::Current(2)).unwrap(), 4);
        assert_eq!(cursor.position(), 4);

        assert_eq!(cursor.seek(SeekFrom::Current(-1)).unwrap(), 3);
        assert_eq!(cursor.position(), 3);
    }

    #[test]
    fn seek_invalid() {
        let mut cursor = Cursor::new([1, 2, 3, 4, 5]);
        // Seeking to negative position.
        assert_eq!(
            cursor.seek(SeekFrom::End(-10)).unwrap_err(),
            CursorError::InvalidSeek
        );
        assert_eq!(
            cursor.seek(SeekFrom::Current(-1)).unwrap_err(),
            CursorError::InvalidSeek
        );
    }

    #[test]
    fn write_to_slice() {
        let mut buf = [0u8; 5];
        let mut cursor = Cursor::new(&mut buf[..]);
        assert_eq!(cursor.write(&[1, 2, 3]).unwrap(), 3);
        assert_eq!(cursor.position(), 3);
        assert_eq!(cursor.write(&[4, 5]).unwrap(), 2);
        assert_eq!(cursor.position(), 5);
        assert_eq!(buf, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn write_slice_full() {
        let mut buf = [0u8; 3];
        let mut cursor = Cursor::new(&mut buf[..]);
        cursor.write_all(&[1, 2, 3]).unwrap();
        assert_eq!(cursor.write(&[4]).unwrap_err(), CursorError::Full);
    }

    #[test]
    fn read_ready() {
        let mut cursor = Cursor::new([1, 2, 3]);
        assert!(cursor.read_ready().unwrap());
    }

    #[test]
    fn write_ready_slice() {
        let mut buf = [0u8; 3];
        let mut cursor = Cursor::new(&mut buf[..]);
        assert!(cursor.write_ready().unwrap());
    }

    #[test]
    fn flush() {
        let mut buf = [0u8; 3];
        let mut cursor = Cursor::new(&mut buf[..]);
        assert!(cursor.flush().is_ok());
    }

    #[test]
    fn clone() {
        let cursor = Cursor::new([1, 2, 3]);
        let cloned = cursor.clone();
        assert_eq!(cursor.get_ref(), cloned.get_ref());
        assert_eq!(cursor.position(), cloned.position());
    }

    #[test]
    fn default() {
        let cursor: Cursor<[u8; 0]> = Cursor::default();
        assert_eq!(cursor.position(), 0);
        assert_eq!(cursor.get_ref(), &[]);
    }

    #[test]
    fn error_kind() {
        use crate::Error;
        assert_eq!(CursorError::InvalidSeek.kind(), ErrorKind::InvalidInput);
        assert_eq!(CursorError::Full.kind(), ErrorKind::WriteZero);
    }

    #[cfg(feature = "alloc")]
    mod alloc_tests {
        use alloc::boxed::Box;
        use alloc::vec;
        use alloc::vec::Vec;

        use super::*;

        #[test]
        fn write_to_vec() {
            let mut cursor = Cursor::new(Vec::new());
            assert_eq!(cursor.write(&[1, 2, 3]).unwrap(), 3);
            assert_eq!(cursor.write(&[4, 5]).unwrap(), 2);
            assert_eq!(cursor.into_inner(), vec![1, 2, 3, 4, 5]);
        }

        #[test]
        fn write_to_vec_with_seek() {
            let mut cursor = Cursor::new(vec![0, 0, 0, 0, 0]);
            cursor.set_position(2);
            assert_eq!(cursor.write(&[1, 2, 3]).unwrap(), 3);
            assert_eq!(cursor.into_inner(), vec![0, 0, 1, 2, 3]);
        }

        #[test]
        fn write_to_vec_extend() {
            let mut cursor = Cursor::new(vec![1, 2]);
            cursor.set_position(1);
            // Overwrite one byte and extend.
            assert_eq!(cursor.write(&[10, 20, 30]).unwrap(), 3);
            assert_eq!(cursor.into_inner(), vec![1, 10, 20, 30]);
        }

        #[test]
        fn write_to_mut_vec() {
            let mut vec = Vec::new();
            let mut cursor = Cursor::new(&mut vec);
            cursor.write_all(&[1, 2, 3]).unwrap();
            drop(cursor);
            assert_eq!(vec, vec![1, 2, 3]);
        }

        #[test]
        fn write_to_boxed_slice() {
            let mut cursor = Cursor::new(vec![0u8; 5].into_boxed_slice());
            assert_eq!(cursor.write(&[1, 2, 3]).unwrap(), 3);
            assert_eq!(cursor.position(), 3);
            assert_eq!(&*cursor.into_inner(), &[1, 2, 3, 0, 0]);
        }

        #[test]
        fn write_boxed_slice_full() {
            let mut cursor = Cursor::new(vec![0u8; 2].into_boxed_slice());
            cursor.write_all(&[1, 2]).unwrap();
            assert_eq!(cursor.write(&[3]).unwrap_err(), CursorError::Full);
        }

        #[test]
        fn write_ready_vec() {
            let mut cursor = Cursor::new(Vec::new());
            assert!(cursor.write_ready().unwrap());
        }

        #[test]
        fn write_ready_boxed_slice() {
            let mut cursor = Cursor::new(Box::<[u8]>::from(vec![0u8; 3]));
            assert!(cursor.write_ready().unwrap());
        }

        #[test]
        fn read_from_vec() {
            let mut cursor = Cursor::new(vec![1, 2, 3]);
            let mut buf = [0; 2];
            assert_eq!(cursor.read(&mut buf).unwrap(), 2);
            assert_eq!(buf, [1, 2]);
        }

        #[test]
        fn read_from_boxed_slice() {
            let mut cursor = Cursor::new(vec![1, 2, 3].into_boxed_slice());
            let mut buf = [0; 2];
            assert_eq!(cursor.read(&mut buf).unwrap(), 2);
            assert_eq!(buf, [1, 2]);
        }

        #[test]
        fn error_display() {
            assert!(!CursorError::InvalidSeek.to_string().is_empty());
            assert!(!CursorError::Full.to_string().is_empty());
        }
    }
}
