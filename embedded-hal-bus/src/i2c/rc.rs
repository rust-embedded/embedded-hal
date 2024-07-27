extern crate alloc;
use alloc::rc::Rc;

use core::cell::RefCell;
use embedded_hal::i2c::{ErrorType, I2c};

/// `Rc<RefCell<T>>`-based shared bus [`I2c`] implementation.
/// This is the reference-counting equivalent of [`RefCellDevice`](super::RefCellDevice).
///
/// Sharing is implemented with a [`RefCell`] and ownership is managed by [`Rc`].
/// Like [`RefCellDevice`](super::RefCellDevice), `RcDevice` instances are not [`Send`],
/// so they can only be shared within a single thread (interrupt priority level).
///
/// When this `RcDevice` is dropped, the reference count of the I2C bus will be decremented.
/// Once that reference count hits zero, it will be cleaned up.
#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
pub struct RcDevice<Bus> {
    bus: Rc<RefCell<Bus>>,
}

impl<Bus> RcDevice<Bus> {
    /// Creates a new `RcDevice`.
    ///
    /// This function does not increment the reference count for the bus:
    /// you will need to call `Rc::clone(&bus)` if you only have a `&Rc<RefCell<Bus>>`.
    #[inline]
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        Self { bus }
    }
}

impl<Bus> ErrorType for RcDevice<Bus>
where
    Bus: ErrorType,
{
    type Error = Bus::Error;
}

impl<Bus> I2c for RcDevice<Bus>
where
    Bus: I2c,
{
    #[inline]
    fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.borrow_mut();
        bus.read(address, read)
    }

    #[inline]
    fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.borrow_mut();
        bus.write(address, write)
    }

    #[inline]
    fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.borrow_mut();
        bus.write_read(address, write, read)
    }

    #[inline]
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.borrow_mut();
        bus.transaction(address, operations)
    }
}
