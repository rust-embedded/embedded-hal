use embedded_hal::delay::DelayUs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::{ErrorType, Operation, SpiBus, SpiDevice};
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
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub struct MutexDevice<'a, BUS, CS, D> {
    bus: &'a Mutex<BUS>,
    cs: CS,
    delay: D,
}

impl<'a, BUS, CS, D> MutexDevice<'a, BUS, CS, D> {
    /// Create a new ExclusiveDevice
    pub fn new(bus: &'a Mutex<BUS>, cs: CS, delay: D) -> Self {
        Self { bus, cs, delay }
    }
}

impl<'a, BUS, CS> MutexDevice<'a, BUS, CS, super::NoDelay> {
    /// Create a new MutexDevice without support for in-transaction delays.
    ///
    /// # Panics
    ///
    /// The returned device will panic if you try to execute a transaction
    /// that contains any operations of type `Operation::DelayUs`.
    pub fn new_no_delay(bus: &'a Mutex<BUS>, cs: CS) -> Self {
        Self {
            bus,
            cs,
            delay: super::NoDelay,
        }
    }
}

impl<'a, BUS, CS, D> ErrorType for MutexDevice<'a, BUS, CS, D>
where
    BUS: ErrorType,
    CS: OutputPin,
{
    type Error = DeviceError<BUS::Error, CS::Error>;
}

impl<'a, Word: Copy + 'static, BUS, CS, D> SpiDevice<Word> for MutexDevice<'a, BUS, CS, D>
where
    BUS: SpiBus<Word>,
    CS: OutputPin,
    D: DelayUs,
{
    fn transaction(&mut self, operations: &mut [Operation<'_, Word>]) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.lock().unwrap();

        self.cs.set_low().map_err(DeviceError::Cs)?;

        let op_res = operations.iter_mut().try_for_each(|op| match op {
            Operation::Read(buf) => bus.read(buf),
            Operation::Write(buf) => bus.write(buf),
            Operation::Transfer(read, write) => bus.transfer(read, write),
            Operation::TransferInPlace(buf) => bus.transfer_in_place(buf),
            Operation::DelayUs(us) => {
                bus.flush()?;
                self.delay.delay_us(*us);
                Ok(())
            }
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
