use core::convert::Infallible;

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::{ErrorType, Operation, SpiBus, SpiDevice};
use std::sync::Mutex;

use crate::spi::shared::transaction;

/// `std` `Mutex`-based shared bus [`SpiDevice`] implementation.
///
/// This allows for sharing an [`SpiBus`], obtaining multiple [`SpiDevice`] instances,
/// each with its own `CS` pin.
///
/// Sharing is implemented with a `std` [`Mutex`]. It allows a single bus across multiple threads,
/// with finer-grained locking than [`CriticalSectionDevice`](super::CriticalSectionDevice). The downside is
/// it is only available in `std` targets.
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub struct MutexDevice<'a, BUS, CS, D> {
    bus: &'a Mutex<BUS>,
    cs: CS,
    delay: D,
}

impl<'a, BUS, CS, D> MutexDevice<'a, BUS, CS, D> {
    /// Create a new [`MutexDevice`].
    #[inline]
    pub fn new(bus: &'a Mutex<BUS>, cs: CS, delay: D) -> Self {
        Self { bus, cs, delay }
    }
}

impl<'a, BUS, CS> MutexDevice<'a, BUS, CS, super::NoDelay> {
    /// Create a new [`MutexDevice`] without support for in-transaction delays.
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
    CS: OutputPin<Error = Infallible>,
{
    type Error = BUS::Error;
}

impl<'a, Word: Copy + 'static, BUS, CS, D> SpiDevice<Word> for MutexDevice<'a, BUS, CS, D>
where
    BUS: SpiBus<Word>,
    CS: OutputPin<Error = Infallible>,
    D: DelayNs,
{
    #[inline]
    fn transaction(&mut self, operations: &mut [Operation<'_, Word>]) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.lock().unwrap();

        transaction(operations, bus, &mut self.delay, &mut self.cs)
    }
}
