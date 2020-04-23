//! Defines futures for read operations.
use core::future;
use core::pin;
use core::task;

/// A future that ensures that a read operation is performed.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Read<'a, A>
where
    A: super::Read + Unpin + ?Sized,
{
    reader: &'a mut A,
    buffer: &'a mut [u8],
    position: usize,
}

/// Creates a new [`Read`] for the provided reader.
pub fn read<'a, A>(reader: &'a mut A, buffer: &'a mut [u8]) -> Read<'a, A>
where
    A: super::Read + Unpin + ?Sized,
{
    let position = 0;
    Read {
        reader,
        buffer,
        position,
    }
}

impl<A> future::Future for Read<'_, A>
where
    A: super::Read + Unpin + ?Sized,
{
    type Output = Result<usize, A::Error>;

    fn poll(mut self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        pin::Pin::new(&mut *this.reader).poll_read(cx, this.buffer)
    }
}
