use core::cell::RefCell;
use critical_section::Mutex;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::{ErrorType, Operation, SpiBus, SpiDevice};

use super::DeviceError;
use crate::spi::shared::transaction;

/// `critical-section`-based shared bus [`SpiDevice`] implementation.
///
/// This allows for sharing an [`SpiBus`], obtaining multiple [`SpiDevice`] instances,
/// each with its own `CS` pin.
///
/// Sharing is implemented with a `critical-section` [`Mutex`]. A critical section is taken for
/// the entire duration of a transaction. This allows sharing a single bus across multiple threads (interrupt priority levels).
/// The downside is critical sections typically require globally disabling interrupts, so `CriticalSectionDevice` will likely
/// negatively impact real-time properties, such as interrupt latency. If you can, prefer using
/// [`RefCellDevice`](super::RefCellDevice) instead, which does not require taking critical sections.
pub struct CriticalSectionDevice<'a, BUS, CS, D> {
    bus: &'a Mutex<RefCell<BUS>>,
    cs: CS,
    delay: D,
}

impl<'a, BUS, CS, D> CriticalSectionDevice<'a, BUS, CS, D> {
    /// Create a new [`CriticalSectionDevice`].
    #[inline]
    pub fn new(bus: &'a Mutex<RefCell<BUS>>, cs: CS, delay: D) -> Self {
        Self { bus, cs, delay }
    }
}

impl<'a, BUS, CS> CriticalSectionDevice<'a, BUS, CS, super::NoDelay> {
    /// Create a new [`CriticalSectionDevice`] without support for in-transaction delays.
    ///
    /// # Panics
    ///
    /// The returned device will panic if you try to execute a transaction
    /// that contains any operations of type [`Operation::DelayNs`].
    #[inline]
    pub fn new_no_delay(bus: &'a Mutex<RefCell<BUS>>, cs: CS) -> Self {
        Self {
            bus,
            cs,
            delay: super::NoDelay,
        }
    }
}

impl<'a, BUS, CS, D> ErrorType for CriticalSectionDevice<'a, BUS, CS, D>
where
    BUS: ErrorType,
    CS: OutputPin,
{
    type Error = DeviceError<BUS::Error, CS::Error>;
}

impl<'a, Word: Copy + 'static, BUS, CS, D> SpiDevice<Word> for CriticalSectionDevice<'a, BUS, CS, D>
where
    BUS: SpiBus<Word>,
    CS: OutputPin,
    D: DelayNs,
{
    #[inline]
    fn transaction(&mut self, operations: &mut [Operation<'_, Word>]) -> Result<(), Self::Error> {
        critical_section::with(|cs| {
            let bus = &mut *self.bus.borrow_ref_mut(cs);

            transaction(operations, bus, &mut self.delay, &mut self.cs)
        })
    }
}
