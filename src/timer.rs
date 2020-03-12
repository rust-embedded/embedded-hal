//! Timers

use nb;

/// A count down timer
///
/// # Contract
///
/// - `self.start(count); block!(self.try_wait());` MUST block for AT LEAST the time specified by
/// `count`.
///
/// *Note* that the implementer doesn't necessarily have to be a *downcounting* timer; it could also
/// be an *upcounting* timer as long as the above contract is upheld.
///
/// # Examples
///
/// You can use this timer to create delays
///
/// ```
/// extern crate embedded_hal as hal;
/// #[macro_use(block)]
/// extern crate nb;
///
/// use hal::prelude::*;
///
/// fn main() {
///     let mut led: Led = {
///         // ..
/// #       Led
///     };
///     let mut timer: Timer6 = {
///         // ..
/// #       Timer6
///     };
///
///     Led.on();
///     timer.try_start(1.s()).unwrap();
///     block!(timer.try_wait()); // blocks for 1 second
///     Led.off();
/// }
///
/// # use core::convert::Infallible;
/// # struct Seconds(u32);
/// # trait U32Ext { fn s(self) -> Seconds; }
/// # impl U32Ext for u32 { fn s(self) -> Seconds { Seconds(self) } }
/// # struct Led;
/// # impl Led {
/// #     pub fn off(&mut self) {}
/// #     pub fn on(&mut self) {}
/// # }
/// # struct Timer6;
/// # impl hal::timer::CountDown for Timer6 {
/// #     type Error = Infallible;
/// #     type Time = Seconds;
/// #     fn try_start<T>(&mut self, _: T) -> Result<(), Self::Error> where T: Into<Seconds> { Ok(()) }
/// #     fn try_wait(&mut self) -> ::nb::Result<(), Infallible> { Ok(()) }
/// # }
/// ```
pub trait CountDown {
    /// An enumeration of `CountDown` errors.
    ///
    /// For infallible implementations, will be `Infallible`
    type Error;

    /// The unit of time used by this timer
    type Time;

    /// Starts a new count down
    fn try_start<T>(&mut self, count: T) -> Result<(), Self::Error>
    where
        T: Into<Self::Time>;

    /// Non-blockingly "waits" until the count down finishes
    ///
    /// # Contract
    ///
    /// - If `Self: Periodic`, the timer will start a new count down right after the last one
    /// finishes.
    /// - Otherwise the behavior of calling `try_wait` after the last call returned `Ok` is UNSPECIFIED.
    /// Implementers are suggested to panic on this scenario to signal a programmer error.
    fn try_wait(&mut self) -> nb::Result<(), Self::Error>;
}

/// Marker trait that indicates that a timer is periodic
pub trait Periodic {}

/// Trait for cancelable countdowns.
pub trait Cancel: CountDown {
    /// Tries to cancel this countdown.
    ///
    /// # Errors
    ///
    /// An error will be returned if the countdown has already been canceled or was never started.
    /// An error is also returned if the countdown is not `Periodic` and has already expired.
    fn try_cancel(&mut self) -> Result<(), Self::Error>;
}
