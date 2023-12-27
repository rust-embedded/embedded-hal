//! Blocking I2C API.
//!
//! This API supports 7-bit and 10-bit addresses. Traits feature an [`AddressMode`]
//! marker type parameter. Two implementation of the [`AddressMode`] exist:
//! [`SevenBitAddress`] and [`TenBitAddress`].
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
//! [`SevenBitAddress`] has been set as default mode and thus can be omitted if desired.
//!
//! # Bus sharing
//!
//! I2C allows sharing a single bus between many I2C devices. The SDA and SCL lines are
//! wired in parallel to all devices. When starting a transfer an "address" is sent
//! so that the addressed device can respond and all the others can ignore the transfer.
//!
#![doc = include_str!("i2c-shared-bus.svg")]
//!
//! This bus sharing is common when having multiple I2C devices in the same board, since it uses fewer MCU
//! pins (`2` instead of `2*n`), and fewer MCU I2C peripherals (`1` instead of `n`).
//!
//! This API supports bus sharing natively. Types implementing [`I2c`] are allowed
//! to represent either exclusive or shared access to an I2C bus. HALs typically
//! provide exclusive access implementations. Drivers shouldn't care which
//! kind they receive, they just do transactions on it and let the
//! underlying implementation share or not.
//!
//! The [`embedded-hal-bus`](https://docs.rs/embedded-hal-bus) crate provides several
//! implementations for sharing I2C buses. You can use them to take an exclusive instance
//! you've received from a HAL and "split" it into multiple shared ones, to instantiate
//! several drivers on the same bus.
//!
//! # Flushing
//!
//! Implementations must flush the transfer, ensuring the bus has returned to an idle state before returning.
//! No pipelining is allowed. Users must be able to shut down the I2C peripheral immediately after a transfer
//! returns, without any risk of e.g. cutting short a stop condition.
//!
//! (Implementations must wait until the last ACK bit to report it as an error anyway. Therefore pipelining would only
//! yield very small time savings, not worth the complexity)
//!
//! # For driver authors
//!
//! Drivers can select the adequate address length with `I2c<SevenBitAddress>` or `I2c<TenBitAddress>` depending
//! on the target device. If it can use either, the driver can
//! be generic over the address kind as well, though this is rare.
//!
//! Drivers should take the `I2c` instance as an argument to `new()`, and store it in their
//! struct. They **should not** take `&mut I2c`, the trait has a blanket impl for all `&mut T`
//! so taking just `I2c` ensures the user can still pass a `&mut`, but is not forced to.
//!
//! Drivers **should not** try to enable bus sharing by taking `&mut I2c` at every method.
//! This is much less ergonomic than owning the `I2c`, which still allows the user to pass an
//! implementation that does sharing behind the scenes
//! (from [`embedded-hal-bus`](https://docs.rs/embedded-hal-bus), or others).
//!
//! ## Device driver compatible only with 7-bit addresses
//!
//! For demonstration purposes the address mode parameter has been omitted in this example.
//!
//! ```
//! use embedded_hal::i2c::{I2c, Error};
//!
//! const ADDR: u8 = 0x15;
//! # const TEMP_REGISTER: u8 = 0x1;
//! pub struct TemperatureSensorDriver<I2C> {
//!     i2c: I2C,
//! }
//!
//! impl<I2C: I2c> TemperatureSensorDriver<I2C> {
//!     pub fn new(i2c: I2C) -> Self {
//!         Self { i2c }
//!     }
//!
//!     pub fn read_temperature(&mut self) -> Result<u8, I2C::Error> {
//!         let mut temp = [0];
//!         self.i2c.write_read(ADDR, &[TEMP_REGISTER], &mut temp)?;
//!         Ok(temp[0])
//!     }
//! }
//! ```
//!
//! ## Device driver compatible only with 10-bit addresses
//!
//! ```
//! use embedded_hal::i2c::{Error, TenBitAddress, I2c};
//!
//! const ADDR: u16 = 0x158;
//! # const TEMP_REGISTER: u8 = 0x1;
//! pub struct TemperatureSensorDriver<I2C> {
//!     i2c: I2C,
//! }
//!
//! impl<I2C: I2c<TenBitAddress>> TemperatureSensorDriver<I2C> {
//!     pub fn new(i2c: I2C) -> Self {
//!         Self { i2c }
//!     }
//!
//!     pub fn read_temperature(&mut self) -> Result<u8, I2C::Error> {
//!         let mut temp = [0];
//!         self.i2c.write_read(ADDR, &[TEMP_REGISTER], &mut temp)?;
//!         Ok(temp[0])
//!     }
//! }
//! ```
//!
//! # For HAL authors
//!
//! HALs **should not** include bus sharing mechanisms. They should expose a single type representing
//! exclusive ownership over the bus, and let the user use [`embedded-hal-bus`](https://docs.rs/embedded-hal-bus)
//! if they want to share it. (One exception is if the underlying platform already
//! supports sharing, such as Linux or some RTOSs.)
//!
//! Here is an example of an embedded-hal implementation of the `I2C` trait
//! for both addressing modes. All trait methods have have default implementations in terms of `transaction`.
//! As such, that is the only method that requires implementation in the HAL.
//!
//! ```
//! use embedded_hal::i2c::{self, SevenBitAddress, TenBitAddress, I2c, Operation};
//!
//! /// I2C0 hardware peripheral which supports both 7-bit and 10-bit addressing.
//! pub struct I2c0;
//!
//! #[derive(Debug, Copy, Clone, Eq, PartialEq)]
//! pub enum Error {
//!     // ...
//! }
//!
//! impl i2c::Error for Error {
//!     fn kind(&self) -> i2c::ErrorKind {
//!         match *self {
//!             // ...
//!         }
//!     }
//! }
//!
//! impl i2c::ErrorType for I2c0 {
//!     type Error = Error;
//! }
//!
//! impl I2c<SevenBitAddress> for I2c0 {
//!     fn transaction(&mut self, address: u8, operations: &mut [Operation<'_>]) -> Result<(), Self::Error> {
//!         // ...
//! #       Ok(())
//!     }
//! }
//!
//! impl I2c<TenBitAddress> for I2c0 {
//!     fn transaction(&mut self, address: u16, operations: &mut [Operation<'_>]) -> Result<(), Self::Error> {
//!         // ...
//! #       Ok(())
//!     }
//! }
//! ```

use crate::private;

#[cfg(feature = "defmt-03")]
use crate::defmt;

/// I2C error.
pub trait Error: core::fmt::Debug {
    /// Convert error to a generic I2C error kind.
    ///
    /// By using this method, I2C errors freely defined by HAL implementations
    /// can be converted to a set of generic I2C errors upon which generic
    /// code can act.
    fn kind(&self) -> ErrorKind;
}

impl Error for core::convert::Infallible {
    #[inline]
    fn kind(&self) -> ErrorKind {
        match *self {}
    }
}

/// I2C error kind.
///
/// This represents a common set of I2C operation errors. HAL implementations are
/// free to define more specific or additional error types. However, by providing
/// a mapping to these common I2C errors, generic code can still react to them.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[non_exhaustive]
pub enum ErrorKind {
    /// Bus error occurred. e.g. A START or a STOP condition is detected and is not
    /// located after a multiple of 9 SCL clock pulses.
    Bus,
    /// The arbitration was lost, e.g. electrical problems with the clock signal.
    ArbitrationLoss,
    /// A bus operation was not acknowledged, e.g. due to the addressed device not
    /// being available on the bus or the device not being ready to process requests
    /// at the moment.
    NoAcknowledge(NoAcknowledgeSource),
    /// The peripheral receive buffer was overrun.
    Overrun,
    /// A different error occurred. The original error may contain more information.
    Other,
}

/// I2C no acknowledge error source.
///
/// In cases where it is possible, a device should indicate if a no acknowledge
/// response was received to an address versus a no acknowledge to a data byte.
/// Where it is not possible to differentiate, `Unknown` should be indicated.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
    #[inline]
    fn kind(&self) -> ErrorKind {
        *self
    }
}

impl core::fmt::Display for ErrorKind {
    #[inline]
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
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Address => write!(f, "The device did not acknowledge its address"),
            Self::Data => write!(f, "The device did not acknowledge the data"),
            Self::Unknown => write!(f, "The device did not acknowledge its address or the data"),
        }
    }
}

/// I2C error type trait.
///
/// This just defines the error type, to be used by the other traits.
pub trait ErrorType {
    /// Error type
    type Error: Error;
}

impl<T: ErrorType + ?Sized> ErrorType for &mut T {
    type Error = T::Error;
}

/// Address mode (7-bit / 10-bit).
///
/// Note: This trait is sealed and should not be implemented outside of this crate.
pub trait AddressMode: private::Sealed + 'static {}

/// 7-bit address mode type.
///
/// Note that 7-bit addresses defined by drivers should be specified in **right-aligned** form,
/// e.g. in the range `0x00..=0x7F`.
///
/// For example, a device that has the seven bit address of `0b011_0010`, and therefore is addressed on the wire using:
///
/// * `0b0110010_0` or `0x64` for *writes*
/// * `0b0110010_1` or `0x65` for *reads*
///
/// Should be specified as `0b0011_0010` or `0x32`, NOT `0x64` or `0x65`. Care should be taken by both HAL and driver
/// crate writers to use this scheme consistently.
pub type SevenBitAddress = u8;

/// 10-bit address mode type.
pub type TenBitAddress = u16;

impl AddressMode for SevenBitAddress {}

impl AddressMode for TenBitAddress {}

/// I2C operation.
///
/// Several operations can be combined as part of a transaction.
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Operation<'a> {
    /// Read data into the provided buffer.
    Read(&'a mut [u8]),
    /// Write data from the provided buffer.
    Write(&'a [u8]),
}

/// Blocking I2C.
pub trait I2c<A: AddressMode = SevenBitAddress>: ErrorType {
    /// Reads enough bytes from slave with `address` to fill `read`.
    ///
    /// # I2C Events (contract)
    ///
    /// ``` text
    /// Master: ST SAD+R        MAK    MAK ...    NMAK SP
    /// Slave:           SAK B0     B1     ... BN
    /// ```
    ///
    /// Where
    ///
    /// - `ST` = start condition
    /// - `SAD+R` = slave address followed by bit 1 to indicate reading
    /// - `SAK` = slave acknowledge
    /// - `Bi` = ith byte of data
    /// - `MAK` = master acknowledge
    /// - `NMAK` = master no acknowledge
    /// - `SP` = stop condition
    #[inline]
    fn read(&mut self, address: A, read: &mut [u8]) -> Result<(), Self::Error> {
        self.transaction(address, &mut [Operation::Read(read)])
    }

    /// Writes bytes to slave with address `address`.
    ///
    /// # I2C Events (contract)
    ///
    /// ``` text
    /// Master: ST SAD+W     B0     B1     ... BN     SP
    /// Slave:           SAK    SAK    SAK ...    SAK
    /// ```
    ///
    /// Where
    ///
    /// - `ST` = start condition
    /// - `SAD+W` = slave address followed by bit 0 to indicate writing
    /// - `SAK` = slave acknowledge
    /// - `Bi` = ith byte of data
    /// - `SP` = stop condition
    #[inline]
    fn write(&mut self, address: A, write: &[u8]) -> Result<(), Self::Error> {
        self.transaction(address, &mut [Operation::Write(write)])
    }

    /// Writes bytes to slave with address `address` and then reads enough bytes to fill `read` *in a
    /// single transaction*.
    ///
    /// # I2C Events (contract)
    ///
    /// ``` text
    /// Master: ST SAD+W     O0     O1     ... OM     SR SAD+R        MAK    MAK ...    NMAK SP
    /// Slave:           SAK    SAK    SAK ...    SAK          SAK I0     I1     ... IN
    /// ```
    ///
    /// Where
    ///
    /// - `ST` = start condition
    /// - `SAD+W` = slave address followed by bit 0 to indicate writing
    /// - `SAK` = slave acknowledge
    /// - `Oi` = ith outgoing byte of data
    /// - `SR` = repeated start condition
    /// - `SAD+R` = slave address followed by bit 1 to indicate reading
    /// - `Ii` = ith incoming byte of data
    /// - `MAK` = master acknowledge
    /// - `NMAK` = master no acknowledge
    /// - `SP` = stop condition
    #[inline]
    fn write_read(&mut self, address: A, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error> {
        self.transaction(
            address,
            &mut [Operation::Write(write), Operation::Read(read)],
        )
    }

    /// Execute the provided operations on the I2C bus.
    ///
    /// Transaction contract:
    /// - Before executing the first operation an ST is sent automatically. This is followed by SAD+R/W as appropriate.
    /// - Data from adjacent operations of the same type are sent after each other without an SP or SR.
    /// - Between adjacent operations of a different type an SR and SAD+R/W is sent.
    /// - After executing the last operation an SP is sent automatically.
    /// - If the last operation is a `Read` the master does not send an acknowledge for the last byte.
    ///
    /// - `ST` = start condition
    /// - `SAD+R/W` = slave address followed by bit 1 to indicate reading or 0 to indicate writing
    /// - `SR` = repeated start condition
    /// - `SP` = stop condition
    fn transaction(
        &mut self,
        address: A,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error>;
}

impl<A: AddressMode, T: I2c<A> + ?Sized> I2c<A> for &mut T {
    #[inline]
    fn read(&mut self, address: A, read: &mut [u8]) -> Result<(), Self::Error> {
        T::read(self, address, read)
    }

    #[inline]
    fn write(&mut self, address: A, write: &[u8]) -> Result<(), Self::Error> {
        T::write(self, address, write)
    }

    #[inline]
    fn write_read(&mut self, address: A, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error> {
        T::write_read(self, address, write, read)
    }

    #[inline]
    fn transaction(
        &mut self,
        address: A,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        T::transaction(self, address, operations)
    }
}
