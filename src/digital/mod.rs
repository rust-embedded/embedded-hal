//! Digital I/O
//!
//!
//!

// Fallible digital traits
// This has been left as a submodule to smooth transitions from v0.2.x and
// may be removed in future
#[deprecated(
    since = "0.3.0",
    note = "Please use traits directly from `digital::` instead"
)]
pub mod v2;

// Re-export default traits
pub use self::v2::*;
