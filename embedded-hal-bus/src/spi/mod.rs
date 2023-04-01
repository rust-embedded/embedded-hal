//! `SpiDevice` implementations.

use core::fmt::Debug;
use embedded_hal::spi::{Error, ErrorKind};

mod exclusive;
pub use exclusive::*;
mod refcell;
pub use refcell::*;
#[cfg(feature = "std")]
mod mutex;
#[cfg(feature = "std")]
pub use mutex::*;
mod critical_section;
pub use self::critical_section::*;

/// Error type for [`ExclusiveDevice`] operations.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DeviceError<BUS, CS> {
    /// An inner SPI bus operation failed
    Spi(BUS),
    /// Asserting or deasserting CS failed
    Cs(CS),
}

impl<BUS, CS> Error for DeviceError<BUS, CS>
where
    BUS: Error + Debug,
    CS: Debug,
{
    fn kind(&self) -> ErrorKind {
        match self {
            Self::Spi(e) => e.kind(),
            Self::Cs(_) => ErrorKind::ChipSelectFault,
        }
    }
}
