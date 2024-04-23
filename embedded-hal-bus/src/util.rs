//! Utilities shared by all bus types.

use core::cell::UnsafeCell;

/// Cell type used by [`spi::AtomicDevice`](crate::spi::AtomicDevice) and [`i2c::AtomicDevice`](crate::i2c::AtomicDevice).
///
/// To use `AtomicDevice`, you must wrap the bus with this struct, and then
/// construct multiple `AtomicDevice` instances with references to it.
pub struct AtomicCell<BUS> {
    pub(crate) bus: UnsafeCell<BUS>,
    pub(crate) busy: portable_atomic::AtomicBool,
}

unsafe impl<BUS: Send> Send for AtomicCell<BUS> {}
unsafe impl<BUS: Send> Sync for AtomicCell<BUS> {}

impl<BUS> AtomicCell<BUS> {
    /// Create a new `AtomicCell`
    pub fn new(bus: BUS) -> Self {
        Self {
            bus: UnsafeCell::new(bus),
            busy: portable_atomic::AtomicBool::from(false),
        }
    }
}
