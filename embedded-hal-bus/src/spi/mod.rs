//! `SpiDevice` implementations.

use core::fmt::Debug;

mod exclusive;
pub use exclusive::*;
mod refcell;
pub use refcell::*;
#[cfg(feature = "std")]
mod mutex;
#[cfg(feature = "std")]
pub use mutex::*;
mod critical_section;
mod shared;

pub use self::critical_section::*;

#[cfg(feature = "defmt-03")]
use crate::defmt;

/// Dummy [`DelayNs`](embedded_hal::delay::DelayNs) implementation that panics on use.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct NoDelay;

#[cold]
fn no_delay_panic() {
    panic!("You've tried to execute a SPI transaction containing a `Operation::DelayNs` in a `SpiDevice` created with `new_no_delay()`. Create it with `new()` instead, passing a `DelayNs` implementation.");
}

impl embedded_hal::delay::DelayNs for NoDelay {
    #[inline]
    fn delay_ns(&mut self, _ns: u32) {
        no_delay_panic();
    }
}

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
impl embedded_hal_async::delay::DelayNs for NoDelay {
    #[inline]
    async fn delay_ns(&mut self, _ns: u32) {
        no_delay_panic();
    }
}
