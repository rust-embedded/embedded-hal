//! Defines futures for initiating a SPI transaction.
use core::future;
use core::pin;
use core::task;

/// A future which begins a SPI transaction.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct BeginTransaction<'a, A>
where
    A: super::Spi + Unpin + ?Sized,
{
    spi: &'a mut A,
}

/// Creates a new [`BeginTransaction`] for the provided SPI instance.
pub fn begin_transaction<A>(spi: &mut A) -> BeginTransaction<A>
where
    A: super::Spi + Unpin + ?Sized,
{
    BeginTransaction { spi }
}

impl<A> future::Future for BeginTransaction<'_, A>
where
    A: super::Spi + Unpin + ?Sized,
{
    type Output = Result<A::Transaction, A::Error>;

    fn poll(mut self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        pin::Pin::new(&mut *this.spi).poll_begin_transaction(cx)
    }
}
