use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::{ErrorType, Operation, SpiBus, SpiDevice};
use mutex::{BlockingMutex, RawMutex};

use super::DeviceError;
use crate::spi::shared::transaction;

type Mutex<R, T> = BlockingMutex<R, T>;

/// `mutex-traits`-based shared bus [`SpiDevice`] implementation.
///
/// This allows for sharing an [`SpiBus`], obtaining multiple [`SpiDevice`] instances,
/// each with its own `CS` pin.
///
/// Whether a single bus can be used across multiple threads depends on which
/// implementation of `RawMutex` is used.
pub struct MutexTraitsDevice<'a, R, BUS, CS, D> {
    bus: &'a Mutex<R, BUS>,
    cs: CS,
    delay: D,
}

impl<'a, R: RawMutex, BUS, CS, D> MutexTraitsDevice<'a, R, BUS, CS, D> {
    /// Create a new [`MutexTraitsDevice`].
    ///
    /// This sets the `cs` pin high, and returns an error if that fails. It is recommended
    /// to set the pin high the moment it's configured as an output, to avoid glitches.
    #[inline]
    pub fn new(bus: &'a Mutex<R, BUS>, mut cs: CS, delay: D) -> Result<Self, CS::Error>
    where
        CS: OutputPin,
    {
        cs.set_high()?;
        Ok(Self { bus, cs, delay })
    }
}

impl<'a, R: RawMutex, BUS, CS> MutexTraitsDevice<'a, R, BUS, CS, super::NoDelay> {
    /// Create a new [`MutexTraitsDevice`] without support for in-transaction delays.
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
    pub fn new_no_delay(bus: &'a Mutex<R, BUS>, mut cs: CS) -> Result<Self, CS::Error>
    where
        CS: OutputPin,
    {
        cs.set_high()?;
        Ok(Self {
            bus,
            cs,
            delay: super::NoDelay,
        })
    }
}

impl<R, BUS, CS, D> ErrorType for MutexTraitsDevice<'_, R, BUS, CS, D>
where
    R: RawMutex,
    BUS: ErrorType,
    CS: OutputPin,
{
    type Error = DeviceError<BUS::Error, CS::Error>;
}

impl<Word: Copy + 'static, R, BUS, CS, D> SpiDevice<Word> for MutexTraitsDevice<'_, R, BUS, CS, D>
where
    R: RawMutex,
    BUS: SpiBus<Word>,
    CS: OutputPin,
    D: DelayNs,
{
    #[inline]
    fn transaction(&mut self, operations: &mut [Operation<'_, Word>]) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.lock();

        transaction(operations, bus, &mut self.delay, &mut self.cs)
    }
}
