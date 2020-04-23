//! Defines futures for setting the value of a GPIO pin.
use core::future;
use core::pin;
use core::task;

/// A future which sets the value of a GPIO pin.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Set<'a, A>
where
    A: super::OutputPin + Unpin + ?Sized,
{
    pin: &'a mut A,
    high: bool,
}

/// Creates a new [`Set`] for the provided GPIO pin, that, when polled, will drive it to the
/// specified high or low value.
pub fn set<A>(pin: &mut A, high: bool) -> Set<A>
where
    A: super::OutputPin + Unpin + ?Sized,
{
    Set { pin, high }
}

impl<A> future::Future for Set<'_, A>
where
    A: super::OutputPin + Unpin + ?Sized,
{
    type Output = Result<(), A::Error>;

    fn poll(mut self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        pin::Pin::new(&mut *this.pin).poll_set(cx, this.high)
    }
}
