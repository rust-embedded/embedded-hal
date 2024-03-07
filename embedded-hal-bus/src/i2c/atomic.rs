use embedded_hal::i2c::{Error, ErrorKind, ErrorType, I2c};
use core::cell::UnsafeCell;

/// `UnsafeCell`-based shared bus [`I2c`] implementation.
///
/// Sharing is implemented with a `UnsafeCell`. This means it has low overhead, similar to `RefCellDevice` instances, but they are `Send`.
/// so it only allows sharing across multiple threads (interrupt priority levels). When attempting
/// to preempt usage of the bus, a `AtomicError::Busy` error is returned.
///
/// This primitive is particularly well-suited for applications that have external arbitration
/// rules, such as the RTIC framework.
///
/// # Examples
///
/// Assuming there is a pressure sensor with address `0x42` on the same bus as a temperature sensor
/// with address `0x20`; [`RefCellDevice`] can be used to give access to both of these sensors
/// from a single `i2c` instance.
///
/// ```
/// use embedded_hal_bus::i2c;
/// use core::cell::UnsafeCell;
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
/// let i2c_unsafe_cell = UnsafeCell::new(i2c);
/// let mut temperature_sensor = TemperatureSensor::new(
///   i2c::AtomicDevice::new(&i2c_unsafe_cell),
///   0x20,
/// );
/// let mut pressure_sensor = PressureSensor::new(
///   i2c::AtomicDevice::new(&i2c_unsafe_cell),
///   0x42,
/// );
/// ```
pub struct AtomicDevice<'a, T> {
    bus: &'a UnsafeCell<T>,
    busy: portable_atomic::AtomicBool,
}


#[derive(Debug, Copy, Clone)]
/// Wrapper type for errors originating from the atomically-checked I2C bus manager.
pub enum AtomicError<T: ErrorType> {
    /// This error is returned if the I2C bus was already in use when an operation was attempted,
    /// which indicates that the driver requirements are not being met with regard to
    /// synchronization.
    Busy,

    /// An I2C-related error occurred, and the internal error should be inspected.
    Other(T::Error),
}

impl<T: ErrorType + core::fmt::Debug> Error for AtomicError<T> {
    fn kind(&self) -> ErrorKind {
        match self {
            AtomicError::Other(e) => e.kind(),
            _ => ErrorKind::Other,
        }
    }
}

unsafe impl<'a, T> Sync for AtomicDevice<'a, T> {}

impl<'a, T> AtomicDevice<'a, T>
where T: I2c + ErrorType
{
    /// Create a new `AtomicDevice`.
    #[inline]
    pub fn new(bus: &'a UnsafeCell<T>) -> Self {
        Self {
            bus,
            busy: portable_atomic::AtomicBool::from(false),
        }
    }

    fn lock<R, F>(&self, f: F) -> Result<R, AtomicError<T>>
    where
    F: FnOnce(&mut T) -> Result<R, <T as ErrorType>::Error>
    {
        self.busy.compare_exchange(
            false,
            true,
            core::sync::atomic::Ordering::SeqCst,
            core::sync::atomic::Ordering::SeqCst,
            ).map_err(|_| AtomicError::<T>::Busy)?;

        let result = f(unsafe { &mut *self.bus.get() });

        self.busy.store(false, core::sync::atomic::Ordering::SeqCst);

        result.map_err(|e| AtomicError::Other(e))
    }
}

impl<'a, T> ErrorType for AtomicDevice<'a, T>
where
    T: I2c + core::fmt::Debug,
{
    type Error = AtomicError<T>;
}

impl<'a, T> I2c for AtomicDevice<'a, T>
where
    T: I2c + core::fmt::Debug,
{
    #[inline]
    fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        self.lock(|bus| bus.read(address, read))
    }

    #[inline]
    fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
        self.lock(|bus| bus.write(address, write))
    }

    #[inline]
    fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.lock(|bus| bus.write_read(address, write, read))
    }

    #[inline]
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.lock(|bus| bus.transaction(address, operations))
    }
}
