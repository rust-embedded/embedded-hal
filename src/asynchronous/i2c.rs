//! Definitions for I²C peripherals.
use crate::asynchronous::io;
use core::fmt;
use core::pin;
use core::task;

pub mod begin_read;
pub mod begin_write;
pub mod initialize;

/// A peripheral that can perform I²C read operations.
// TODO: this should maybe capture the lifetime of self and let it flow into Self::Read
pub trait I2cRead: fmt::Debug {
    /// The common error type for I²C read operations.
    ///
    /// A single error type for all operations is enforced for simplicity.
    type Error: io::ReadError;
    /// An object that can be used to complete the read operation.
    type Read: io::Read<Error = Self::Error> + Unpin;

    /// Polls the start of a read operation to completion.
    fn poll_begin_read(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        addr: u8,
    ) -> task::Poll<Result<Self::Read, Self::Error>>;
}

/// Extension functions for instances of [`I2cRead`].
// TODO: this should maybe capture the lifetime of self and let it flow into Self::Read
pub trait I2cReadExt: I2cRead {
    /// Initiates a read operation on the specified address.
    ///
    /// The returned object can be used to read the actual data from the address.  The user must
    /// read the data until completion, or else it might leave this I²C peripheral in an incomplete
    /// state.
    fn begin_read(&mut self, address: u8) -> begin_read::BeginRead<Self>
    where
        Self: Unpin,
    {
        begin_read::begin_read(self, address)
    }
}

impl<'r, A> I2cReadExt for A where A: I2cRead {}

/// Reads from the specified address into the specified buffer.
///
/// Returns the number of bytes read.
pub async fn read<R>(i2c: &mut R, address: u8, dest: &mut [u8]) -> Result<usize, R::Error>
where
    R: I2cRead + Unpin,
{
    use crate::asynchronous::io::ReadExt;

    let mut reader = i2c.begin_read(address).await?;
    let size = reader.read(dest).await?;

    Ok(size)
}

/// Reads from the specified address into the specified buffer, waiting for the exact amount of
/// bytes to arrive.
pub async fn read_exact<R>(i2c: &mut R, address: u8, dest: &mut [u8]) -> Result<(), R::Error>
where
    R: I2cRead + Unpin,
{
    use crate::asynchronous::io::ReadExt;

    let mut reader = i2c.begin_read(address).await?;
    reader.read_exact(dest).await?;

    Ok(())
}

/// A peripheral that can perform I²C write operations.
pub trait I2cWrite: fmt::Debug {
    /// The common error type for I²C write operations.
    ///
    /// A single error type for all operations is enforced for simplicity.
    type Error: io::WriteError;
    /// An object that can be used to complete the write operation.
    type Write: io::Write<Error = Self::Error> + Unpin;

    /// Polls the start of a write operation to completion.
    fn poll_begin_write(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        addr: u8,
    ) -> task::Poll<Result<Self::Write, Self::Error>>;
}

/// Extension functions for instances of [`I2cWrite`].
pub trait I2cWriteExt: I2cWrite {
    /// Initiates a write operation on the specified address.
    ///
    /// The returned object can be used to write the actual data to the address.  The user must call
    /// `shutdown` when done writing, or else it might leave this I²C peripheral in an incomplete
    /// state.  For example, the I²C peripheral might decide to flush remaining data in the [`Drop`]
    /// implementation, which will be blocking.
    fn begin_write(&mut self, address: u8) -> begin_write::BeginWrite<Self>
    where
        Self: Unpin,
    {
        begin_write::begin_write(self, address)
    }
}

impl<A> I2cWriteExt for A where A: I2cWrite {}

/// Writes from the specified buffer to the specified address.
///
/// Returns the number of bytes that were written.
pub async fn write<W>(i2c: &mut W, address: u8, data: &[u8]) -> Result<usize, W::Error>
where
    W: I2cWrite + Unpin,
{
    use crate::asynchronous::io::WriteExt;

    let mut writer = i2c.begin_write(address).await?;
    let size = writer.write(data).await?;
    writer.shutdown().await?;

    Ok(size)
}

/// Writes all of the bytes from the specified buffer to the specified address.
pub async fn write_all<W>(i2c: &mut W, address: u8, data: &[u8]) -> Result<(), W::Error>
where
    W: I2cWrite + Unpin,
{
    use crate::asynchronous::io::WriteExt;

    let mut writer = i2c.begin_write(address).await?;
    writer.write_all(data).await?;
    writer.shutdown().await?;

    Ok(())
}

/// Defines a mapping for two GPIO pins that can be used to create an I²C bus.
pub trait I2cBusMapping<SDA, SCL> {
    /// The common error type for I²C operations.
    ///
    /// A single error type for all operations is enforced for simplicity.
    type Error: io::ReadError + io::WriteError;
    /// The I²C bus that will be produced once initialization based off of this mapping succeeds.
    type Bus: I2cRead<Error = Self::Error> + I2cWrite<Error = Self::Error>;

    /// Polls the initialization operation to completion.
    fn poll_initialize(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        sda: &mut SDA,
        scl: &mut SCL,
    ) -> task::Poll<Result<Self::Bus, Self::Error>>
    where
        Self: Sized;
}

/// Extension functions for instances of [`I2cBusMapping`].
pub trait I2cBusMappingExt<SDA, SCL>: I2cBusMapping<SDA, SCL>
where
    SDA: Unpin,
    SCL: Unpin,
{
    /// Initializes a new I²C bus based off of the two provided SDA (data) and SCL (clock) pins.
    fn initialize(self, sda: SDA, scl: SCL) -> initialize::Initialize<Self, SDA, SCL>
    where
        Self: Sized + Unpin,
    {
        initialize::initialize(self, sda, scl)
    }
}

impl<A, SDA, SCL> I2cBusMappingExt<SDA, SCL> for A
where
    A: I2cBusMapping<SDA, SCL>,
    SDA: Unpin,
    SCL: Unpin,
{
}
