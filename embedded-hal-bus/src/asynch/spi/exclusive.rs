use embedded_hal::{digital::OutputPin, spi as blocking};
use embedded_hal_async::{
    delay::DelayUs,
    spi::{ErrorType, Operation, SpiBus, SpiDevice},
};

pub use crate::spi::DeviceError;

/// [`SpiDevice`] implementation with exclusive access to the bus (not shared).
///
/// This is the most straightforward way of obtaining an [`SpiDevice`] from an [`SpiBus`],
/// ideal for when no sharing is required (only one SPI device is present on the bus).
pub struct ExclusiveDevice<BUS, CS, D> {
    bus: BUS,
    cs: CS,
    delay: D,
}

impl<BUS, CS, D> ExclusiveDevice<BUS, CS, D> {
    /// Create a new ExclusiveDevice
    pub fn new(bus: BUS, cs: CS, delay: D) -> Self {
        Self { bus, cs, delay }
    }

    /// Returns a reference to the underlying bus object.
    pub fn bus(&self) -> &BUS {
        &self.bus
    }

    /// Returns a mutable reference to the underlying bus object.
    pub fn bus_mut(&mut self) -> &mut BUS {
        &mut self.bus
    }
}

impl<BUS, CS> ExclusiveDevice<BUS, CS, super::NoDelay> {
    /// Create a new ExclusiveDevice without support for in-transaction delays.
    ///
    /// # Panics
    ///
    /// The returned device will panic if you try to execute a transaction
    /// that contains any operations of type `Operation::DelayUs`.
    pub fn new_no_delay(bus: BUS, cs: CS) -> Self {
        Self {
            bus,
            cs,
            delay: super::NoDelay,
        }
    }
}

impl<BUS, CS, D> ErrorType for ExclusiveDevice<BUS, CS, D>
where
    BUS: ErrorType,
    CS: OutputPin,
{
    type Error = DeviceError<BUS::Error, CS::Error>;
}

impl<Word: Copy + 'static, BUS, CS, D> blocking::SpiDevice<Word> for ExclusiveDevice<BUS, CS, D>
where
    BUS: blocking::SpiBus<Word>,
    CS: OutputPin,
    D: embedded_hal::delay::DelayUs,
{
    fn transaction(&mut self, operations: &mut [Operation<'_, Word>]) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(DeviceError::Cs)?;

        let op_res = 'ops: {
            for op in operations {
                let res = match op {
                    Operation::Read(buf) => self.bus.read(buf),
                    Operation::Write(buf) => self.bus.write(buf),
                    Operation::Transfer(read, write) => self.bus.transfer(read, write),
                    Operation::TransferInPlace(buf) => self.bus.transfer_in_place(buf),
                    Operation::DelayUs(us) => match self.bus.flush() {
                        Err(e) => Err(e),
                        Ok(()) => {
                            self.delay.delay_us(*us);
                            Ok(())
                        }
                    },
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

        op_res.map_err(DeviceError::Spi)?;
        flush_res.map_err(DeviceError::Spi)?;
        cs_res.map_err(DeviceError::Cs)?;

        Ok(())
    }
}

impl<Word: Copy + 'static, BUS, CS, D> SpiDevice<Word> for ExclusiveDevice<BUS, CS, D>
where
    BUS: SpiBus<Word>,
    CS: OutputPin,
    D: DelayUs,
{
    async fn transaction(
        &mut self,
        operations: &mut [Operation<'_, Word>],
    ) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(DeviceError::Cs)?;

        let op_res = 'ops: {
            for op in operations {
                let res = match op {
                    Operation::Read(buf) => self.bus.read(buf).await,
                    Operation::Write(buf) => self.bus.write(buf).await,
                    Operation::Transfer(read, write) => self.bus.transfer(read, write).await,
                    Operation::TransferInPlace(buf) => self.bus.transfer_in_place(buf).await,
                    Operation::DelayUs(us) => match self.bus.flush().await {
                        Err(e) => Err(e),
                        Ok(()) => {
                            self.delay.delay_us(*us).await;
                            Ok(())
                        }
                    },
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

        op_res.map_err(DeviceError::Spi)?;
        flush_res.map_err(DeviceError::Spi)?;
        cs_res.map_err(DeviceError::Cs)?;

        Ok(())
    }
}
