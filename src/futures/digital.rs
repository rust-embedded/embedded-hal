//! Asynchronous digital I/O

use core::future::Future;

/// Asynchronously wait for a pin to become high or low.
pub trait AsyncInputPin {
    /// The future returned by the `until_high` function.
    type UntilHighFuture<'a>: Future<Output=()> + 'a;

    /// The future returned by the `until_low` function.
    type UntilLowFuture<'a>: Future<Output=()> + 'a;

    /// Returns a future that resolves when this pin becomes high.
    fn until_high<'a>(&self) -> Self::UntilHighFuture<'a>;

    /// Returns a future that resolves when this pin becomes high.
    fn until_low<'a>(&self) -> Self::UntilLowFuture<'a>;
}
