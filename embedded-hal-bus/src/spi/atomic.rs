use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::{Error, ErrorKind, ErrorType, Operation, SpiBus, SpiDevice};

use super::DeviceError;
use crate::spi::shared::transaction;
use crate::util::AtomicCell;

/// Atomics-based shared bus [`SpiDevice`] implementation.
///
/// This allows for sharing an [`SpiBus`], obtaining multiple [`SpiDevice`] instances,
/// each with its own `CS` pin.
///
/// Sharing is implemented with a [`AtomicDevice`], which consists of an `UnsafeCell` and an `AtomicBool` "locked" flag.
/// This means it has low overhead, like [`RefCellDevice`](crate::spi::RefCellDevice). Additionally, it is `Send`,
/// which allows sharing a single bus across multiple threads (interrupt priority level), like [`CriticalSectionDevice`](crate::spi::CriticalSectionDevice),
/// while not using critical sections and therefore impacting real-time performance less.
///
/// The downside is using a simple `AtomicBool` for locking doesn't prevent two threads from trying to lock it at once.
/// For example, the main thread can be doing a SPI transaction, and an interrupt fires and tries to do another. In this
/// case, a `Busy` error is returned that must be handled somehow, usually dropping the data or trying again later.
///
/// Note that retrying in a loop on `Busy` errors usually leads to deadlocks. In the above example, it'll prevent the
/// interrupt handler from returning, which will starve the main thread and prevent it from releasing the lock. If
/// this is an issue for your use case, you most likely should use [`CriticalSectionDevice`](crate::spi::CriticalSectionDevice) instead.
///
/// This primitive is particularly well-suited for applications that have external arbitration
/// rules that prevent `Busy` errors in the first place, such as the RTIC framework.
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "portable-atomic", target_has_atomic = "8")))
)]
pub struct AtomicDevice<'a, BUS, CS, D> {
    bus: &'a AtomicCell<BUS>,
    cs: CS,
    delay: D,
}

#[derive(Debug, Copy, Clone)]
/// Wrapper type for errors returned by [`AtomicDevice`].
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "portable-atomic", target_has_atomic = "8")))
)]
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
    ///
    /// This sets the `cs` pin high, and returns an error if that fails. It is recommended
    /// to set the pin high the moment it's configured as an output, to avoid glitches.
    #[inline]
    pub fn new(bus: &'a AtomicCell<BUS>, mut cs: CS, delay: D) -> Result<Self, CS::Error>
    where
        CS: OutputPin,
    {
        cs.set_high()?;
        Ok(Self { bus, cs, delay })
    }
}

impl<'a, BUS, CS> AtomicDevice<'a, BUS, CS, super::NoDelay>
where
    BUS: ErrorType,
    CS: OutputPin,
{
    /// Create a new [`AtomicDevice`] without support for in-transaction delays.
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
    pub fn new_no_delay(bus: &'a AtomicCell<BUS>, mut cs: CS) -> Result<Self, CS::Error>
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

impl<T: Error> Error for AtomicError<T> {
    fn kind(&self) -> ErrorKind {
        match self {
            AtomicError::Other(e) => e.kind(),
            _ => ErrorKind::Other,
        }
    }
}

impl<BUS, CS, D> ErrorType for AtomicDevice<'_, BUS, CS, D>
where
    BUS: ErrorType,
    CS: OutputPin,
{
    type Error = AtomicError<DeviceError<BUS::Error, CS::Error>>;
}

impl<Word: Copy + 'static, BUS, CS, D> SpiDevice<Word> for AtomicDevice<'_, BUS, CS, D>
where
    BUS: SpiBus<Word>,
    CS: OutputPin,
    D: DelayNs,
{
    #[inline]
    fn transaction(&mut self, operations: &mut [Operation<'_, Word>]) -> Result<(), Self::Error> {
        self.bus
            .busy
            .compare_exchange(
                false,
                true,
                core::sync::atomic::Ordering::SeqCst,
                core::sync::atomic::Ordering::SeqCst,
            )
            .map_err(|_| AtomicError::Busy)?;

        let bus = unsafe { &mut *self.bus.bus.get() };

        let result = transaction(operations, bus, &mut self.delay, &mut self.cs);

        self.bus
            .busy
            .store(false, core::sync::atomic::Ordering::SeqCst);

        result.map_err(AtomicError::Other)
    }
}
