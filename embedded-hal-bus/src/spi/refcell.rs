use core::cell::RefCell;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::{ErrorType, Operation, SpiBus, SpiDevice};

use super::DeviceError;
use crate::spi::shared::transaction;

/// `RefCell`-based shared bus [`SpiDevice`] implementation.
///
/// This allows for sharing an [`SpiBus`], obtaining multiple [`SpiDevice`] instances,
/// each with its own `CS` pin.
///
/// Sharing is implemented with a `RefCell`. This means it has low overhead, but `RefCellDevice` instances are not `Send`,
/// so it only allows sharing within a single thread (interrupt priority level). If you need to share a bus across several
/// threads, use [`CriticalSectionDevice`](super::CriticalSectionDevice) instead.
pub struct RefCellDevice<'a, BUS, CS, D> {
    bus: &'a RefCell<BUS>,
    cs: CS,
    delay: D,
    /// Implementation of <https://docs.rs/embedded-hal/latest/embedded_hal/spi/index.html#cs-to-clock-delays>
    cs_to_clock_delay_ns: u32,
    clock_to_cs_delay_ns: u32,
}

impl<'a, BUS, CS, D> RefCellDevice<'a, BUS, CS, D> {
    /// Create a new [`RefCellDevice`].
    ///
    /// This sets the `cs` pin high, and returns an error if that fails. It is recommended
    /// to set the pin high the moment it's configured as an output, to avoid glitches.
    #[inline]
    pub fn new(bus: &'a RefCell<BUS>, mut cs: CS, delay: D) -> Result<Self, CS::Error>
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
}

impl<'a, BUS, CS> RefCellDevice<'a, BUS, CS, super::NoDelay> {
    /// Create a new [`RefCellDevice`] without support for in-transaction delays.
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
    pub fn new_no_delay(bus: &'a RefCell<BUS>, mut cs: CS) -> Result<Self, CS::Error>
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

impl<BUS, CS, D> ErrorType for RefCellDevice<'_, BUS, CS, D>
where
    BUS: ErrorType,
    CS: OutputPin,
{
    type Error = DeviceError<BUS::Error, CS::Error>;
}

impl<Word: Copy + 'static, BUS, CS, D> SpiDevice<Word> for RefCellDevice<'_, BUS, CS, D>
where
    BUS: SpiBus<Word>,
    CS: OutputPin,
    D: DelayNs,
{
    #[inline]
    fn transaction(&mut self, operations: &mut [Operation<'_, Word>]) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.borrow_mut();

        transaction(
            operations,
            bus,
            &mut self.delay,
            &mut self.cs,
            self.cs_to_clock_delay_ns,
            self.clock_to_cs_delay_ns,
        )
    }
}
