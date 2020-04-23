//! Defines futures for complete write operations.
use core::future;
use core::mem;
use core::pin;
use core::task;

/// A future that ensures that a complete write operation is performed.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct WriteAll<'a, A: ?Sized> {
    writer: &'a mut A,
    buffer: &'a [u8],
}

/// Creates a new [`WriteAll`] for the provided writer.
pub fn write_all<'a, A>(writer: &'a mut A, buffer: &'a [u8]) -> WriteAll<'a, A>
where
    A: super::Write + Unpin + ?Sized,
{
    WriteAll { writer, buffer }
}

impl<A> future::Future for WriteAll<'_, A>
where
    A: super::Write + Unpin + ?Sized,
{
    type Output = Result<(), A::Error>;

    fn poll(mut self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        use super::Write;
        use super::WriteError;

        let this = &mut *self;
        while !this.buffer.is_empty() {
            let n = futures::ready!(pin::Pin::new(&mut this.writer).poll_write(cx, this.buffer))?;
            {
                let (_, rest) = mem::replace(&mut this.buffer, &[]).split_at(n);
                this.buffer = rest;
            }
            if n == 0 {
                return task::Poll::Ready(Err(A::Error::write_zero()));
            }
        }

        task::Poll::Ready(Ok(()))
    }
}
