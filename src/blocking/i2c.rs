//! Blocking I2C API
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
//! # use embedded_hal::blocking::i2c::{SevenBitAddress, TenBitAddress, Write};
//! /// I2C0 hardware peripheral which supports both 7-bit and 10-bit addressing.
//! pub struct I2c0;
//!
//! impl Write<SevenBitAddress> for I2c0
//! {
//! #   type Error = ();
//! #
//!     fn try_write(&mut self, addr: u8, output: &[u8]) -> Result<(), Self::Error> {
//!         // ...
//! #       Ok(())
//!     }
//! }
//!
//! impl Write<TenBitAddress> for I2c0
//! {
//! #   type Error = ();
//! #
//!     fn try_write(&mut self, addr: u16, output: &[u8]) -> Result<(), Self::Error> {
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
//! # use embedded_hal::blocking::i2c::WriteRead;
//! const ADDR: u8  = 0x15;
//! # const TEMP_REGISTER: u8 = 0x1;
//! pub struct TemperatureSensorDriver<I2C> {
//!     i2c: I2C,
//! }
//!
//! impl<I2C, E> TemperatureSensorDriver<I2C>
//! where
//!     I2C: WriteRead<Error = E>,
//! {
//!     pub fn read_temperature(&mut self) -> Result<u8, E> {
//!         let mut temp = [0];
//!         self.i2c
//!             .try_write_read(ADDR, &[TEMP_REGISTER], &mut temp)
//!             .and(Ok(temp[0]))
//!     }
//! }
//! ```
//!
//! ### Device driver compatible only with 10-bit addresses
//!
//! ```
//! # use embedded_hal::blocking::i2c::{TenBitAddress, WriteRead};
//! const ADDR: u16  = 0x158;
//! # const TEMP_REGISTER: u8 = 0x1;
//! pub struct TemperatureSensorDriver<I2C> {
//!     i2c: I2C,
//! }
//!
//! impl<I2C, E> TemperatureSensorDriver<I2C>
//! where
//!     I2C: WriteRead<TenBitAddress, Error = E>,
//! {
//!     pub fn read_temperature(&mut self) -> Result<u8, E> {
//!         let mut temp = [0];
//!         self.i2c
//!             .try_write_read(ADDR, &[TEMP_REGISTER], &mut temp)
//!             .and(Ok(temp[0]))
//!     }
//! }
//! ```

use crate::private;

/// Address mode (7-bit / 10-bit)
///
/// Note: This trait is sealed and should not be implemented outside of this crate.
pub trait AddressMode: private::Sealed {}

/// 7-bit address mode type
pub type SevenBitAddress = u8;

/// 10-bit address mode type
pub type TenBitAddress = u16;

impl AddressMode for SevenBitAddress {}

impl AddressMode for TenBitAddress {}

/// Blocking read
pub trait Read<A: AddressMode = SevenBitAddress> {
    /// Error type
    type Error;

    /// Reads enough bytes from slave with `address` to fill `buffer`
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
    fn try_read(&mut self, address: A, buffer: &mut [u8]) -> Result<(), Self::Error>;
}

/// Blocking write
pub trait Write<A: AddressMode = SevenBitAddress> {
    /// Error type
    type Error;

    /// Sends bytes to slave with address `address`
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
    fn try_write(&mut self, address: A, bytes: &[u8]) -> Result<(), Self::Error>;
}

/// Blocking write (iterator version)
pub trait WriteIter<A: AddressMode = SevenBitAddress> {
    /// Error type
    type Error;

    /// Sends bytes to slave with address `address`
    ///
    /// # I2C Events (contract)
    ///
    /// Same as `Write`
    fn try_write_iter<B>(&mut self, address: A, bytes: B) -> Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8>;
}

/// Blocking write + read
pub trait WriteRead<A: AddressMode = SevenBitAddress> {
    /// Error type
    type Error;

    /// Sends bytes to slave with address `address` and then reads enough bytes to fill `buffer` *in a
    /// single transaction*
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
    fn try_write_read(
        &mut self,
        address: A,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error>;
}

/// Blocking write (iterator version) + read
pub trait WriteIterRead<A: AddressMode = SevenBitAddress> {
    /// Error type
    type Error;

    /// Sends bytes to slave with address `address` and then reads enough bytes to fill `buffer` *in a
    /// single transaction*
    ///
    /// # I2C Events (contract)
    ///
    /// Same as the `WriteRead` trait
    fn try_write_iter_read<B>(
        &mut self,
        address: A,
        bytes: B,
        buffer: &mut [u8],
    ) -> Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8>;
}
