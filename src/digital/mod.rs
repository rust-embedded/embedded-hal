//! Digital I/O
//!
//!
//!

// Deprecated / infallible traits
#[deprecated(
    since = "0.2.2",
    note = "Deprecated because the methods cannot return errors. \
                                      Users should use the traits in digital::v2."
)]
pub mod v1;

// New / fallible traits
pub mod v2;

// v2 -> v1 compatibility wrappers
// These require explicit casts from v2 -> v1
pub mod v1_compat;

// v1 -> v2 compatibility shims
// These are implicit over v1 implementations
pub mod v2_compat;

// Re-export old traits so this isn't a breaking change
#[allow(deprecated)]
pub use self::v1::*;
