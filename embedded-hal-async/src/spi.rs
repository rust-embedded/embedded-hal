//! SPI master mode traits.

use core::fmt::Debug;

use embedded_hal::digital::OutputPin;
use embedded_hal::spi as blocking;
pub use embedded_hal::spi::{
    Error, ErrorKind, ErrorType, Mode, Operation, Phase, Polarity, MODE_0, MODE_1, MODE_2, MODE_3,
};

/// SPI read-only device trait
///
/// `SpiDeviceRead` represents ownership over a single SPI device on a (possibly shared) bus, selected
/// with a CS (Chip Select) pin.
///
/// See (the docs on embedded-hal)[embedded_hal::spi] for important information on SPI Bus vs Device traits.
///
/// See the [module-level documentation](self) for important usage information.
pub trait SpiDeviceRead<Word: Copy + 'static = u8>: ErrorType {
    /// Perform a read transaction against the device.
    ///
    /// - Locks the bus
    /// - Asserts the CS (Chip Select) pin.
    /// - Performs all the operations.
    /// - [Flushes](SpiBusFlush::flush) the bus.
    /// - Deasserts the CS pin.
    /// - Unlocks the bus.
    ///
    /// The locking mechanism is implementation-defined. The only requirement is it must prevent two
    /// transactions from executing concurrently against the same bus. Examples of implementations are:
    /// critical sections, blocking mutexes, returning an error or panicking if the bus is already busy.
    ///
    /// On bus errors the implementation should try to deassert CS.
    /// If an error occurs while deasserting CS the bus error should take priority as the return value.
    async fn read_transaction(&mut self, operations: &mut [&mut [Word]])
        -> Result<(), Self::Error>;

    /// Do a read within a transaction.
    ///
    /// This is a convenience method equivalent to `device.read_transaction(&mut [buf])`.
    ///
    /// See also: [`SpiDeviceRead::read_transaction`], [`SpiBusRead::read`]
    async fn read(&mut self, buf: &mut [Word]) -> Result<(), Self::Error> {
        self.read_transaction(&mut [buf]).await
    }
}

/// SPI write-only device trait
///
/// `SpiDeviceWrite` represents ownership over a single SPI device on a (possibly shared) bus, selected
/// with a CS (Chip Select) pin.
///
/// See (the docs on embedded-hal)[embedded_hal::spi] for important information on SPI Bus vs Device traits.
///
/// See the [module-level documentation](self) for important usage information.
pub trait SpiDeviceWrite<Word: Copy + 'static = u8>: ErrorType {
    /// Perform a write transaction against the device.
    ///
    /// - Locks the bus
    /// - Asserts the CS (Chip Select) pin.
    /// - Performs all the operations.
    /// - [Flushes](SpiBusFlush::flush) the bus.
    /// - Deasserts the CS pin.
    /// - Unlocks the bus.
    ///
    /// The locking mechanism is implementation-defined. The only requirement is it must prevent two
    /// transactions from executing concurrently against the same bus. Examples of implementations are:
    /// critical sections, blocking mutexes, returning an error or panicking if the bus is already busy.
    ///
    /// On bus errors the implementation should try to deassert CS.
    /// If an error occurs while deasserting CS the bus error should take priority as the return value.
    async fn write_transaction(&mut self, operations: &[&[Word]]) -> Result<(), Self::Error>;

    /// Do a write within a transaction.
    ///
    /// This is a convenience method equivalent to `device.write_transaction(&mut [buf])`.
    ///
    /// See also: [`SpiDeviceWrite::write_transaction`], [`SpiBusWrite::write`]
    async fn write(&mut self, buf: &[Word]) -> Result<(), Self::Error> {
        self.write_transaction(&[buf]).await
    }
}

/// SPI device trait
///
/// `SpiDevice` represents ownership over a single SPI device on a (possibly shared) bus, selected
/// with a CS (Chip Select) pin.
///
/// See (the docs on embedded-hal)[embedded_hal::spi] for important information on SPI Bus vs Device traits.
///
/// See the [module-level documentation](self) for important usage information.
pub trait SpiDevice<Word: Copy + 'static = u8>:
    SpiDeviceRead<Word> + SpiDeviceWrite<Word> + ErrorType
{
    /// Perform a transaction against the device.
    ///
    /// - Locks the bus
    /// - Asserts the CS (Chip Select) pin.
    /// - Performs all the operations.
    /// - [Flushes](SpiBusFlush::flush) the bus.
    /// - Deasserts the CS pin.
    /// - Unlocks the bus.
    ///
    /// The locking mechanism is implementation-defined. The only requirement is it must prevent two
    /// transactions from executing concurrently against the same bus. Examples of implementations are:
    /// critical sections, blocking mutexes, returning an error or panicking if the bus is already busy.
    ///
    /// On bus errors the implementation should try to deassert CS.
    /// If an error occurs while deasserting CS the bus error should take priority as the return value.
    async fn transaction(
        &mut self,
        operations: &mut [Operation<'_, Word>],
    ) -> Result<(), Self::Error>;

    /// Do a transfer within a transaction.
    ///
    /// This is a convenience method equivalent to `device.transaction(|bus| bus.transfer(read, write))`.
    ///
    /// See also: [`SpiDevice::transaction`], [`SpiBus::transfer`]
    async fn transfer(&mut self, read: &mut [Word], write: &[Word]) -> Result<(), Self::Error> {
        self.transaction(&mut [Operation::Transfer(read, write)])
            .await
    }

    /// Do an in-place transfer within a transaction.
    ///
    /// This is a convenience method equivalent to `device.transaction(|bus| bus.transfer_in_place(buf))`.
    ///
    /// See also: [`SpiDevice::transaction`], [`SpiBus::transfer_in_place`]
    async fn transfer_in_place(&mut self, buf: &mut [Word]) -> Result<(), Self::Error> {
        self.transaction(&mut [Operation::TransferInPlace(buf)])
            .await
    }
}

impl<Word: Copy + 'static, T: SpiDeviceRead<Word>> SpiDeviceRead<Word> for &mut T {
    async fn read_transaction(
        &mut self,
        operations: &mut [&mut [Word]],
    ) -> Result<(), Self::Error> {
        T::read_transaction(self, operations).await
    }

    async fn read(&mut self, buf: &mut [Word]) -> Result<(), Self::Error> {
        T::read(self, buf).await
    }
}

impl<Word: Copy + 'static, T: SpiDeviceWrite<Word>> SpiDeviceWrite<Word> for &mut T {
    async fn write_transaction(&mut self, operations: &[&[Word]]) -> Result<(), Self::Error> {
        T::write_transaction(self, operations).await
    }

    async fn write(&mut self, buf: &[Word]) -> Result<(), Self::Error> {
        T::write(self, buf).await
    }
}

impl<Word: Copy + 'static, T: SpiDevice<Word>> SpiDevice<Word> for &mut T {
    async fn transaction(
        &mut self,
        operations: &mut [Operation<'_, Word>],
    ) -> Result<(), Self::Error> {
        T::transaction(self, operations).await
    }

    async fn transfer(&mut self, read: &mut [Word], write: &[Word]) -> Result<(), Self::Error> {
        T::transfer(self, read, write).await
    }

    async fn transfer_in_place(&mut self, buf: &mut [Word]) -> Result<(), Self::Error> {
        T::transfer_in_place(self, buf).await
    }
}

/// Flush support for SPI bus
pub trait SpiBusFlush: ErrorType {
    /// Wait until all operations have completed and the bus is idle.
    ///
    /// See (the docs on embedded-hal)[embedded_hal::spi] for information on flushing.
    async fn flush(&mut self) -> Result<(), Self::Error>;
}

impl<T: SpiBusFlush> SpiBusFlush for &mut T {
    async fn flush(&mut self) -> Result<(), Self::Error> {
        T::flush(self).await
    }
}

/// Read-only SPI bus
pub trait SpiBusRead<Word: 'static + Copy = u8>: SpiBusFlush {
    /// Read `words` from the slave.
    ///
    /// The word value sent on MOSI during reading is implementation-defined,
    /// typically `0x00`, `0xFF`, or configurable.
    ///
    /// Implementations are allowed to return before the operation is
    /// complete. See (the docs on embedded-hal)[embedded_hal::spi] for details on flushing.
    async fn read(&mut self, words: &mut [Word]) -> Result<(), Self::Error>;
}

impl<T: SpiBusRead<Word>, Word: 'static + Copy> SpiBusRead<Word> for &mut T {
    async fn read(&mut self, words: &mut [Word]) -> Result<(), Self::Error> {
        T::read(self, words).await
    }
}

/// Write-only SPI
pub trait SpiBusWrite<Word: 'static + Copy = u8>: SpiBusFlush {
    /// Write `words` to the slave, ignoring all the incoming words
    ///
    /// Implementations are allowed to return before the operation is
    /// complete. See (the docs on embedded-hal)[embedded_hal::spi] for details on flushing.
    async fn write(&mut self, words: &[Word]) -> Result<(), Self::Error>;
}

impl<T: SpiBusWrite<Word>, Word: 'static + Copy> SpiBusWrite<Word> for &mut T {
    async fn write(&mut self, words: &[Word]) -> Result<(), Self::Error> {
        T::write(self, words).await
    }
}

/// Read-write SPI bus
///
/// `SpiBus` represents **exclusive ownership** over the whole SPI bus, with SCK, MOSI and MISO pins.
///
/// See (the docs on embedded-hal)[embedded_hal::spi] for important information on SPI Bus vs Device traits.
pub trait SpiBus<Word: 'static + Copy = u8>: SpiBusRead<Word> + SpiBusWrite<Word> {
    /// Write and read simultaneously. `write` is written to the slave on MOSI and
    /// words received on MISO are stored in `read`.
    ///
    /// It is allowed for `read` and `write` to have different lengths, even zero length.
    /// The transfer runs for `max(read.len(), write.len())` words. If `read` is shorter,
    /// incoming words after `read` has been filled will be discarded. If `write` is shorter,
    /// the value of words sent in MOSI after all `write` has been sent is implementation-defined,
    /// typically `0x00`, `0xFF`, or configurable.
    ///
    /// Implementations are allowed to return before the operation is
    /// complete. See (the docs on embedded-hal)[embedded_hal::spi] for details on flushing.
    async fn transfer<'a>(
        &'a mut self,
        read: &'a mut [Word],
        write: &'a [Word],
    ) -> Result<(), Self::Error>;

    /// Write and read simultaneously. The contents of `words` are
    /// written to the slave, and the received words are stored into the same
    /// `words` buffer, overwriting it.
    ///
    /// Implementations are allowed to return before the operation is
    /// complete. See (the docs on embedded-hal)[embedded_hal::spi] for details on flushing.
    async fn transfer_in_place<'a>(&'a mut self, words: &'a mut [Word]) -> Result<(), Self::Error>;
}

impl<T: SpiBus<Word>, Word: 'static + Copy> SpiBus<Word> for &mut T {
    async fn transfer<'a>(
        &'a mut self,
        read: &'a mut [Word],
        write: &'a [Word],
    ) -> Result<(), Self::Error> {
        T::transfer(self, read, write).await
    }

    async fn transfer_in_place<'a>(&'a mut self, words: &'a mut [Word]) -> Result<(), Self::Error> {
        T::transfer_in_place(self, words).await
    }
}

/// Error type for [`ExclusiveDevice`] operations.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ExclusiveDeviceError<BUS, CS> {
    /// An inner SPI bus operation failed
    Spi(BUS),
    /// Asserting or deasserting CS failed
    Cs(CS),
}

impl<BUS, CS> Error for ExclusiveDeviceError<BUS, CS>
where
    BUS: Error + Debug,
    CS: Debug,
{
    fn kind(&self) -> ErrorKind {
        match self {
            Self::Spi(e) => e.kind(),
            Self::Cs(_) => ErrorKind::ChipSelectFault,
        }
    }
}

/// [`SpiDevice`] implementation with exclusive access to the bus (not shared).
///
/// This is the most straightforward way of obtaining an [`SpiDevice`] from an [`SpiBus`],
/// ideal for when no sharing is required (only one SPI device is present on the bus).
pub struct ExclusiveDevice<BUS, CS> {
    bus: BUS,
    cs: CS,
}

impl<BUS, CS> ExclusiveDevice<BUS, CS> {
    /// Create a new ExclusiveDevice
    pub fn new(bus: BUS, cs: CS) -> Self {
        Self { bus, cs }
    }
}

impl<BUS, CS> ErrorType for ExclusiveDevice<BUS, CS>
where
    BUS: ErrorType,
    CS: OutputPin,
{
    type Error = ExclusiveDeviceError<BUS::Error, CS::Error>;
}

impl<Word: Copy + 'static, BUS, CS> blocking::SpiDeviceRead<Word> for ExclusiveDevice<BUS, CS>
where
    BUS: blocking::SpiBusRead<Word>,
    CS: OutputPin,
{
    fn read_transaction(&mut self, operations: &mut [&mut [Word]]) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(ExclusiveDeviceError::Cs)?;

        let mut op_res = Ok(());

        for buf in operations {
            if let Err(e) = self.bus.read(buf) {
                op_res = Err(e);
                break;
            }
        }

        // On failure, it's important to still flush and deassert CS.
        let flush_res = self.bus.flush();
        let cs_res = self.cs.set_high();

        op_res.map_err(ExclusiveDeviceError::Spi)?;
        flush_res.map_err(ExclusiveDeviceError::Spi)?;
        cs_res.map_err(ExclusiveDeviceError::Cs)?;

        Ok(())
    }
}

impl<Word: Copy + 'static, BUS, CS> blocking::SpiDeviceWrite<Word> for ExclusiveDevice<BUS, CS>
where
    BUS: blocking::SpiBusWrite<Word>,
    CS: OutputPin,
{
    fn write_transaction(&mut self, operations: &[&[Word]]) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(ExclusiveDeviceError::Cs)?;

        let mut op_res = Ok(());

        for buf in operations {
            if let Err(e) = self.bus.write(buf) {
                op_res = Err(e);
                break;
            }
        }

        // On failure, it's important to still flush and deassert CS.
        let flush_res = self.bus.flush();
        let cs_res = self.cs.set_high();

        op_res.map_err(ExclusiveDeviceError::Spi)?;
        flush_res.map_err(ExclusiveDeviceError::Spi)?;
        cs_res.map_err(ExclusiveDeviceError::Cs)?;

        Ok(())
    }
}

impl<Word: Copy + 'static, BUS, CS> blocking::SpiDevice<Word> for ExclusiveDevice<BUS, CS>
where
    BUS: blocking::SpiBus<Word>,
    CS: OutputPin,
{
    fn transaction(&mut self, operations: &mut [Operation<'_, Word>]) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(ExclusiveDeviceError::Cs)?;

        let op_res = 'ops: {
            for op in operations {
                let res = match op {
                    Operation::Read(buf) => self.bus.read(buf),
                    Operation::Write(buf) => self.bus.write(buf),
                    Operation::Transfer(read, write) => self.bus.transfer(read, write),
                    Operation::TransferInPlace(buf) => self.bus.transfer_in_place(buf),
                };
                if let Err(e) = res {
                    break 'ops Err(e);
                }
            }
            Ok(())
        };

        // On failure, it's important to still flush and deassert CS.
        let flush_res = self.bus.flush();
        let cs_res = self.cs.set_high();

        op_res.map_err(ExclusiveDeviceError::Spi)?;
        flush_res.map_err(ExclusiveDeviceError::Spi)?;
        cs_res.map_err(ExclusiveDeviceError::Cs)?;

        Ok(())
    }
}

impl<Word: Copy + 'static, BUS, CS> SpiDeviceRead<Word> for ExclusiveDevice<BUS, CS>
where
    BUS: SpiBusRead<Word>,
    CS: OutputPin,
{
    async fn read_transaction(
        &mut self,
        operations: &mut [&mut [Word]],
    ) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(ExclusiveDeviceError::Cs)?;

        let mut op_res = Ok(());

        for buf in operations {
            if let Err(e) = self.bus.read(buf).await {
                op_res = Err(e);
                break;
            }
        }

        // On failure, it's important to still flush and deassert CS.
        let flush_res = self.bus.flush().await;
        let cs_res = self.cs.set_high();

        op_res.map_err(ExclusiveDeviceError::Spi)?;
        flush_res.map_err(ExclusiveDeviceError::Spi)?;
        cs_res.map_err(ExclusiveDeviceError::Cs)?;

        Ok(())
    }
}

impl<Word: Copy + 'static, BUS, CS> SpiDeviceWrite<Word> for ExclusiveDevice<BUS, CS>
where
    BUS: SpiBusWrite<Word>,
    CS: OutputPin,
{
    async fn write_transaction(&mut self, operations: &[&[Word]]) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(ExclusiveDeviceError::Cs)?;

        let mut op_res = Ok(());

        for buf in operations {
            if let Err(e) = self.bus.write(buf).await {
                op_res = Err(e);
                break;
            }
        }

        // On failure, it's important to still flush and deassert CS.
        let flush_res = self.bus.flush().await;
        let cs_res = self.cs.set_high();

        op_res.map_err(ExclusiveDeviceError::Spi)?;
        flush_res.map_err(ExclusiveDeviceError::Spi)?;
        cs_res.map_err(ExclusiveDeviceError::Cs)?;

        Ok(())
    }
}

impl<Word: Copy + 'static, BUS, CS> SpiDevice<Word> for ExclusiveDevice<BUS, CS>
where
    BUS: SpiBus<Word>,
    CS: OutputPin,
{
    async fn transaction(
        &mut self,
        operations: &mut [Operation<'_, Word>],
    ) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(ExclusiveDeviceError::Cs)?;

        let op_res = 'ops: {
            for op in operations {
                let res = match op {
                    Operation::Read(buf) => self.bus.read(buf).await,
                    Operation::Write(buf) => self.bus.write(buf).await,
                    Operation::Transfer(read, write) => self.bus.transfer(read, write).await,
                    Operation::TransferInPlace(buf) => self.bus.transfer_in_place(buf).await,
                };
                if let Err(e) = res {
                    break 'ops Err(e);
                }
            }
            Ok(())
        };

        // On failure, it's important to still flush and deassert CS.
        let flush_res = self.bus.flush().await;
        let cs_res = self.cs.set_high();

        op_res.map_err(ExclusiveDeviceError::Spi)?;
        flush_res.map_err(ExclusiveDeviceError::Spi)?;
        cs_res.map_err(ExclusiveDeviceError::Cs)?;

        Ok(())
    }
}
