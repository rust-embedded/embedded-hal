//! Pulse Width Modulation

use core::time::Duration;

/// Pulse Width Modulation
///
/// # Examples
///
/// Use this interface to control the power output of some actuator
///
/// ```
/// extern crate embedded_hal as hal;
///
/// use core::time::Duration;
/// # use core::marker::PhantomData;
/// use hal::pwm::*;
///
/// fn main() {
///     let mut pwm = {
///         // ..
/// #       Pwm1(HasPins)
///     };
///
///     pwm.try_set_period(1.khz()).unwrap();
///
///     let max_duty = Pwm1::<HasPins>::MAX_DUTY;
///
///     // brightest LED
///     pwm.try_set_duty(0, max_duty).unwrap();
///
///     // dimmer LED
///     pwm.try_set_duty(1, max_duty / 4).unwrap();
///
///     let (pwm, [mut ch1, mut ch2]) = pwm.split(); 
/// 
///     ch1.try_set_duty(max_duty / 5).unwrap();
///     ch2.try_set_duty(max_duty / 5).unwrap();
///
/// }
///
/// # use core::convert::Infallible;
/// # trait U32Ext { fn khz(self) -> Duration; }
/// # impl U32Ext for u32 { fn khz(self) -> Duration { Duration::from_nanos( 1_000_000u64 / self as u64 ) } }
/// // Note: possibly you could genericise this over [PwmPinImpl; CHANNELS], which would be great for accessing channels internally
/// // except, these are probably not all going to be the same type, so instead we're using marker traits to specify whether it has been split or not
/// # struct Pwm1<S>(S);
/// # struct Pwm1Pin;
/// # struct HasPins;
/// # impl Pwm1<HasPins> {
/// #     pub fn split(self) -> (Pwm1<()>, [Pwm1Pin; 2]) {
/// #       (Pwm1(()), [Pwm1Pin, Pwm1Pin])
/// #     }
/// # }
/// # impl Pwm1<()> {
/// #     pub fn join(self, pins: [Pwm1Pin; 2]) -> Pwm1<HasPins> {
/// #       Pwm1(HasPins)
/// #     }
/// # }
/// #
/// # impl <S> hal::pwm::Pwm<u16> for Pwm1<S> {
/// #     const CHANNELS: usize = 4;
/// #     const MAX_DUTY: u16 = 1024;
/// #     type Error = Infallible;
/// #
/// #     fn try_disable(&mut self, _: usize) -> Result<(), Self::Error> { unimplemented!() }
/// #     fn try_enable(&mut self, _: usize) -> Result<(), Self::Error> { unimplemented!() }
/// #     fn try_get_period(&self) -> Result<Duration, Self::Error> { unimplemented!() }
/// #     fn try_set_period<T>(&mut self, _: T) -> Result<(), Self::Error> where T: Into<Duration> { Ok(()) }
/// # }
/// #
/// # impl hal::pwm::PwmDuty<u16> for Pwm1<HasPins> {
/// #     fn try_get_duty(&self, _: usize) -> Result<u16, Self::Error> { unimplemented!() }
/// #     fn try_set_duty<D>(&mut self, _: usize, _: D) -> Result<(), Self::Error> where D: Into<u16> { Ok(()) }
/// # }
/// # impl hal::pwm::PwmPin<u16> for Pwm1Pin {
/// #     const MAX_DUTY: u16 = 1024;
/// #     type Error = Infallible;
/// #     fn try_disable(&mut self) -> Result<(), Self::Error> { unimplemented!() }
/// #     fn try_enable(&mut self) -> Result<(), Self::Error> { unimplemented!() }
/// #     fn try_get_duty(&self) -> Result<u16, Self::Error> { unimplemented!() }
/// #     fn try_set_duty<D>(&mut self, _: D) -> Result<(), Self::Error> where D: Into<u16> { Ok(()) }
/// # }
/// ```
// unproven reason: pre-singletons API. The `PwmPin` trait seems more useful because it models independent
// PWM channels. Here a certain number of channels are multiplexed in a single implementer.
pub trait Pwm<Duty> {
    /// Number of available channels
    const CHANNELS: usize;

    /// Maximum duty cycle value
    const MAX_DUTY: Duty;

    /// Enumeration of `Pwm` errors
    type Error;

    /// Disables a PWM `channel`
    fn try_disable(&mut self, channel: usize) -> Result<(), Self::Error>;

    /// Enables a PWM `channel`
    fn try_enable(&mut self, channel: usize) -> Result<(), Self::Error>;

    // Note: should you be able to _set_ the period once the channels have been split?
    // my feeling is, probably not

    /// Returns the current PWM period
    fn try_get_period(&self) -> Result<Duration, Self::Error>;

    /// Sets a new PWM period
    fn try_set_period<P>(&mut self, period: P) -> Result<(), Self::Error>
    where
        P: Into<Duration>;

    // Note: technically there could be a `channel` or `split` method here but this is
    // rather extremely difficult prior to const-generics landing (and CHANNELS would need to be a const generic)
}

/// PwmDuty trait allows PWM pins duty-cycles to be set per-channel
///
/// This should be implemented for a `Pwm` type that currently holds channel references
pub trait PwmDuty<Duty>: Pwm<Duty> {

    /// Returns the current duty cycle
    ///
    /// While the pin is transitioning to the new duty cycle after a `try_set_duty` call, this may
    /// return the old or the new duty cycle depending on the implementation.
    fn try_get_duty(&self, channel: usize) -> Result<Duty, Self::Error>;

    /// Sets a new duty cycle
    fn try_set_duty<D>(&mut self, channel: usize, duty: D) -> Result<(), Self::Error>
    where
        D: Into<Duty>;
}

/// A single PWM channel / pin
///
/// This trait should be implemented over split PWM channels, see `Pwm` for details
pub trait PwmPin<Duty> {

    /// Maximum duty cycle value
    const MAX_DUTY: Duty;

    /// Enumeration of `PwmPin` errors
    type Error;

    /// Disables a PWM `channel`
    fn try_disable(&mut self) -> Result<(), Self::Error>;

    /// Enables a PWM `channel`
    fn try_enable(&mut self) -> Result<(), Self::Error>;

    /// Returns the current duty cycle
    ///
    /// While the pin is transitioning to the new duty cycle after a `try_set_duty` call, this may
    /// return the old or the new duty cycle depending on the implementation.
    fn try_get_duty(&self) -> Result<Duty, Self::Error>;

    /// Sets a new duty cycle
    fn try_set_duty<D>(&mut self, duty: D) -> Result<(), Self::Error>
    where
        D: Into<Duty>;
}

