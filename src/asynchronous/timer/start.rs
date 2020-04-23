//! Defines futures for starting a timer.
use core::future;
use core::pin;
use core::task;

/// A future which starts a timer.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Start<'a, A>
where
    A: super::Timer + Unpin + ?Sized,
{
    timer: &'a mut A,
}

/// Creates a new [`Start`] for the provided timer.
pub fn start<A>(timer: &mut A) -> Start<A>
where
    A: super::Timer + Unpin + ?Sized,
{
    Start { timer }
}

impl<A> future::Future for Start<'_, A>
where
    A: super::Timer + Unpin + ?Sized,
{
    type Output = Result<(), A::Error>;

    fn poll(mut self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        pin::Pin::new(&mut *this.timer).poll_start(cx)
    }
}
