use core::cell::UnsafeCell;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::{Error, ErrorKind, ErrorType, Operation, SpiBus, SpiDevice};

use super::DeviceError;
use crate::spi::shared::transaction;

/// `UnsafeCell`-based shared bus [`SpiDevice`] implementation.
///
/// This allows for sharing an [`SpiBus`], obtaining multiple [`SpiDevice`] instances,
/// each with its own `CS` pin.
///
/// Sharing is implemented with a `UnsafeCell`. This means it has low overhead, and, unlike [`crate::spi::RefCellDevice`], instances are `Send`,
/// so it only allows sharing across multiple threads (interrupt priority level). When attempting
/// to preempt usage of the bus, a `AtomicError::Busy` error is returned.
///
/// This primitive is particularly well-suited for applications that have external arbitration
/// rules, such as the RTIC framework.
///
pub struct AtomicDevice<'a, BUS, CS, D> {
    bus: &'a UnsafeCell<BUS>,
    cs: CS,
    delay: D,
    busy: portable_atomic::AtomicBool,
}

#[derive(Debug, Copy, Clone)]
/// Wrapper type for errors originating from the atomically-checked SPI bus manager.
pub enum AtomicError<T: Error> {
    /// This error is returned if the SPI bus was already in use when an operation was attempted,
    /// which indicates that the driver requirements are not being met with regard to
    /// synchronization.
    Busy,

    /// An SPI-related error occurred, and the internal error should be inspected.
    Other(T),
}

impl<'a, BUS, CS, D> AtomicDevice<'a, BUS, CS, D> {
    /// Create a new [`AtomicDevice`].
    #[inline]
    pub fn new(bus: &'a UnsafeCell<BUS>, cs: CS, delay: D) -> Self {
        Self {
            bus,
            cs,
            delay,
            busy: portable_atomic::AtomicBool::from(false),
        }
    }
}

impl<'a, BUS, CS> AtomicDevice<'a, BUS, CS, super::NoDelay>
where
    BUS: ErrorType,
    CS: OutputPin,
{
    /// Create a new [`AtomicDevice`] without support for in-transaction delays.
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
    pub fn new_no_delay(bus: &'a UnsafeCell<BUS>, cs: CS) -> Self {
        Self {
            bus,
            cs,
            delay: super::NoDelay,
            busy: portable_atomic::AtomicBool::from(false),
        }
    }
}

unsafe impl<'a, BUS, CS, D> Send for AtomicDevice<'a, BUS, CS, D> {}

impl<T: Error> Error for AtomicError<T> {
    fn kind(&self) -> ErrorKind {
        match self {
            AtomicError::Other(e) => e.kind(),
            _ => ErrorKind::Other,
        }
    }
}

impl<'a, BUS, CS, D> ErrorType for AtomicDevice<'a, BUS, CS, D>
where
    BUS: ErrorType,
    CS: OutputPin,
{
    type Error = AtomicError<DeviceError<BUS::Error, CS::Error>>;
}

impl<'a, Word: Copy + 'static, BUS, CS, D> SpiDevice<Word> for AtomicDevice<'a, BUS, CS, D>
where
    BUS: SpiBus<Word>,
    CS: OutputPin,
    D: DelayNs,
{
    #[inline]
    fn transaction(&mut self, operations: &mut [Operation<'_, Word>]) -> Result<(), Self::Error> {
        self.busy
            .compare_exchange(
                false,
                true,
                core::sync::atomic::Ordering::SeqCst,
                core::sync::atomic::Ordering::SeqCst,
            )
            .map_err(|_| AtomicError::Busy)?;

        let bus = unsafe { &mut *self.bus.get() };

        let result = transaction(operations, bus, &mut self.delay, &mut self.cs);

        self.busy.store(false, core::sync::atomic::Ordering::SeqCst);

        result.map_err(AtomicError::Other)
    }
}
