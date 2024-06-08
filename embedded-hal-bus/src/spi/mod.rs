//! `SpiDevice` implementations.

use core::fmt::{self, Debug, Display, Formatter};
use embedded_hal::spi::{Error, ErrorKind};

mod exclusive;
pub use exclusive::*;
mod refcell;
pub use refcell::*;
#[cfg(feature = "std")]
mod mutex;
#[cfg(feature = "std")]
pub use mutex::*;
#[cfg(any(feature = "atomic-device", target_has_atomic = "8"))]
mod atomic;
mod critical_section;
mod shared;
#[cfg(any(feature = "atomic-device", target_has_atomic = "8"))]
pub use atomic::*;

pub use self::critical_section::*;

#[cfg(feature = "defmt-03")]
use crate::defmt;

/// Error type for [`ExclusiveDevice`] operations.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum DeviceError<BUS, CS> {
    /// An inner SPI bus operation failed.
    Spi(BUS),
    /// Asserting or deasserting CS failed.
    Cs(CS),
}

impl<BUS: Display, CS: Display> Display for DeviceError<BUS, CS> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Spi(bus) => write!(f, "SPI bus error: {}", bus),
            Self::Cs(cs) => write!(f, "SPI CS error: {}", cs),
        }
    }
}

#[cfg(feature = "std")]
impl<BUS: Debug + Display, CS: Debug + Display> std::error::Error for DeviceError<BUS, CS> {}

impl<BUS, CS> Error for DeviceError<BUS, CS>
where
    BUS: Error + Debug,
    CS: Debug,
{
    #[inline]
    fn kind(&self) -> ErrorKind {
        match self {
            Self::Spi(e) => e.kind(),
            Self::Cs(_) => ErrorKind::ChipSelectFault,
        }
    }
}

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
