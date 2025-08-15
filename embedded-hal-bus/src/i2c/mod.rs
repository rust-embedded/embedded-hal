//! `I2c` shared bus implementations.

mod refcell;
pub use refcell::*;
#[cfg(feature = "std")]
mod mutex;
#[cfg(feature = "std")]
pub use mutex::*;
mod mutex_trait;
pub use mutex_trait::*;
mod critical_section;
pub use self::critical_section::*;
#[cfg(any(feature = "portable-atomic", target_has_atomic = "8"))]
mod atomic;
#[cfg(any(feature = "portable-atomic", target_has_atomic = "8"))]
pub use atomic::*;

#[cfg(feature = "alloc")]
mod rc;
#[cfg(feature = "alloc")]
pub use rc::*;
