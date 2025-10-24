//! Async I2C API.
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

pub use embedded_hal::i2c::{
    AddressMode, Error, ErrorKind, ErrorType, NoAcknowledgeSource, Operation, SevenBitAddress,
    TenBitAddress,
};

/// Async I2c.
pub trait I2c<A: AddressMode = SevenBitAddress>: ErrorType {
    /// Reads enough bytes from target with `address` to fill `buffer`.
    ///
    /// # I2C Events (contract)
    ///
    /// ``` text
    /// Controller: ST TAD+R        CAK    CAK ...    NCAK SP
    /// Target:              TAK B0     B1     ... BN
    /// ```
    ///
    /// Where
    ///
    /// - `ST` = start condition
    /// - `TAD+R` = target address followed by bit 1 to indicate reading
    /// - `TAK` = target acknowledge
    /// - `Bi` = ith byte of data
    /// - `CAK` = controller acknowledge
    /// - `NCAK` = controller no acknowledge
    /// - `SP` = stop condition
    #[inline]
    async fn read(&mut self, address: A, read: &mut [u8]) -> Result<(), Self::Error> {
        self.transaction(address, &mut [Operation::Read(read)])
            .await
    }

    /// Writes bytes to target with address `address`.
    ///
    /// # I2C Events (contract)
    ///
    /// ``` text
    /// Controller: ST TAD+W     B0     B1     ... BN     SP
    /// Target:              TAK    TAK    TAK ...    TAK
    /// ```
    ///
    /// Where
    ///
    /// - `ST` = start condition
    /// - `TAD+W` = target address followed by bit 0 to indicate writing
    /// - `TAK` = target acknowledge
    /// - `Bi` = ith byte of data
    /// - `SP` = stop condition
    #[inline]
    async fn write(&mut self, address: A, write: &[u8]) -> Result<(), Self::Error> {
        self.transaction(address, &mut [Operation::Write(write)])
            .await
    }

    /// Writes bytes to target with address `address` and then reads enough bytes to fill `read` *in a
    /// single transaction*.
    ///
    /// # I2C Events (contract)
    ///
    /// ``` text
    /// Controller: ST TAD+W     O0     O1     ... OM     SR TAD+R        CAK    CAK ...    NCAK SP
    /// Target:              TAK    TAK    TAK ...    TAK          TAK I0     I1     ... IN
    /// ```
    ///
    /// Where
    ///
    /// - `ST` = start condition
    /// - `TAD+W` = target address followed by bit 0 to indicate writing
    /// - `TAK` = target acknowledge
    /// - `Oi` = ith outgoing byte of data
    /// - `SR` = repeated start condition
    /// - `TAD+R` = target address followed by bit 1 to indicate reading
    /// - `Ii` = ith incoming byte of data
    /// - `CAK` = controller acknowledge
    /// - `NCAK` = controller no acknowledge
    /// - `SP` = stop condition
    #[inline]
    async fn write_read(
        &mut self,
        address: A,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.transaction(
            address,
            &mut [Operation::Write(write), Operation::Read(read)],
        )
        .await
    }

    /// Execute the provided operations on the I2C bus as a single transaction.
    ///
    /// Transaction contract:
    /// - Before executing the first operation an ST is sent automatically. This is followed by TAD+R/W as appropriate.
    /// - Data from adjacent operations of the same type are sent after each other without an SP or SR.
    /// - Between adjacent operations of a different type an SR and TAD+R/W is sent.
    /// - After executing the last operation an SP is sent automatically.
    /// - At the end of each read operation (before SP or SR), the controller does not send an acknowledge for the last byte.
    ///
    /// - `ST` = start condition
    /// - `TAD+R/W` = target address followed by bit 1 to indicate reading or 0 to indicate writing
    /// - `SR` = repeated start condition
    /// - `SP` = stop condition
    async fn transaction(
        &mut self,
        address: A,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error>;
}

impl<A: AddressMode, T: I2c<A> + ?Sized> I2c<A> for &mut T {
    #[inline]
    async fn read(&mut self, address: A, read: &mut [u8]) -> Result<(), Self::Error> {
        T::read(self, address, read).await
    }

    #[inline]
    async fn write(&mut self, address: A, write: &[u8]) -> Result<(), Self::Error> {
        T::write(self, address, write).await
    }

    #[inline]
    async fn write_read(
        &mut self,
        address: A,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        T::write_read(self, address, write, read).await
    }

    #[inline]
    async fn transaction(
        &mut self,
        address: A,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        T::transaction(self, address, operations).await
    }
}
