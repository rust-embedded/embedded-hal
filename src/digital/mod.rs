//! Digital I/O
//!
//!
//!

#[allow(deprecated)]
pub use embedded_hal_v3::digital::{v1, v2, v1_compat, v2_compat};

// Re-export old traits so this isn't a breaking change
#[allow(deprecated)]
pub use self::v1::*;
