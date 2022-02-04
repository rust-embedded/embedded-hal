//! Timer traits

/// Marker trait that indicates that a timer is periodic
pub trait Periodic {}

/// Non-blocking timer traits
pub mod nb {

    /// A count down timer
    ///
    /// # Contract
    ///
    /// - `self.start(count); block!(self.wait());` MUST block for AT LEAST the time specified by
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
    /// use hal::timer::nb::CountDown;
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
    ///     timer.start(1.s()).unwrap();
    ///     block!(timer.wait()); // blocks for 1 second
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
    /// # impl hal::timer::nb::CountDown for Timer6 {
    /// #     type Error = Infallible;
    /// #     type Time = Seconds;
    /// #     fn start(&mut self, _: Seconds) -> Result<(), Self::Error> { Ok(()) }
    /// #     fn wait(&mut self) -> ::nb::Result<(), Infallible> { Ok(()) }
    /// # }
    /// ```
    pub trait CountDown {
        /// An enumeration of `CountDown` errors.
        ///
        /// For infallible implementations, will be `Infallible`
        type Error: core::fmt::Debug;

        /// The unit of time used by this timer
        type Time;

        /// Starts a new count down
        fn start(&mut self, count: Self::Time) -> Result<(), Self::Error>;

        /// Non-blockingly "waits" until the count down finishes
        ///
        /// # Contract
        ///
        /// - If `Self: Periodic`, the timer will start a new count down right after the last one
        /// finishes.
        /// - Otherwise the behavior of calling `wait` after the last call returned `Ok` is UNSPECIFIED.
        /// Implementers are suggested to panic on this scenario to signal a programmer error.
        fn wait(&mut self) -> nb::Result<(), Self::Error>;
    }

    impl<T: CountDown> CountDown for &mut T {
        type Error = T::Error;

        type Time = T::Time;

        fn start(&mut self, count: Self::Time) -> Result<(), Self::Error> {
            T::start(self, count)
        }

        fn wait(&mut self) -> nb::Result<(), Self::Error> {
            T::wait(self)
        }
    }

    /// Trait for cancelable countdowns.
    pub trait Cancel: CountDown {
        /// Tries to cancel this countdown.
        ///
        /// # Errors
        ///
        /// An error will be returned if the countdown has already been canceled or was never started.
        /// An error is also returned if the countdown is not `Periodic` and has already expired.
        fn cancel(&mut self) -> Result<(), Self::Error>;
    }

    impl<T: Cancel> Cancel for &mut T {
        fn cancel(&mut self) -> Result<(), Self::Error> {
            T::cancel(self)
        }
    }
}
