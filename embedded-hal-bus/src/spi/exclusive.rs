//! SPI bus sharing mechanisms.

use embedded_hal::digital::OutputPin;
use embedded_hal::spi::{
    ErrorType, Operation, SpiBus, SpiBusRead, SpiBusWrite, SpiDevice, SpiDeviceRead, SpiDeviceWrite,
};

use super::DeviceError;

/// [`SpiDevice`] implementation with exclusive access to the bus (not shared).
///
/// This is the most straightforward way of obtaining an [`SpiDevice`] from an [`SpiBus`](embedded_hal::spi::SpiBus),
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
    type Error = DeviceError<BUS::Error, CS::Error>;
}

impl<Word: Copy + 'static, BUS, CS> SpiDeviceRead<Word> for ExclusiveDevice<BUS, CS>
where
    BUS: SpiBusRead<Word>,
    CS: OutputPin,
{
    fn read_transaction(&mut self, operations: &mut [&mut [Word]]) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(DeviceError::Cs)?;

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

        op_res.map_err(DeviceError::Spi)?;
        flush_res.map_err(DeviceError::Spi)?;
        cs_res.map_err(DeviceError::Cs)?;

        Ok(())
    }
}

impl<Word: Copy + 'static, BUS, CS> SpiDeviceWrite<Word> for ExclusiveDevice<BUS, CS>
where
    BUS: SpiBusWrite<Word>,
    CS: OutputPin,
{
    fn write_transaction(&mut self, operations: &[&[Word]]) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(DeviceError::Cs)?;

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

        op_res.map_err(DeviceError::Spi)?;
        flush_res.map_err(DeviceError::Spi)?;
        cs_res.map_err(DeviceError::Cs)?;

        Ok(())
    }
}

impl<Word: Copy + 'static, BUS, CS> SpiDevice<Word> for ExclusiveDevice<BUS, CS>
where
    BUS: SpiBus<Word>,
    CS: OutputPin,
{
    fn transaction(&mut self, operations: &mut [Operation<'_, Word>]) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(DeviceError::Cs)?;

        let op_res = operations.iter_mut().try_for_each(|op| match op {
            Operation::Read(buf) => self.bus.read(buf),
            Operation::Write(buf) => self.bus.write(buf),
            Operation::Transfer(read, write) => self.bus.transfer(read, write),
            Operation::TransferInPlace(buf) => self.bus.transfer_in_place(buf),
        });

        // On failure, it's important to still flush and deassert CS.
        let flush_res = self.bus.flush();
        let cs_res = self.cs.set_high();

        op_res.map_err(DeviceError::Spi)?;
        flush_res.map_err(DeviceError::Spi)?;
        cs_res.map_err(DeviceError::Cs)?;

        Ok(())
    }
}
