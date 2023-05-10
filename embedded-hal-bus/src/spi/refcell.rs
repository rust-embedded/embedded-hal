use core::cell::RefCell;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::{ErrorType, Operation, SpiBus, SpiDevice};

use super::DeviceError;

/// `RefCell`-based shared bus [`SpiDevice`] implementation.
///
/// This allows for sharing an [`SpiBus`](embedded_hal::spi::SpiBus), obtaining multiple [`SpiDevice`] instances,
/// each with its own `CS` pin.
///
/// Sharing is implemented with a `RefCell`. This means it has low overhead, but `RefCellDevice` instances are not `Send`,
/// so it only allows sharing within a single thread (interrupt priority level). If you need to share a bus across several
/// threads, use [`CriticalSectionDevice`](super::CriticalSectionDevice) instead.
pub struct RefCellDevice<'a, BUS, CS> {
    bus: &'a RefCell<BUS>,
    cs: CS,
}

impl<'a, BUS, CS> RefCellDevice<'a, BUS, CS> {
    /// Create a new ExclusiveDevice
    pub fn new(bus: &'a RefCell<BUS>, cs: CS) -> Self {
        Self { bus, cs }
    }
}

impl<'a, BUS, CS> ErrorType for RefCellDevice<'a, BUS, CS>
where
    BUS: ErrorType,
    CS: OutputPin,
{
    type Error = DeviceError<BUS::Error, CS::Error>;
}

impl<'a, Word: Copy + 'static, BUS, CS> SpiDevice<Word> for RefCellDevice<'a, BUS, CS>
where
    BUS: SpiBus<Word>,
    CS: OutputPin,
{
    fn transaction(&mut self, operations: &mut [Operation<'_, Word>]) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.borrow_mut();

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
