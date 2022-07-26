//! Blocking I2C API
//!
//! # Bus vs Device
//!
//! I2C allows sharing a single bus between many I2C devices. The SDA and SCL lines are
//! wired in parallel to all the devices. When starting a transfer an "address" is sent
//! so that the addressed device can respond and all the others can ignore the transfer.
//!
#![doc= include_str!("i2c-shared-bus.svg")]
//!
//! This bus sharing is common when having multiple I2C devices in the same board, since it uses fewer MCU
//! pins (`2` instead of `2*n`), and fewer MCU I2C peripherals (`1` instead of `n`).
//!
//! However, it poses a challenge when building portable drivers for I2C devices. The driver needs to
//! be able to talk to its device on the bus, while not interfering with other drivers talking to other
//! devices.
//!
//! To solve this, `embedded-hal` has two kinds of I2C traits: **I2C bus** and **I2C device**.
//!
//! ## Bus
//!
//! I2C bus traits represent **exclusive ownership** over the whole I2C bus. This is usually the entire
//! I2C MCU peripheral, plus the SDA and SCL pins.
//!
//! Owning an instance of an I2C bus guarantees exclusive access, this is, we have the guarantee no other
//! piece of code will try to use the bus while we own it.
//!
//! ## Device
//!
//! [`I2cDevice`] represents **ownership over a single I2C device** in a (possibly shared) bus. This consists of
//! access to the **underlying I2C bus**. If shared, it'll be behind some kind of lock/mutex.
//!
//! An [`I2cDevice`] allows initiating [transactions](I2cDevice::transaction) against the target device on the bus. A transaction
//! consists of several read or write transfers, possibly with multiple start, stop or repeated start conditions.
//!
//! For the entire duration of the transaction, the [`I2cDevice`] implementation will ensure no other transaction
//! can be opened on the same bus. This is the key that allows correct sharing of the bus.
//!
//! # For driver authors
//!
//! When implementing a driver, use [`I2cDevice`], so that your driver cooperates nicely with other
//! drivers for other devices in the same shared I2C bus.
//!
//! ```
//! // TODO code snippet
//! ```
//!
//! # For HAL authors
//!
//! HALs **must** implement [`I2cBusBase`], and [`I2cBus`] for the supported address modes.
//!
//! There is little reason for HALs to implement [`I2cDevice`], this task is better left to the HAL-independent implementations).
//!
//! HALs **must not** add infrastructure for sharing at the [`I2cBus`] level. User code owning a [`I2cBus`] must have the guarantee
//! of exclusive access.
//!
//! # Flushing
//!
//! To improve performance, [`I2cBus`] implementations are allowed to return before the operation is finished, i.e. when the bus is still not
//! idle.
//!
//! When using a [`I2cBus`], call [`flush`](I2cBus::flush) to wait for operations to actually finish. Examples of situations
//! where this is needed are:
//! - To synchronize I2C activity and GPIO activity during a transaction.
//! - Before deinitializing the hardware I2C peripheral.
//!
//! When using a [`I2cDevice`], you can still call [`flush`](I2cBus::flush) on the bus within a transaction.
//! It's very rarely needed, because [`transaction`](I2cDevice::transaction) already flushes the bus for you
//! when finished.
//!
//! For example, for [`write`](I2cBusBase::write) operations, it is common for hardware I2C peripherals to have a small
//! FIFO buffer, usually 1-4 bytes. Software writes data to the FIFO, and the peripheral sends it on SDA at its own pace,
//! at the specified I2C frequency. It is allowed for an implementation of [`write`](I2cBusBase::write) to return as soon
//! as all the data has been written to the FIFO, before it is actually sent. Calling [`flush`](I2cBus::flush) would
//! wait until all the bits have actually been sent and the FIFO is empty.
//!
//! # Addresses
//!
//! This API supports 7-bit and 10-bit addresses. Traits feature an `AddressMode`
//! marker type parameter. Two implementation of the `AddressMode` exist:
//! `SevenBitAddress` and `TenBitAddress`.
//!
//! Through this marker types it is possible to implement each address mode for
//! the traits independently in `embedded-hal` implementations and device drivers
//! can depend only on the mode that they support.
//!
//! Additionally, the I2C 10-bit address mode has been developed to be fully
//! backwards compatible with the 7-bit address mode. This allows for a
//! software-emulated 10-bit addressing implementation if the address mode
//! is not supported by the hardware.
//!
//! Since 7-bit addressing is the mode of the majority of I2C devices,
//! `SevenBitAddress` has been set as default mode and thus can be omitted if desired.
//!
//! ## Examples
//!
//! ### `embedded-hal` implementation for an MCU
//! Here is an example of an embedded-hal implementation of the `Write` trait
//! for both modes:
//! ```
//! # use embedded_hal::i2c::{ErrorKind, ErrorType, SevenBitAddress, TenBitAddress, blocking::{I2c, Operation}};
//! /// I2C0 hardware peripheral which supports both 7-bit and 10-bit addressing.
//! pub struct I2c0;
//!
//! # impl ErrorType for I2c0 { type Error = ErrorKind; }
//! impl I2c<SevenBitAddress> for I2c0
//! {
//!     fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
//!         // ...
//! #       Ok(())
//!     }
//!     fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
//!         // ...
//! #       Ok(())
//!     }
//!     fn write_iter<B: IntoIterator<Item = u8>>(&mut self, addr: u8, bytes: B) -> Result<(), Self::Error> {
//!         // ...
//! #       Ok(())
//!     }
//!     fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
//!         // ...
//! #       Ok(())
//!     }
//!     fn write_iter_read<B: IntoIterator<Item = u8>>(&mut self, addr: u8, bytes: B, buffer: &mut [u8]) -> Result<(), Self::Error> {
//!         // ...
//! #       Ok(())
//!     }
//!     fn transaction<'a>(&mut self, address: u8, operations: &mut [Operation<'a>]) -> Result<(), Self::Error> {
//!         // ...
//! #       Ok(())
//!     }
//!     fn transaction_iter<'a, O: IntoIterator<Item = Operation<'a>>>(&mut self, address: u8, operations: O) -> Result<(), Self::Error> {
//!         // ...
//! #       Ok(())
//!     }
//! }
//!
//! impl I2c<TenBitAddress> for I2c0
//! {
//!     fn read(&mut self, addr: u16, buffer: &mut [u8]) -> Result<(), Self::Error> {
//!         // ...
//! #       Ok(())
//!     }
//!     fn write(&mut self, addr: u16, bytes: &[u8]) -> Result<(), Self::Error> {
//!         // ...
//! #       Ok(())
//!     }
//!     fn write_iter<B: IntoIterator<Item = u8>>(&mut self, addr: u16, bytes: B) -> Result<(), Self::Error> {
//!         // ...
//! #       Ok(())
//!     }
//!     fn write_read(&mut self, addr: u16, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
//!         // ...
//! #       Ok(())
//!     }
//!     fn write_iter_read<B: IntoIterator<Item = u8>>(&mut self, addr: u16, bytes: B, buffer: &mut [u8]) -> Result<(), Self::Error> {
//!         // ...
//! #       Ok(())
//!     }
//!     fn transaction<'a>(&mut self, address: u16, operations: &mut [Operation<'a>]) -> Result<(), Self::Error> {
//!         // ...
//! #       Ok(())
//!     }
//!     fn transaction_iter<'a, O: IntoIterator<Item = Operation<'a>>>(&mut self, address: u16, operations: O) -> Result<(), Self::Error> {
//!         // ...
//! #       Ok(())
//!     }
//! }
//! ```
//!
//! ### Device driver compatible only with 7-bit addresses
//!
//! For demonstration purposes the address mode parameter has been omitted in this example.
//!
//! ```
//! # use embedded_hal::i2c::{blocking::I2c, Error};
//! const ADDR: u8  = 0x15;
//! # const TEMP_REGISTER: u8 = 0x1;
//! pub struct TemperatureSensorDriver<I2C> {
//!     i2c: I2C,
//! }
//!
//! impl<I2C, E: Error> TemperatureSensorDriver<I2C>
//! where
//!     I2C: I2c<Error = E>,
//! {
//!     pub fn read_temperature(&mut self) -> Result<u8, E> {
//!         let mut temp = [0];
//!         self.i2c
//!             .write_read(ADDR, &[TEMP_REGISTER], &mut temp)
//!             .and(Ok(temp[0]))
//!     }
//! }
//! ```
//!
//! ### Device driver compatible only with 10-bit addresses
//!
//! ```
//! # use embedded_hal::i2c::{Error, TenBitAddress, blocking::I2c};
//! const ADDR: u16  = 0x158;
//! # const TEMP_REGISTER: u8 = 0x1;
//! pub struct TemperatureSensorDriver<I2C> {
//!     i2c: I2C,
//! }
//!
//! impl<I2C, E: Error> TemperatureSensorDriver<I2C>
//! where
//!     I2C: I2c<TenBitAddress, Error = E>,
//! {
//!     pub fn read_temperature(&mut self) -> Result<u8, E> {
//!         let mut temp = [0];
//!         self.i2c
//!             .write_read(ADDR, &[TEMP_REGISTER], &mut temp)
//!             .and(Ok(temp[0]))
//!     }
//! }
//! ```

mod private {
    pub trait Sealed {}
    impl Sealed for super::SevenBitAddress {}
    impl Sealed for super::TenBitAddress {}
}

/// I2C error
pub trait Error: core::fmt::Debug {
    /// Convert error to a generic I2C error kind
    ///
    /// By using this method, I2C errors freely defined by HAL implementations
    /// can be converted to a set of generic I2C errors upon which generic
    /// code can act.
    fn kind(&self) -> ErrorKind;
}

impl Error for core::convert::Infallible {
    fn kind(&self) -> ErrorKind {
        match *self {}
    }
}

/// I2C error kind
///
/// This represents a common set of I2C operation errors. HAL implementations are
/// free to define more specific or additional error types. However, by providing
/// a mapping to these common I2C errors, generic code can still react to them.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Bus error occurred. e.g. A START or a STOP condition is detected and is not
    /// located after a multiple of 9 SCL clock pulses.
    Bus,
    /// The arbitration was lost, e.g. electrical problems with the clock signal
    ArbitrationLoss,
    /// A bus operation was not acknowledged, e.g. due to the addressed device not
    /// being available on the bus or the device not being ready to process requests
    /// at the moment
    NoAcknowledge(NoAcknowledgeSource),
    /// The peripheral receive buffer was overrun
    Overrun,
    /// A different error occurred. The original error may contain more information.
    Other,
}

/// I2C no acknowledge error source
///
/// In cases where it is possible, a device should indicate if a no acknowledge
/// response was received to an address versus a no acknowledge to a data byte.
/// Where it is not possible to differentiate, `Unknown` should be indicated.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum NoAcknowledgeSource {
    /// The device did not acknowledge its address. The device may be missing.
    Address,
    /// The device did not acknowledge the data. It may not be ready to process
    /// requests at the moment.
    Data,
    /// Either the device did not acknowledge its address or the data, but it is
    /// unknown which.
    Unknown,
}

impl Error for ErrorKind {
    fn kind(&self) -> ErrorKind {
        *self
    }
}

impl core::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Bus => write!(f, "Bus error occurred"),
            Self::ArbitrationLoss => write!(f, "The arbitration was lost"),
            Self::NoAcknowledge(s) => s.fmt(f),
            Self::Overrun => write!(f, "The peripheral receive buffer was overrun"),
            Self::Other => write!(
                f,
                "A different error occurred. The original error may contain more information"
            ),
        }
    }
}

impl core::fmt::Display for NoAcknowledgeSource {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Address => write!(f, "The device did not acknowledge its address"),
            Self::Data => write!(f, "The device did not acknowledge the data"),
            Self::Unknown => write!(f, "The device did not acknowledge its address or the data"),
        }
    }
}

/// I2C error type trait
///
/// This just defines the error type, to be used by the other traits.
pub trait ErrorType {
    /// Error type
    type Error: Error;
}

impl<T: ErrorType> ErrorType for &mut T {
    type Error = T::Error;
}

/// Address mode (7-bit / 10-bit)
///
/// Note: This trait is sealed and should not be implemented outside of this crate.
pub trait AddressMode: Copy + private::Sealed + 'static {}

/// 7-bit address mode type
pub type SevenBitAddress = u8;

/// 10-bit address mode type
pub type TenBitAddress = u16;

impl AddressMode for SevenBitAddress {}

impl AddressMode for TenBitAddress {}

/// Direction of an i2c transfer.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    /// I2C read (RnW bit is 1)
    Read,
    /// I2C write (RnW bit is 0)
    Write,
}

/// Blocking I2C traits
pub mod blocking {
    use super::*;

    /// I2C device trait
    ///
    /// `I2cDevice` represents ownership over a single I2C device on a (possibly shared) bus.
    ///
    /// See the [module-level documentation](self) for important usage information.
    pub trait I2cDevice: ErrorType {
        /// I2C Bus type for this device.
        type Bus: I2cBusBase;

        /// Perform a transaction against the device.
        ///
        /// - Locks the bus
        /// - Calls `f` with an exclusive reference to the bus, which can then be used to do transfers against the device.
        /// - Does a [stop condition](I2cBus::stop) on the bus.
        /// - [Flushes](I2cBus::flush) the bus.
        /// - Unlocks the bus.
        ///
        /// The locking mechanism is implementation-defined. The only requirement is it must prevent two
        /// transactions from executing concurrently against the same bus. Examples of implementations are:
        /// critical sections, blocking mutexes, returning an error or panicking if the bus is already busy.
        fn transaction<R>(
            &mut self,
            f: impl FnOnce(&mut Self::Bus) -> Result<R, <Self::Bus as ErrorType>::Error>,
        ) -> Result<R, Self::Error>;

        /// Do a write within a transaction.
        ///
        /// This is a convenience method equivalent to `device.transaction(|bus| { bus.start(address, Write); bus.write(buf) })`.
        ///
        /// See also: [`I2cDevice::transaction`], [`I2cBusBase::write`]
        fn write<A: AddressMode>(&mut self, address: A, buf: &[u8]) -> Result<(), Self::Error>
        where
            Self::Bus: I2cBus<A>,
        {
            self.transaction(|bus| {
                bus.start(address, Direction::Write)?;
                bus.write(buf)
            })
        }

        /// Do a read within a transaction.
        ///
        /// This is a convenience method equivalent to `device.transaction(|bus| bus.read(buf))`.
        ///
        /// See also: [`I2cDevice::transaction`], [`I2cBusBase::read`]
        fn read<A: AddressMode>(&mut self, address: A, buf: &mut [u8]) -> Result<(), Self::Error>
        where
            Self::Bus: I2cBus<A>,
        {
            self.transaction(|bus| {
                bus.start(address, Direction::Read)?;
                bus.read(buf)
            })
        }

        /// Do a write, restart, read transaction.
        ///
        /// This is a convenience method equivalent to `device.transaction(|bus| bus.transfer(read, write))`.
        ///
        /// See also: [`I2cDevice::transaction`], [`I2cBus::transfer`]
        fn write_read<A: AddressMode>(
            &mut self,
            address: A,
            write: &[u8],
            read: &mut [u8],
        ) -> Result<(), Self::Error>
        where
            Self::Bus: I2cBus<A>,
        {
            self.transaction(|bus| {
                bus.start(address, Direction::Write)?;
                bus.write(write)?;
                bus.start(address, Direction::Read)?;
                bus.read(read)
            })
        }
    }

    impl<T: I2cDevice> I2cDevice for &mut T {
        type Bus = T::Bus;
        fn transaction<R>(
            &mut self,
            f: impl FnOnce(&mut Self::Bus) -> Result<R, <Self::Bus as ErrorType>::Error>,
        ) -> Result<R, Self::Error> {
            T::transaction(self, f)
        }
    }

    /// I2C bus base trait.
    ///
    /// This trait contains the methods that don't depend on the address mode. You will
    /// likely want to add bounds on [`I2cBus`] instead.
    pub trait I2cBusBase: ErrorType {
        /// Read data bytes from the SPI device.
        ///
        /// This is an error if the bus is not in "started" state, or if it's started for the wrong
        /// direction. You must have called [`start`](I2cBus::start)
        /// before calling this method with the correct direction.
        fn read(&mut self, bytes: &mut [u8]) -> Result<(), Self::Error>;

        /// Write data bytes to the SPI device.
        ///
        /// This is an error if the bus is not in "started" state, or if it's started for the wrong
        /// direction. You must have called [`start`](I2cBus::start)
        /// before calling this method with the correct direction.
        fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error>;

        /// Do a stop condition.
        ///
        /// This is a no-op if the bus is already in "stopped" state.
        fn stop(&mut self) -> Result<(), Self::Error>;

        /// Wait until all operations have completed, and return all pending errors.
        ///
        /// See the [module-level documentation](self) for more details on flushing.
        fn flush(&mut self) -> Result<(), Self::Error>;
    }

    /// I2C bus trait.
    ///
    /// This trait is generic on the address mode, and has [`I2cBusBase`] as a supertrait
    /// for the methods that don't depend on the address mode.
    pub trait I2cBus<A: AddressMode = SevenBitAddress>: I2cBusBase + ErrorType {
        /// Do a start or repeated-start condition, and send the address byte(s).
        ///
        /// This does a start condition if the bus was in "stopped" state, and a repeated-start
        /// condition if it was in the "started" state. The bus changes to the "started" state.
        ///
        /// Note that implementations are allowed to buffer operations and defer errors. This means
        /// that a call to `start` returning without an error doesn't necessarily mean the addressed
        /// device has ACKed the address byte. The NACK error can be reported in later calls instead.
        /// For more details, see the [module-level documentation](self).
        fn start(&mut self, address: A, direction: Direction) -> Result<(), Self::Error>;
    }

    impl<T: I2cBusBase> I2cBusBase for &mut T {
        fn read(&mut self, bytes: &mut [u8]) -> Result<(), Self::Error> {
            T::read(self, bytes)
        }

        fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
            T::write(self, bytes)
        }

        fn stop(&mut self) -> Result<(), Self::Error> {
            T::stop(self)
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            T::flush(self)
        }
    }

    impl<A: AddressMode, T: I2cBus<A>> I2cBus<A> for &mut T {
        fn start(&mut self, address: A, direction: Direction) -> Result<(), Self::Error> {
            T::start(self, address, direction)
        }
    }
}
