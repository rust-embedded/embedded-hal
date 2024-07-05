extern crate alloc;
use alloc::rc::Rc;

use core::cell::RefCell;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::{ErrorType, Operation, SpiBus, SpiDevice};

use super::DeviceError;
use crate::spi::shared::transaction;

/// Implementation of [`SpiDevice`] around a bus shared with `Rc<RefCell<T>>`.
/// This is the reference-counting equivalent of [`RefCellDevice`](super::RefCellDevice), requiring allocation.
///
/// A single [`SpiBus`] is shared via [`RefCell`], and its ownership is handled by [`Rc`].
/// Both of these mechanisms only allow sharing within a single thread (or interrupt priority level).
/// For this reason, this does not implement [`Send`].
///
/// When this structure is dropped, the reference count of the `Bus` instance will be decremented,
/// and it will be cleaned up once the reference count reaches zero.
#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
pub struct RcDevice<Bus, Cs, Delay> {
    bus: Rc<RefCell<Bus>>,
    cs: Cs,
    delay: Delay,
}

impl<Bus, Cs, Delay> RcDevice<Bus, Cs, Delay> {
    /// Creates a new [`RcDevice`].
    ///
    /// This sets the `cs` pin high, and returns an error if that fails.
    /// It is recommended to have already set that pin high the moment it has been configured as an output, to avoid glitches.
    ///
    /// This function does not increment the reference count:
    /// you will need to call `Rc::clone(&bus)` if you only have a `&Rc<RefCell<Bus>>`.
    #[inline]
    pub fn new(bus: Rc<RefCell<Bus>>, mut cs: Cs, delay: Delay) -> Result<Self, Cs::Error>
    where
        Cs: OutputPin,
    {
        cs.set_high()?;

        Ok(Self { bus, cs, delay })
    }
}

impl<Bus, Cs> RcDevice<Bus, Cs, super::NoDelay> {
    /// Creates a new [`RcDevice`] without support for in-transaction delays.
    ///
    /// **Warning**: It's advised to prefer [`RcDevice::new`],
    /// as the contract of [`SpiDevice`] requests support for in-transaction delays.
    ///
    /// Refer to [`RefCellDevice::new_no_delay`](super::RefCellDevice::new_no_delay) for more information.
    #[inline]
    pub fn new_no_delay(bus: Rc<RefCell<Bus>>, mut cs: Cs) -> Result<Self, Cs::Error>
    where
        Cs: OutputPin,
    {
        cs.set_high()?;

        Ok(Self {
            bus,
            cs,
            delay: super::NoDelay,
        })
    }
}

impl<Bus, Cs, Delay> ErrorType for RcDevice<Bus, Cs, Delay>
where
    Bus: ErrorType,
    Cs: OutputPin,
{
    type Error = DeviceError<Bus::Error, Cs::Error>;
}

impl<Word, Bus, Cs, Delay> SpiDevice<Word> for RcDevice<Bus, Cs, Delay>
where
    Word: Copy + 'static,
    Bus: SpiBus<Word>,
    Cs: OutputPin,
    Delay: DelayNs,
{
    #[inline]
    fn transaction(&mut self, operations: &mut [Operation<'_, Word>]) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.borrow_mut();

        transaction(operations, bus, &mut self.delay, &mut self.cs)
    }
}
