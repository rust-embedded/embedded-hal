//! Defines futures for initiating reads from an I²C peripheral.
use core::future;
use core::pin;
use core::task;

/// A future which initializes reads from an I²C peripheral.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct BeginRead<'a, A>
where
    A: super::I2cRead + Unpin + ?Sized,
{
    reader: &'a mut A,
    address: u8,
}

/// Creates a new [`BeginRead`] for the provided I²C peripheral.
///
/// The read will access the specified address.
pub fn begin_read<A>(reader: &mut A, address: u8) -> BeginRead<A>
where
    A: super::I2cRead + Unpin + ?Sized,
{
    BeginRead { reader, address }
}

impl<A> future::Future for BeginRead<'_, A>
where
    A: super::I2cRead + Unpin + ?Sized,
{
    type Output = Result<A::Read, A::Error>;

    fn poll(mut self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        pin::Pin::new(&mut *this.reader).poll_begin_read(cx, this.address)
    }
}
