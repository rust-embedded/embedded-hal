//! Defines futures for exact read operations.
use core::future;
use core::pin;
use core::task;

/// A future that ensures that an exact read operation is performed.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct ReadExact<'a, A>
where
    A: super::Read + Unpin + ?Sized,
{
    reader: &'a mut A,
    buffer: &'a mut [u8],
    position: usize,
}

/// Creates a new [`ReadExact`] for the provided reader.
pub fn read_exact<'a, A>(reader: &'a mut A, buffer: &'a mut [u8]) -> ReadExact<'a, A>
where
    A: super::Read + Unpin + ?Sized,
{
    let position = 0;
    ReadExact {
        reader,
        buffer,
        position,
    }
}

impl<A> future::Future for ReadExact<'_, A>
where
    A: super::Read + Unpin + ?Sized,
{
    type Output = Result<usize, A::Error>;

    fn poll(mut self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        use super::ReadError;

        loop {
            // if our buffer is empty, then we need to read some data to continue.
            if self.position < self.buffer.len() {
                let this = &mut *self;
                let n = futures::ready!(pin::Pin::new(&mut *this.reader)
                    .poll_read(cx, &mut this.buffer[this.position..]))?;
                this.position += n;
                if n == 0 {
                    return Err(A::Error::eof()).into();
                }
            }

            if self.position >= self.buffer.len() {
                return task::Poll::Ready(Ok(self.position));
            }
        }
    }
}
