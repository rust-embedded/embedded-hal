//! Input capture

use nb;

/// Input capture
///
/// # Examples
///
/// You can use this interface to measure the period of (quasi) periodic signals
/// / events
///
/// ```
/// extern crate embedded_hal as hal;
/// #[macro_use(block)]
/// extern crate nb;
///
/// use hal::prelude::*;
///
/// fn main() {
///     let mut capture: Capture1 = {
///         // ..
/// #       Capture1
///     };
///
///     capture.try_set_resolution(1.ms()).unwrap();
///
///     let before = block!(capture.try_capture(Channel::_1)).unwrap();
///     let after = block!(capture.try_capture(Channel::_1)).unwrap();
///
///     let period = after.wrapping_sub(before);
///
///     println!("Period: {} ms", period);
/// }
///
/// # use core::convert::Infallible;
/// # struct MilliSeconds(u32);
/// # trait U32Ext { fn ms(self) -> MilliSeconds; }
/// # impl U32Ext for u32 { fn ms(self) -> MilliSeconds { MilliSeconds(self) } }
/// # struct Capture1;
/// # enum Channel { _1 }
/// # impl hal::capture::Capture for Capture1 {
/// #     type Error = Infallible;
/// #     type Capture = u16;
/// #     type Channel = Channel;
/// #     type Time = MilliSeconds;
/// #     fn try_capture(&mut self, _: Channel) -> ::nb::Result<u16, Self::Error> { Ok(0) }
/// #     fn try_disable(&mut self, _: Channel) -> Result<(), Self::Error> { unimplemented!() }
/// #     fn try_enable(&mut self, _: Channel) -> Result<(), Self::Error> { unimplemented!() }
/// #     fn try_get_resolution(&self) -> Result<MilliSeconds, Self::Error> { unimplemented!() }
/// #     fn try_set_resolution<T>(&mut self, _: T) -> Result<(), Self::Error> where T: Into<MilliSeconds> { Ok(()) }
/// # }
/// ```
// unproven reason: pre-singletons API. With singletons a `CapturePin` (cf. `PwmPin`) trait seems more
// appropriate
pub trait Capture {
    /// Enumeration of `Capture` errors
    ///
    /// Possible errors:
    ///
    /// - *overcapture*, the previous capture value was overwritten because it
    ///   was not read in a timely manner
    type Error;

    /// Enumeration of channels that can be used with this `Capture` interface
    ///
    /// If your `Capture` interface has no channels you can use the type `()`
    /// here
    type Channel;

    /// A time unit that can be converted into a human time unit (e.g. seconds)
    type Time;

    /// The type of the value returned by `capture`
    type Capture;

    /// "Waits" for a transition in the capture `channel` and returns the value
    /// of counter at that instant
    ///
    /// NOTE that you must multiply the returned value by the *resolution* of
    /// this `Capture` interface to get a human time unit (e.g. seconds)
    fn try_capture(&mut self, channel: Self::Channel) -> nb::Result<Self::Capture, Self::Error>;

    /// Disables a capture `channel`
    fn try_disable(&mut self, channel: Self::Channel) -> Result<(), Self::Error>;

    /// Enables a capture `channel`
    fn try_enable(&mut self, channel: Self::Channel) -> Result<(), Self::Error>;

    /// Returns the current resolution
    fn try_get_resolution(&self) -> Result<Self::Time, Self::Error>;

    /// Sets the resolution of the capture timer
    fn try_set_resolution<R>(&mut self, resolution: R) -> Result<(), Self::Error>
    where
        R: Into<Self::Time>;
}
