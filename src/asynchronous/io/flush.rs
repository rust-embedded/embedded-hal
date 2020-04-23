//! Defines futures for flushing write operations.
use core::future;
use core::pin;
use core::task;

/// A future that ensures that a write operation is flushed.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Flush<'a, A: ?Sized> {
    writer: &'a mut A,
}

/// Creates a new [`Flush`] for the provided writer.
pub fn flush<A>(writer: &mut A) -> Flush<A>
where
    A: super::Write + Unpin + ?Sized,
{
    Flush { writer }
}

impl<A> future::Future for Flush<'_, A>
where
    A: super::Write + Unpin + ?Sized,
{
    type Output = Result<(), A::Error>;

    fn poll(mut self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        pin::Pin::new(&mut *this.writer).poll_flush(cx)
    }
}
