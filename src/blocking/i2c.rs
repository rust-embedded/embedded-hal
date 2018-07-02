//! Blocking I2C API
//!
//! Slave addresses used by this API are 7-bit I2C addresses ranging from 0 to 127.
//!
//! Operations on 10-bit slave addresses are not supported by the API yet (but applications might
//! be able to emulate some operations).

use core::cell::RefCell;

/// Blocking read
pub trait Read {
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
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error>;
}

/// Blocking write
pub trait Write {
    /// Error type
    type Error;

    /// Sends bytes to slave with address `addr`
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
    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error>;
}

/// Blocking write + read
pub trait WriteRead {
    /// Error type
    type Error;

    /// Sends bytes to slave with address `addr` and then reads enough bytes to fill `buffer` *in a
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
    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error>;
}

/// Blanket implementations to enable I2C bus sharing via RefCell

/// # Example usage
///
/// ``` ignore
/// let mut i2c = hal::I2c::i2c (...);
///
/// // Stash i2c instance into a RefCell for sharing
/// let shared_i2c = RefCell::new(i2c);
///
/// // Pass it on to one or more drivers for devices on the same bus
/// let mut driver_a = DriverA::new(&mut &shared_i2c);
///
/// // Use the shared bus with the drivers
/// driver_a.do_stuff();
///
/// // Use it independently of a driver for direct bus interaction
/// let mut data = [0; 2];
/// shared_i2c.borrow_mut().read(0x11, &mut data);
/// ```
impl<'a, I> ::blocking::i2c::Read for &'a RefCell<I>
where
    I: ::blocking::i2c::Read,
{
    type Error = <I as Read>::Error;
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.borrow_mut().read(address, buffer)
    }
}

/// # Example usage
///
/// ``` ignore
/// let mut i2c = hal::I2c::i2c (...);
///
/// // Stash i2c instance into a RefCell for sharing
/// let shared_i2c = RefCell::new(i2c);
///
/// // Pass it on to one or more drivers for devices on the same bus
/// let mut driver_a = DriverA::new(&mut &shared_i2c);
///
/// // Use the shared bus with the drivers
/// driver_a.do_stuff();
///
/// // Use it independently of a driver for direct bus interaction
/// shared_i2c.borrow_mut().write(0x33, &[0x01, 0x02]);
/// ```
impl<'a, I> ::blocking::i2c::Write for &'a RefCell<I>
where
    I: ::blocking::i2c::Write,
{
    type Error = <I as Write>::Error;
    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.borrow_mut().write(addr, bytes)
    }
}

/// # Example usage
///
/// ``` ignore
/// let mut i2c = hal::I2c::i2c (...);
///
/// // Stash i2c instance into a RefCell for sharing
/// let shared_i2c = RefCell::new(i2c);
///
/// // Pass it on to one or more drivers for devices on the same bus
/// let mut driver_a = DriverA::new(&mut &shared_i2c);
///
/// // Use the shared bus with the drivers
/// driver_a.do_stuff();
///
/// // Use it independently of a driver for direct bus interaction
/// let mut data = [0; 2];
/// shared_i2c.borrow_mut().write_read(0x22, &[0x00], &mut data);
/// ```
impl<'a, I> ::blocking::i2c::WriteRead for &'a RefCell<I>
where
    I: ::blocking::i2c::WriteRead,
{
    type Error = <I as WriteRead>::Error;
    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.borrow_mut().write_read(address, bytes, buffer)
    }
}
