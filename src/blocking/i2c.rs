//! Blocking I2C API
//!
//! Slave addresses used by this API are 7-bit I2C addresses ranging from 0 to 127.
//!
//! Operations on 10-bit slave addresses are not supported by the API yet (but applications might
//! be able to emulate some operations).

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

/// Blocking write (iterator version)
#[cfg(feature = "unproven")]
pub trait WriteIter {
    /// Error type
    type Error;

    /// Sends bytes to slave with address `addr`
    ///
    /// # I2C Events (contract)
    ///
    /// Same as `Write`
    fn write<B>(&mut self, addr: u8, bytes: B) -> Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8>;
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

/// Blocking write (iterator version) + read
#[cfg(feature = "unproven")]
pub trait WriteIterRead {
    /// Error type
    type Error;

    /// Sends bytes to slave with address `addr` and then reads enough bytes to fill `buffer` *in a
    /// single transaction*
    ///
    /// # I2C Events (contract)
    ///
    /// Same as the `WriteRead` trait
    fn write_iter_read<B>(
        &mut self,
        address: u8,
        bytes: B,
        buffer: &mut [u8],
    ) -> Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8>;
}

/// Actions for transactional I2C trait
///
/// This allows composition of I2C operations into a single bus transaction
#[cfg(feature = "unproven")]
#[derive(Debug, PartialEq)]
pub enum Actions<'a> {
    /// Read data into the provided buffer
    Read(&'a mut [u8]),
    /// Write data from the provided buffer
    Write(&'a [u8]),
}

/// Transactional trait allows multiple actions to be executed
/// as part of a single I2C transaction
#[cfg(feature = "unproven")]
pub trait Transactional {
    /// Associated error type
    type Error;

    /// Execute the provided actions against the provided I2C address
    fn exec<'a, A>(&mut self, address: u8, actions: A) -> Result<(), Self::Error>
        where A: AsMut<[Actions<'a>]>;
}
