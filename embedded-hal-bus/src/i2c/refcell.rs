use core::cell::RefCell;
use embedded_hal::i2c::{ErrorType, I2c};

/// `RefCell`-based shared bus [`I2c`] implementation.
///
/// Sharing is implemented with a `RefCell`. This means it has low overhead, but `RefCellDevice` instances are not `Send`,
/// so it only allows sharing within a single thread (interrupt priority level). If you need to share a bus across several
/// threads, use [`CriticalSectionDevice`](super::CriticalSectionDevice) instead.
///
/// # Examples
///
/// Assuming there is a pressure sensor with address `0x42` on the same bus as a temperature sensor
/// with address `0x20`; [`RefCellDevice`] can be used to give access to both of these sensors
/// from a single `i2c` instance.
///
/// ```
/// use embedded_hal_bus::i2c;
/// use core::cell::RefCell;
/// # use embedded_hal::i2c::{self as hali2c, SevenBitAddress, TenBitAddress, I2c, Operation, ErrorKind};
/// # pub struct Sensor<I2C> {
/// #     i2c: I2C,
/// #     address: u8,
/// # }
/// # impl<I2C: I2c> Sensor<I2C> {
/// #     pub fn new(i2c: I2C, address: u8) -> Self {
/// #         Self { i2c, address }
/// #     }
/// # }
/// # type PressureSensor<I2C> = Sensor<I2C>;
/// # type TemperatureSensor<I2C> = Sensor<I2C>;
/// # pub struct I2c0;
/// # #[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// # pub enum Error { }
/// # impl hali2c::Error for Error {
/// #     fn kind(&self) -> hali2c::ErrorKind {
/// #         ErrorKind::Other
/// #     }
/// # }
/// # impl hali2c::ErrorType for I2c0 {
/// #     type Error = Error;
/// # }
/// # impl I2c<SevenBitAddress> for I2c0 {
/// #     fn transaction(&mut self, address: u8, operations: &mut [Operation<'_>]) -> Result<(), Self::Error> {
/// #       Ok(())
/// #     }
/// # }
/// # struct Hal;
/// # impl Hal {
/// #   fn i2c(&self) -> I2c0 {
/// #     I2c0
/// #   }
/// # }
/// # let hal = Hal;
///
/// let i2c = hal.i2c();
/// let i2c_ref_cell = RefCell::new(i2c);
/// let mut temperature_sensor = TemperatureSensor::new(
///   i2c::RefCellDevice::new(&i2c_ref_cell),
///   0x20,
/// );
/// let mut pressure_sensor = PressureSensor::new(
///   i2c::RefCellDevice::new(&i2c_ref_cell),
///   0x42,
/// );
/// ```
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
