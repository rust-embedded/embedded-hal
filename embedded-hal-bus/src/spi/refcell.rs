use core::cell::RefCell;
use embedded_hal::delay::DelayUs;
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
}

impl<'a, BUS, CS, D> RefCellDevice<'a, BUS, CS, D> {
    /// Create a new [`RefCellDevice`].
    #[inline]
    pub fn new(bus: &'a RefCell<BUS>, cs: CS, delay: D) -> Self {
        Self { bus, cs, delay }
    }
}

impl<'a, BUS, CS> RefCellDevice<'a, BUS, CS, super::NoDelay> {
    /// Create a new [`RefCellDevice`] without support for in-transaction delays.
    ///
    /// # Panics
    ///
    /// The returned device will panic if you try to execute a transaction
    /// that contains any operations of type `Operation::DelayUs`.
    #[inline]
    pub fn new_no_delay(bus: &'a RefCell<BUS>, cs: CS) -> Self {
        Self {
            bus,
            cs,
            delay: super::NoDelay,
        }
    }
}

impl<'a, BUS, CS, D> ErrorType for RefCellDevice<'a, BUS, CS, D>
where
    BUS: ErrorType,
    CS: OutputPin,
{
    type Error = DeviceError<BUS::Error, CS::Error>;
}

impl<'a, Word: Copy + 'static, BUS, CS, D> SpiDevice<Word> for RefCellDevice<'a, BUS, CS, D>
where
    BUS: SpiBus<Word>,
    CS: OutputPin,
    D: DelayUs,
{
    #[inline]
    fn transaction(&mut self, operations: &mut [Operation<'_, Word>]) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.borrow_mut();

        transaction(operations, bus, &mut self.delay, &mut self.cs)
    }
}
