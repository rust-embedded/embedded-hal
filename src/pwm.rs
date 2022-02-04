//! Pulse Width Modulation

/// Blocking pulse width modulation traits
pub mod blocking {
    /// Pulse Width Modulation
    ///
    /// # Examples
    ///
    /// Use this interface to control the power output of some actuator
    ///
    /// ```
    /// extern crate embedded_hal as hal;
    ///
    /// use hal::pwm::blocking::Pwm;
    ///
    /// fn main() {
    ///     let mut pwm: Pwm1 = {
    ///         // ..
    /// #       Pwm1
    ///     };
    ///
    ///     pwm.set_period(1.khz()).unwrap();
    ///
    ///     let max_duty = pwm.get_max_duty().unwrap();
    ///
    ///     // brightest LED
    ///     pwm.set_duty(&Channel::_1, max_duty).unwrap();
    ///
    ///     // dimmer LED
    ///     pwm.set_duty(&Channel::_2, max_duty / 4).unwrap();
    /// }
    ///
    /// # use core::convert::Infallible;
    /// # struct KiloHertz(u32);
    /// # trait U32Ext { fn khz(self) -> KiloHertz; }
    /// # impl U32Ext for u32 { fn khz(self) -> KiloHertz { KiloHertz(self) } }
    /// # enum Channel { _1, _2 }
    /// # struct Pwm1;
    /// # impl hal::pwm::blocking::Pwm for Pwm1 {
    /// #     type Error = Infallible;
    /// #     type Channel = Channel;
    /// #     type Time = KiloHertz;
    /// #     type Duty = u16;
    /// #     fn disable(&mut self, _: &Channel) -> Result<(), Self::Error> { unimplemented!() }
    /// #     fn enable(&mut self, _: &Channel) -> Result<(), Self::Error> { unimplemented!() }
    /// #     fn get_duty(&self, _: &Channel) -> Result<u16, Self::Error> { unimplemented!() }
    /// #     fn get_max_duty(&self) -> Result<u16, Self::Error> { Ok(0) }
    /// #     fn set_duty(&mut self, _: &Channel, _: u16) -> Result<(), Self::Error> { Ok(()) }
    /// #     fn get_period(&self) -> Result<KiloHertz, Self::Error> { unimplemented!() }
    /// #     fn set_period(&mut self, _: KiloHertz) -> Result<(), Self::Error> { Ok(()) }
    /// # }
    /// ```
    // unproven reason: pre-singletons API. The `PwmPin` trait seems more useful because it models independent
    // PWM channels. Here a certain number of channels are multiplexed in a single implementer.
    pub trait Pwm {
        /// Enumeration of `Pwm` errors
        type Error: core::fmt::Debug;

        /// Enumeration of channels that can be used with this `Pwm` interface
        ///
        /// If your `Pwm` interface has no channels you can use the type `()`
        /// here
        type Channel;

        /// A time unit that can be converted into a human time unit (e.g. seconds)
        type Time;

        /// Type for the `duty` methods
        ///
        /// The implementer is free to choose a float / percentage representation
        /// (e.g. `0.0 .. 1.0`) or an integer representation (e.g. `0 .. 65535`)
        type Duty;

        /// Disables a PWM `channel`
        fn disable(&mut self, channel: &Self::Channel) -> Result<(), Self::Error>;

        /// Enables a PWM `channel`
        fn enable(&mut self, channel: &Self::Channel) -> Result<(), Self::Error>;

        /// Returns the current PWM period
        fn get_period(&self) -> Result<Self::Time, Self::Error>;

        /// Returns the current duty cycle
        ///
        /// While the pin is transitioning to the new duty cycle after a `set_duty` call, this may
        /// return the old or the new duty cycle depending on the implementation.
        fn get_duty(&self, channel: &Self::Channel) -> Result<Self::Duty, Self::Error>;

        /// Returns the maximum duty cycle value
        fn get_max_duty(&self) -> Result<Self::Duty, Self::Error>;

        /// Sets a new duty cycle
        fn set_duty(
            &mut self,
            channel: &Self::Channel,
            duty: Self::Duty,
        ) -> Result<(), Self::Error>;

        /// Sets a new PWM period
        fn set_period(&mut self, period: Self::Time) -> Result<(), Self::Error>;
    }

    impl<T: Pwm> Pwm for &mut T {
        type Error = T::Error;

        type Channel = T::Channel;

        type Time = T::Time;

        type Duty = T::Duty;

        fn disable(&mut self, channel: &Self::Channel) -> Result<(), Self::Error> {
            T::disable(self, channel)
        }

        fn enable(&mut self, channel: &Self::Channel) -> Result<(), Self::Error> {
            T::enable(self, channel)
        }

        fn get_period(&self) -> Result<Self::Time, Self::Error> {
            T::get_period(self)
        }

        fn get_duty(&self, channel: &Self::Channel) -> Result<Self::Duty, Self::Error> {
            T::get_duty(self, channel)
        }

        fn get_max_duty(&self) -> Result<Self::Duty, Self::Error> {
            T::get_max_duty(self)
        }

        fn set_duty(
            &mut self,
            channel: &Self::Channel,
            duty: Self::Duty,
        ) -> Result<(), Self::Error> {
            T::set_duty(self, channel, duty)
        }

        fn set_period(&mut self, period: Self::Time) -> Result<(), Self::Error> {
            T::set_period(self, period)
        }
    }

    /// A single PWM channel / pin
    ///
    /// See `Pwm` for details
    pub trait PwmPin {
        /// Enumeration of `PwmPin` errors
        type Error: core::fmt::Debug;

        /// Type for the `duty` methods
        ///
        /// The implementer is free to choose a float / percentage representation
        /// (e.g. `0.0 .. 1.0`) or an integer representation (e.g. `0 .. 65535`)
        type Duty;

        /// Disables a PWM `channel`
        fn disable(&mut self) -> Result<(), Self::Error>;

        /// Enables a PWM `channel`
        fn enable(&mut self) -> Result<(), Self::Error>;

        /// Returns the current duty cycle
        ///
        /// While the pin is transitioning to the new duty cycle after a `set_duty` call, this may
        /// return the old or the new duty cycle depending on the implementation.
        fn get_duty(&self) -> Result<Self::Duty, Self::Error>;

        /// Returns the maximum duty cycle value
        fn get_max_duty(&self) -> Result<Self::Duty, Self::Error>;

        /// Sets a new duty cycle
        fn set_duty(&mut self, duty: Self::Duty) -> Result<(), Self::Error>;
    }

    impl<T: PwmPin> PwmPin for &mut T {
        type Error = T::Error;

        type Duty = T::Duty;

        fn disable(&mut self) -> Result<(), Self::Error> {
            T::disable(self)
        }

        fn enable(&mut self) -> Result<(), Self::Error> {
            T::enable(self)
        }

        fn get_duty(&self) -> Result<Self::Duty, Self::Error> {
            T::get_duty(self)
        }

        fn get_max_duty(&self) -> Result<Self::Duty, Self::Error> {
            T::get_max_duty(self)
        }

        fn set_duty(&mut self, duty: Self::Duty) -> Result<(), Self::Error> {
            T::set_duty(self, duty)
        }
    }
}
