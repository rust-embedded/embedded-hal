//! Asynchronous Delays
//!
//! # What's the difference this trait and the `timer::CountDown` trait?
//!
//! The `Delay` trait provides an asynchronous delay abstraction and it's meant to be used either
//! to build higher-level abstractions like I/O timeouts or by itself.

use core::{future::Future, time::Duration};

/// Asynchronously wait a duration of time.
pub trait Delay {
    /// Enumeration of `Delay` errors.
    type Error;

    /// The future returned from `delay`.
    type DelayFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Returns a future that will resolve when `duration` has passed.
    /// It is not guaranteed that _exactly_ `duration` will pass, but it will
    /// be `duration` or longer.
    fn delay<'a>(&mut self, duration: Duration) -> Self::DelayFuture<'a>;
}
