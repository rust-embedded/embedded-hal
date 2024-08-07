//! SPI bus sharing mechanisms.

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::{ErrorType, Operation, SpiBus, SpiDevice};
#[cfg(feature = "async")]
use embedded_hal_async::{
    delay::DelayNs as AsyncDelayNs,
    spi::{SpiBus as AsyncSpiBus, SpiDevice as AsyncSpiDevice},
};

use super::shared::transaction;
use super::DeviceError;

/// [`SpiDevice`] implementation with exclusive access to the bus (not shared).
///
/// This is the most straightforward way of obtaining an [`SpiDevice`] from an [`SpiBus`],
/// ideal for when no sharing is required (only one SPI device is present on the bus).
pub struct ExclusiveDevice<BUS, CS, D> {
    bus: BUS,
    cs: CS,
    delay: D,
    /// Implementation of <https://docs.rs/embedded-hal/latest/embedded_hal/spi/index.html#cs-to-clock-delays>
    cs_to_clock_delay_ns: u32,
    clock_to_cs_delay_ns: u32,
}

impl<BUS, CS, D> ExclusiveDevice<BUS, CS, D> {
    /// Create a new [`ExclusiveDevice`].
    ///
    /// This sets the `cs` pin high, and returns an error if that fails. It is recommended
    /// to set the pin high the moment it's configured as an output, to avoid glitches.
    #[inline]
    pub fn new(bus: BUS, mut cs: CS, delay: D) -> Result<Self, CS::Error>
    where
        CS: OutputPin,
    {
        cs.set_high()?;
        Ok(Self {
            bus,
            cs,
            delay,
            cs_to_clock_delay_ns: 0,
            clock_to_cs_delay_ns: 0,
        })
    }

    /// Set the delay between the CS pin toggle and the first clock
    pub fn set_cs_to_clock_delay_ns(&mut self, delay_ns: u32) {
        self.cs_to_clock_delay_ns = delay_ns;
    }

    /// Set the delay between the last clock and the CS pin reset
    pub fn set_clock_to_cs_delay_ns(&mut self, delay_ns: u32) {
        self.clock_to_cs_delay_ns = delay_ns;
    }

    /// Returns a reference to the underlying bus object.
    #[inline]
    pub fn bus(&self) -> &BUS {
        &self.bus
    }

    /// Returns a mutable reference to the underlying bus object.
    #[inline]
    pub fn bus_mut(&mut self) -> &mut BUS {
        &mut self.bus
    }
}

impl<BUS, CS> ExclusiveDevice<BUS, CS, super::NoDelay> {
    /// Create a new [`ExclusiveDevice`] without support for in-transaction delays.
    ///
    /// This sets the `cs` pin high, and returns an error if that fails. It is recommended
    /// to set the pin high the moment it's configured as an output, to avoid glitches.
    ///
    /// **Warning**: The returned instance *technically* doesn't comply with the `SpiDevice`
    /// contract, which mandates delay support. It is relatively rare for drivers to use
    /// in-transaction delays, so you might still want to use this method because it's more practical.
    ///
    /// Note that a future version of the driver might start using delays, causing your
    /// code to panic. This wouldn't be considered a breaking change from the driver side, because
    /// drivers are allowed to assume `SpiDevice` implementations comply with the contract.
    /// If you feel this risk outweighs the convenience of having `cargo` automatically upgrade
    /// the driver crate, you might want to pin the driver's version.
    ///
    /// # Panics
    ///
    /// The returned device will panic if you try to execute a transaction
    /// that contains any operations of type [`Operation::DelayNs`].
    #[inline]
    pub fn new_no_delay(bus: BUS, mut cs: CS) -> Result<Self, CS::Error>
    where
        CS: OutputPin,
    {
        cs.set_high()?;
        Ok(Self {
            bus,
            cs,
            delay: super::NoDelay,
            cs_to_clock_delay_ns: 0,
            clock_to_cs_delay_ns: 0,
        })
    }
}

impl<BUS, CS, D> ErrorType for ExclusiveDevice<BUS, CS, D>
where
    BUS: ErrorType,
    CS: OutputPin,
{
    type Error = DeviceError<BUS::Error, CS::Error>;
}

impl<Word: Copy + 'static, BUS, CS, D> SpiDevice<Word> for ExclusiveDevice<BUS, CS, D>
where
    BUS: SpiBus<Word>,
    CS: OutputPin,
    D: DelayNs,
{
    #[inline]
    fn transaction(&mut self, operations: &mut [Operation<'_, Word>]) -> Result<(), Self::Error> {
        transaction(
            operations,
            &mut self.bus,
            &mut self.delay,
            &mut self.cs,
            self.cs_to_clock_delay_ns,
            self.clock_to_cs_delay_ns,
        )
    }
}

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
impl<Word: Copy + 'static, BUS, CS, D> AsyncSpiDevice<Word> for ExclusiveDevice<BUS, CS, D>
where
    BUS: AsyncSpiBus<Word>,
    CS: OutputPin,
    D: AsyncDelayNs,
{
    #[inline]
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
                    Operation::DelayNs(ns) => match self.bus.flush().await {
                        Err(e) => Err(e),
                        Ok(()) => {
                            self.delay.delay_ns(*ns).await;
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
