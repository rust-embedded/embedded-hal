//! Quadrature encoder interface traits

/// Count direction
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    /// 3, 2, 1
    Downcounting,
    /// 1, 2, 3
    Upcounting,
}

/// Blocking quadrature encoder interface traits
pub mod blocking {
    use super::Direction;

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
    /// use hal::qei::blocking::Qei;
    ///
    /// fn main() {
    ///     let mut qei: Qei1 = {
    ///         // ..
    /// #       Qei1
    ///     };
    ///
    ///
    ///     let before = qei.count().unwrap();
    ///     // wait some time
    ///     let after = qei.count().unwrap();
    ///
    ///     let speed = after.wrapping_sub(before);
    ///     println!("Speed: {} pulses per second", speed);
    /// }
    ///
    /// # use core::convert::Infallible;
    /// # struct Qei1;
    /// # impl hal::qei::blocking::Qei for Qei1 {
    /// #     type Error = Infallible;
    /// #     type Count = u16;
    /// #     fn count(&self) -> Result<u16, Self::Error> { Ok(0) }
    /// #     fn direction(&self) -> Result<::hal::qei::Direction, Self::Error> { unimplemented!() }
    /// # }
    /// ```
    // unproven reason: needs to be re-evaluated in the new singletons world. At the very least this needs a
    // reference implementation
    pub trait Qei {
        /// Enumeration of `Qei` errors
        type Error: core::fmt::Debug;

        /// The type of the value returned by `count`
        type Count;

        /// Returns the current pulse count of the encoder
        fn count(&self) -> Result<Self::Count, Self::Error>;

        /// Returns the count direction
        fn direction(&self) -> Result<Direction, Self::Error>;
    }

    impl<T: Qei> Qei for &T {
        type Error = T::Error;

        type Count = T::Count;

        fn count(&self) -> Result<Self::Count, Self::Error> {
            T::count(self)
        }

        fn direction(&self) -> Result<Direction, Self::Error> {
            T::direction(self)
        }
    }
}
