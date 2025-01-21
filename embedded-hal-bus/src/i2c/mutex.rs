use embedded_hal::i2c::{ErrorType, I2c};
use std::sync::Mutex;

/// `std` `Mutex`-based shared bus [`I2c`] implementation.
///
/// Sharing is implemented with an `std` [`Mutex`]. It allows a single bus across multiple threads,
/// with finer-grained locking than [`CriticalSectionDevice`](super::CriticalSectionDevice). The downside is that
/// it is only available in `std` targets.
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub struct MutexDevice<'a, T> {
    bus: &'a Mutex<T>,
}

impl<'a, T> MutexDevice<'a, T> {
    /// Create a new `MutexDevice`.
    #[inline]
    pub fn new(bus: &'a Mutex<T>) -> Self {
        Self { bus }
    }
}

impl<T> ErrorType for MutexDevice<'_, T>
where
    T: I2c,
{
    type Error = T::Error;
}

impl<T> I2c for MutexDevice<'_, T>
where
    T: I2c,
{
    #[inline]
    fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.lock().unwrap();
        bus.read(address, read)
    }

    #[inline]
    fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.lock().unwrap();
        bus.write(address, write)
    }

    #[inline]
    fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.lock().unwrap();
        bus.write_read(address, write, read)
    }

    #[inline]
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.lock().unwrap();
        bus.transaction(address, operations)
    }
}
