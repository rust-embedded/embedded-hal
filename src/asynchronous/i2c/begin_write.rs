//! Defines futures for initiating writes to an I²C peripheral.
use core::future;
use core::pin;
use core::task;

/// A future which initializes writes to an I²C peripheral.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct BeginWrite<'a, A>
where
    A: super::I2cWrite + Unpin + ?Sized,
{
    writer: &'a mut A,
    address: u8,
}

/// Creates a new [`BeginWrite`] for the provided I²C peripheral.
///
/// The write will access the specified address.
pub fn begin_write<A>(writer: &mut A, address: u8) -> BeginWrite<A>
where
    A: super::I2cWrite + Unpin + ?Sized,
{
    BeginWrite { writer, address }
}

impl<A> future::Future for BeginWrite<'_, A>
where
    A: super::I2cWrite + Unpin + ?Sized,
{
    type Output = Result<A::Write, A::Error>;

    fn poll(mut self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        pin::Pin::new(&mut *this.writer).poll_begin_write(cx, this.address)
    }
}
