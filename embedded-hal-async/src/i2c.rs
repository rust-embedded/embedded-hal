//! Async I2C API
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

use core::future::Future;
pub use embedded_hal::i2c::blocking::Operation;
pub use embedded_hal::i2c::{
    AddressMode, Error, ErrorKind, ErrorType, NoAcknowledgeSource, SevenBitAddress, TenBitAddress,
};

/// Async i2c
pub trait I2c<A: AddressMode = SevenBitAddress>: ErrorType {
    /// Future returned by the `read` method.
    type ReadFuture<'a>: Future<Output = Result<(), Self::Error>>
    where
        Self: 'a;

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
    fn read<'a>(&'a mut self, address: A, read: &'a mut [u8]) -> Self::ReadFuture<'a>;

    /// Future returned by the `write` method.
    type WriteFuture<'a>: Future<Output = Result<(), Self::Error>>
    where
        Self: 'a;

    /// Writes bytes to slave with address `address`
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
    fn write<'a>(&'a mut self, address: A, write: &'a [u8]) -> Self::WriteFuture<'a>;

    /// Future returned by the `write_read` method.
    type WriteReadFuture<'a>: Future<Output = Result<(), Self::Error>>
    where
        Self: 'a;

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
    fn write_read<'a>(
        &'a mut self,
        address: A,
        write: &'a [u8],
        read: &'a mut [u8],
    ) -> Self::WriteReadFuture<'a>;

    /// Future returned by the `transaction` method.
    type TransactionFuture<'a, 'b>: Future<Output = Result<(), Self::Error>>
    where
        Self: 'a,
        'b: 'a;

    /// Execute the provided operations on the I2C bus as a single transaction.
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
    fn transaction<'a, 'b>(
        &'a mut self,
        address: A,
        operations: &'a mut [Operation<'b>],
    ) -> Self::TransactionFuture<'a, 'b>;
}

impl<A: AddressMode, T: I2c<A>> I2c<A> for &mut T {
    type ReadFuture<'a> = T::ReadFuture<'a> where Self: 'a;

    fn read<'a>(&'a mut self, address: A, buffer: &'a mut [u8]) -> Self::ReadFuture<'a> {
        T::read(self, address, buffer)
    }

    type WriteFuture<'a> = T::WriteFuture<'a> where Self: 'a;

    fn write<'a>(&'a mut self, address: A, bytes: &'a [u8]) -> Self::WriteFuture<'a> {
        T::write(self, address, bytes)
    }

    type WriteReadFuture<'a> = T::WriteReadFuture<'a> where Self: 'a;

    fn write_read<'a>(
        &'a mut self,
        address: A,
        bytes: &'a [u8],
        buffer: &'a mut [u8],
    ) -> Self::WriteReadFuture<'a> {
        T::write_read(self, address, bytes, buffer)
    }

    type TransactionFuture<'a, 'b> = T::TransactionFuture<'a, 'b> where Self: 'a, 'b: 'a;

    fn transaction<'a, 'b>(
        &'a mut self,
        address: A,
        operations: &'a mut [Operation<'b>],
    ) -> Self::TransactionFuture<'a, 'b> {
        T::transaction(self, address, operations)
    }
}
