//! Defines futures for getting the value off of a GPIO pin.
use core::future;
use core::pin;
use core::task;

/// A future which reads the value off a GPIO pin.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Get<'a, A>
where
    A: super::InputPin + Unpin + ?Sized,
{
    pin: &'a mut A,
}

/// Creates a new [`Get`] for the provided GPIO pin.
pub fn get<A>(pin: &mut A) -> Get<A>
where
    A: super::InputPin + Unpin + ?Sized,
{
    Get { pin }
}

impl<A> future::Future for Get<'_, A>
where
    A: super::InputPin + Unpin + ?Sized,
{
    type Output = Result<bool, A::Error>;

    fn poll(mut self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        pin::Pin::new(&mut *this.pin).poll_get(cx)
    }
}
