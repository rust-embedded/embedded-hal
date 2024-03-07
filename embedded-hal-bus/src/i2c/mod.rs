//! `I2c` shared bus implementations.

mod refcell;
pub use refcell::*;
#[cfg(feature = "std")]
mod mutex;
#[cfg(feature = "std")]
pub use mutex::*;
mod critical_section;
pub use self::critical_section::*;
mod atomic;
pub use atomic::*;
