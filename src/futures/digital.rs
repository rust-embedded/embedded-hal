//! Asynchronous digital I/O

use core::future::Future;

use crate::blocking::digital::InputPin;

/// Asynchronously wait for a pin to become high or low.
///
/// # Examples
/// ```rust
/// # use embedded_hal::futures::digital::AsyncInputPin;
/// /// Asynchronously wait until the `ready_pin` becomes high.
/// async fn wait_until_ready<P>(ready_pin: &P)
/// where
///     P: AsyncInputPin,
/// {
///     ready_pin
///         .until_high()
///         .await
///         .expect("failed to await input pin")
/// }
/// ```
///
/// ```rust,ignore
/// # use embedded_hal::futures::digital::AsyncInputPin;
/// # use embedded_hal::futures::delay::Delay;
/// use core::time::Duration;
///
/// /// Wait until the `ready_pin` is high or timeout after 1 millisecond.
/// /// Returns true is the pin became high or false if it timed-out.
/// async fn wait_until_ready_or_timeout<P, D>(ready_pin: &P, delay: &mut D) -> bool
/// where
///     P: AsyncInputPin,
///     D: Delay,
/// {
///     futures::select_biased! {
///         x => ready_pin.until_high() => {
///             x.expect("failed to await input pin");
///             true
///         },
///         _ => delay.delay(Duration::from_millis(1)) => false, // ignore the error
///     }
/// }
/// ```
pub trait AsyncInputPin: InputPin {
    /// The future returned by the `until_high` function.
    type UntilHighFuture<'a>: Future<Output=Result<(), Self::Error>> + 'a;

    /// The future returned by the `until_low` function.
    type UntilLowFuture<'a>: Future<Output=Result<(), Self::Error>> + 'a;

    /// Returns a future that resolves when this pin becomes high.
    fn until_high<'a>(&self) -> Self::UntilHighFuture<'a>;

    /// Returns a future that resolves when this pin becomes high.
    fn until_low<'a>(&self) -> Self::UntilLowFuture<'a>;
}
