use embedded_hal::i2c::{ErrorType, I2c};
use mutex::{BlockingMutex, RawMutex};

type Mutex<R, T> = BlockingMutex<R, T>;

/// `mutex-traits`-based shared bus [`I2c`] implementation.
///
/// Whether a single bus can be used across multiple threads depends on which
/// implementation of `RawMutex` is used.
pub struct MutexTraitsDevice<'a, R, T> {
    bus: &'a Mutex<R, T>,
}

impl<'a, R: RawMutex, T> MutexTraitsDevice<'a, R, T> {
    /// Create a new `MutexTraitsDevice`.
    #[inline]
    pub fn new(bus: &'a Mutex<R, T>) -> Self {
        Self { bus }
    }
}

impl<R, T> ErrorType for MutexTraitsDevice<'_, R, T>
where
    T: I2c,
{
    type Error = T::Error;
}

impl<R: RawMutex, T> I2c for MutexTraitsDevice<'_, R, T>
where
    T: I2c,
{
    #[inline]
    fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.lock();
        bus.read(address, read)
    }

    #[inline]
    fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.lock();
        bus.write(address, write)
    }

    #[inline]
    fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.lock();
        bus.write_read(address, write, read)
    }

    #[inline]
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.lock();
        bus.transaction(address, operations)
    }
}
