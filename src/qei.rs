//! Quadrature encoder interface

/// Quadrature encoder interface
///
/// # Examples
///
/// You can use this interface to measure the speed of a motor
///
/// ```
/// extern crate embedded_hal as hal;
/// #[macro_use(block)]
/// extern crate nb;
///
/// use hal::prelude::*;
///
/// fn main() {
///     let mut qei: Qei1 = {
///         // ..
/// #       Qei1
///     };
///     let mut timer: Timer6 = {
///         // ..
/// #       Timer6
///     };
///
///
///     let before = qei.try_count().unwrap();
///     timer.try_start(1.s()).unwrap();
///     block!(timer.try_wait());
///     let after = qei.try_count().unwrap();
///
///     let speed = after.wrapping_sub(before);
///     println!("Speed: {} pulses per second", speed);
/// }
///
/// # use core::convert::Infallible;
/// # struct Seconds(u32);
/// # trait U32Ext { fn s(self) -> Seconds; }
/// # impl U32Ext for u32 { fn s(self) -> Seconds { Seconds(self) } }
/// # struct Qei1;
/// # impl hal::qei::Qei for Qei1 {
/// #     type Error = Infallible;
/// #     type Count = u16;
/// #     fn try_count(&self) -> Result<u16, Self::Error> { Ok(0) }
/// #     fn try_direction(&self) -> Result<::hal::qei::Direction, Self::Error> { unimplemented!() }
/// # }
/// # struct Timer6;
/// # impl hal::timer::CountDown for Timer6 {
/// #     type Error = Infallible;
/// #     type Time = Seconds;
/// #     fn try_start<T>(&mut self, _: T) -> Result<(), Infallible> where T: Into<Seconds> { Ok(()) }
/// #     fn try_wait(&mut self) -> ::nb::Result<(), Infallible> { Ok(()) }
/// # }
/// ```
// unproven reason: needs to be re-evaluated in the new singletons world. At the very least this needs a
// reference implementation
pub trait Qei {
    /// Enumeration of `Qei` errors
    type Error;

    /// The type of the value returned by `count`
    type Count;

    /// Returns the current pulse count of the encoder
    fn try_count(&self) -> Result<Self::Count, Self::Error>;

    /// Returns the count direction
    fn try_direction(&self) -> Result<Direction, Self::Error>;
}

/// Count direction
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    /// 3, 2, 1
    Downcounting,
    /// 1, 2, 3
    Upcounting,
}
