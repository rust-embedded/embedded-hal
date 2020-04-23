//! Serial data transfer support.
use crate::asynchronous::io;

/// A peripheral that can perform serial read operations.
pub trait SerialRead: io::Read {}

/// A peripheral that can perform serial write operations.
pub trait SerialWrite: io::Write {}
