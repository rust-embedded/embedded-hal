//! Defines futures for completing a SPI transfer.
use core::future;
use core::pin;
use core::task;

/// A future which completes a SPI transfer.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Complete<'a, A: ?Sized> {
    transfer: &'a mut A,
}

/// Creates a new [`Complete`] for the provided SPI transfer.
pub fn complete<A>(transfer: &mut A) -> Complete<A>
where
    A: super::SpiTransfer + Unpin + ?Sized,
{
    Complete { transfer }
}

impl<'a, A> future::Future for Complete<'a, A>
where
    A: super::SpiTransfer + Unpin + ?Sized,
{
    type Output = Result<(), A::Error>;

    fn poll(mut self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        pin::Pin::new(&mut *this.transfer).poll_complete(cx)
    }
}
