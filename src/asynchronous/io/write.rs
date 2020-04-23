//! Defines futures for write operations.
use core::future;
use core::pin;
use core::task;

/// A future that ensures that a write operation is performed.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Write<'a, A: ?Sized> {
    writer: &'a mut A,
    buf: &'a [u8],
}

/// Creates a new [`Write`] for the provided writer.
pub fn write<'a, A>(writer: &'a mut A, buf: &'a [u8]) -> Write<'a, A>
where
    A: super::Write + Unpin + ?Sized,
{
    Write { writer, buf }
}

impl<A> future::Future for Write<'_, A>
where
    A: super::Write + Unpin + ?Sized,
{
    type Output = Result<usize, A::Error>;

    fn poll(mut self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        pin::Pin::new(&mut *this.writer).poll_write(cx, this.buf)
    }
}
