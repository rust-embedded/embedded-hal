//! Utilities shared by all bus types.

#[allow(unused_imports)]
use core::cell::UnsafeCell;

#[cfg(not(feature = "portable-atomic"))]
use core::sync::atomic::AtomicBool;
#[cfg(feature = "portable-atomic")]
use portable_atomic::AtomicBool;

#[cfg(any(feature = "portable-atomic", target_has_atomic = "8"))]
/// Cell type used by [`spi::AtomicDevice`](crate::spi::AtomicDevice) and [`i2c::AtomicDevice`](crate::i2c::AtomicDevice).
///
/// To use `AtomicDevice`, you must wrap the bus with this struct, and then
/// construct multiple `AtomicDevice` instances with references to it.
pub struct AtomicCell<BUS> {
    pub(crate) bus: UnsafeCell<BUS>,
    pub(crate) busy: AtomicBool,
}
#[cfg(any(feature = "portable-atomic", target_has_atomic = "8"))]
unsafe impl<BUS: Send> Send for AtomicCell<BUS> {}
#[cfg(any(feature = "portable-atomic", target_has_atomic = "8"))]
unsafe impl<BUS: Send> Sync for AtomicCell<BUS> {}

#[cfg(any(feature = "portable-atomic", target_has_atomic = "8"))]
impl<BUS> AtomicCell<BUS> {
    /// Create a new `AtomicCell`
    pub fn new(bus: BUS) -> Self {
        Self {
            bus: UnsafeCell::new(bus),
            busy: AtomicBool::from(false),
        }
    }
}
