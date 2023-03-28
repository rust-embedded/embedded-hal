use embedded_hal::digital::OutputPin;
use embedded_hal::spi::{
    ErrorType, Operation, SpiBus, SpiBusRead, SpiBusWrite, SpiDevice, SpiDeviceRead, SpiDeviceWrite,
};
use std::sync::Mutex;

use super::DeviceError;

/// `std` `Mutex`-based shared bus [`SpiDevice`] implementation.
///
/// This allows for sharing an [`SpiBus`](embedded_hal::spi::SpiBus), obtaining multiple [`SpiDevice`] instances,
/// each with its own `CS` pin.
///
/// Sharing is implemented with a `std` [`Mutex`](std::sync::Mutex). It allows a single bus across multiple threads,
/// with finer-grained locking than [`CriticalSectionDevice`](super::CriticalSectionDevice). The downside is
/// it is only available in `std` targets.
pub struct MutexDevice<'a, BUS, CS> {
    bus: &'a Mutex<BUS>,
    cs: CS,
}

impl<'a, BUS, CS> MutexDevice<'a, BUS, CS> {
    /// Create a new ExclusiveDevice
    pub fn new(bus: &'a Mutex<BUS>, cs: CS) -> Self {
        Self { bus, cs }
    }
}

impl<'a, BUS, CS> ErrorType for MutexDevice<'a, BUS, CS>
where
    BUS: ErrorType,
    CS: OutputPin,
{
    type Error = DeviceError<BUS::Error, CS::Error>;
}

impl<'a, Word: Copy + 'static, BUS, CS> SpiDeviceRead<Word> for MutexDevice<'a, BUS, CS>
where
    BUS: SpiBusRead<Word>,
    CS: OutputPin,
{
    fn read_transaction(&mut self, operations: &mut [&mut [Word]]) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.lock().unwrap();

        self.cs.set_low().map_err(DeviceError::Cs)?;

        let mut op_res = Ok(());
        for buf in operations {
            if let Err(e) = bus.read(buf) {
                op_res = Err(e);
                break;
            }
        }

        // On failure, it's important to still flush and deassert CS.
        let flush_res = bus.flush();
        let cs_res = self.cs.set_high();

        op_res.map_err(DeviceError::Spi)?;
        flush_res.map_err(DeviceError::Spi)?;
        cs_res.map_err(DeviceError::Cs)?;

        Ok(())
    }
}

impl<'a, Word: Copy + 'static, BUS, CS> SpiDeviceWrite<Word> for MutexDevice<'a, BUS, CS>
where
    BUS: SpiBusWrite<Word>,
    CS: OutputPin,
{
    fn write_transaction(&mut self, operations: &[&[Word]]) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.lock().unwrap();

        self.cs.set_low().map_err(DeviceError::Cs)?;

        let mut op_res = Ok(());
        for buf in operations {
            if let Err(e) = bus.write(buf) {
                op_res = Err(e);
                break;
            }
        }

        // On failure, it's important to still flush and deassert CS.
        let flush_res = bus.flush();
        let cs_res = self.cs.set_high();

        op_res.map_err(DeviceError::Spi)?;
        flush_res.map_err(DeviceError::Spi)?;
        cs_res.map_err(DeviceError::Cs)?;

        Ok(())
    }
}

impl<'a, Word: Copy + 'static, BUS, CS> SpiDevice<Word> for MutexDevice<'a, BUS, CS>
where
    BUS: SpiBus<Word>,
    CS: OutputPin,
{
    fn transaction(&mut self, operations: &mut [Operation<'_, Word>]) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.lock().unwrap();

        self.cs.set_low().map_err(DeviceError::Cs)?;

        let op_res = operations.iter_mut().try_for_each(|op| match op {
            Operation::Read(buf) => bus.read(buf),
            Operation::Write(buf) => bus.write(buf),
            Operation::Transfer(read, write) => bus.transfer(read, write),
            Operation::TransferInPlace(buf) => bus.transfer_in_place(buf),
        });

        // On failure, it's important to still flush and deassert CS.
        let flush_res = bus.flush();
        let cs_res = self.cs.set_high();

        op_res.map_err(DeviceError::Spi)?;
        flush_res.map_err(DeviceError::Spi)?;
        cs_res.map_err(DeviceError::Cs)?;

        Ok(())
    }
}
