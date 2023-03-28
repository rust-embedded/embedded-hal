use core::cell::RefCell;
use critical_section::Mutex;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::{
    ErrorType, Operation, SpiBus, SpiBusRead, SpiBusWrite, SpiDevice, SpiDeviceRead, SpiDeviceWrite,
};

use super::DeviceError;

/// `critical-section`-based shared bus [`SpiDevice`] implementation.
///
/// This allows for sharing an [`SpiBus`](embedded_hal::spi::SpiBus), obtaining multiple [`SpiDevice`] instances,
/// each with its own `CS` pin.
///
/// Sharing is implemented with a `critical-section` [`Mutex`](critical_section::Mutex). A critical section is taken for
/// the entire duration of a transaction. This allows sharing a single bus across multiple threads (interrupt priority levels).
/// The downside is critical sections typically require globally disabling interrupts, so `CriticalSectionDevice` will likely
/// negatively impact real-time properties, such as interrupt latency. If you can, prefer using
/// [`RefCellDevice`](super::RefCellDevice) instead, which does not require taking critical sections.
pub struct CriticalSectionDevice<'a, BUS, CS> {
    bus: &'a Mutex<RefCell<BUS>>,
    cs: CS,
}

impl<'a, BUS, CS> CriticalSectionDevice<'a, BUS, CS> {
    /// Create a new ExclusiveDevice
    pub fn new(bus: &'a Mutex<RefCell<BUS>>, cs: CS) -> Self {
        Self { bus, cs }
    }
}

impl<'a, BUS, CS> ErrorType for CriticalSectionDevice<'a, BUS, CS>
where
    BUS: ErrorType,
    CS: OutputPin,
{
    type Error = DeviceError<BUS::Error, CS::Error>;
}

impl<'a, Word: Copy + 'static, BUS, CS> SpiDeviceRead<Word> for CriticalSectionDevice<'a, BUS, CS>
where
    BUS: SpiBusRead<Word>,
    CS: OutputPin,
{
    fn read_transaction(&mut self, operations: &mut [&mut [Word]]) -> Result<(), Self::Error> {
        critical_section::with(|cs| {
            let bus = &mut *self.bus.borrow_ref_mut(cs);

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
        })
    }
}

impl<'a, Word: Copy + 'static, BUS, CS> SpiDeviceWrite<Word> for CriticalSectionDevice<'a, BUS, CS>
where
    BUS: SpiBusWrite<Word>,
    CS: OutputPin,
{
    fn write_transaction(&mut self, operations: &[&[Word]]) -> Result<(), Self::Error> {
        critical_section::with(|cs| {
            let bus = &mut *self.bus.borrow_ref_mut(cs);

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
        })
    }
}

impl<'a, Word: Copy + 'static, BUS, CS> SpiDevice<Word> for CriticalSectionDevice<'a, BUS, CS>
where
    BUS: SpiBus<Word>,
    CS: OutputPin,
{
    fn transaction(&mut self, operations: &mut [Operation<'_, Word>]) -> Result<(), Self::Error> {
        critical_section::with(|cs| {
            let bus = &mut *self.bus.borrow_ref_mut(cs);

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
        })
    }
}
