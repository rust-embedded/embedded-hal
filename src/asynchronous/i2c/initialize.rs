//! Defines futures for initializing an I²C peripheral based off of GPIO pins.
use core::future;
use core::pin;
use core::task;

/// A future which initializes an I²C peripheral based off of GPIO pins.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Initialize<A, SDA, SCL>
where
    A: super::I2cBusMapping<SDA, SCL> + Unpin,
    SDA: Unpin,
    SCL: Unpin,
{
    mapping: A,
    sda: SDA,
    scl: SCL,
}

/// Creates a new [`Initialize`] based off of a I²C bus pin mapping, as well as an SDA and SCL pin.
pub fn initialize<A, SDA, SCL>(mapping: A, sda: SDA, scl: SCL) -> Initialize<A, SDA, SCL>
where
    A: super::I2cBusMapping<SDA, SCL> + Unpin,
    SDA: Unpin,
    SCL: Unpin,
{
    Initialize { mapping, sda, scl }
}

impl<A, SDA, SCL> future::Future for Initialize<A, SDA, SCL>
where
    A: super::I2cBusMapping<SDA, SCL> + Unpin,
    SDA: Unpin,
    SCL: Unpin,
{
    type Output = Result<A::Bus, A::Error>;

    fn poll(mut self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        pin::Pin::new(&mut this.mapping).poll_initialize(cx, &mut this.sda, &mut this.scl)
    }
}
