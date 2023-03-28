use core::cell::RefCell;
use embedded_hal::i2c::{ErrorType, I2c};

/// `RefCell`-based shared bus [`I2c`] implementation.
///
/// Sharing is implemented with a `RefCell`. This means it has low overhead, but `RefCellDevice` instances are not `Send`,
/// so it only allows sharing within a single thread (interrupt priority level). If you need to share a bus across several
/// threads, use [`CriticalSectionDevice`](super::CriticalSectionDevice) instead.
pub struct RefCellDevice<'a, T> {
    bus: &'a RefCell<T>,
}

impl<'a, T> RefCellDevice<'a, T> {
    /// Create a new `RefCellDevice`
    pub fn new(bus: &'a RefCell<T>) -> Self {
        Self { bus }
    }
}

impl<'a, T> ErrorType for RefCellDevice<'a, T>
where
    T: I2c,
{
    type Error = T::Error;
}

impl<'a, T> I2c for RefCellDevice<'a, T>
where
    T: I2c,
{
    fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.borrow_mut();
        bus.read(address, read)
    }

    fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.borrow_mut();
        bus.write(address, write)
    }

    fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.borrow_mut();
        bus.write_read(address, write, read)
    }

    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.borrow_mut();
        bus.transaction(address, operations)
    }
}
