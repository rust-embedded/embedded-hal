//! Traits for interactions with a processors watchdog timer.

/// Blocking processor watchdog traits

pub mod blocking {
    /// Feeds an existing watchdog to ensure the processor isn't reset. Sometimes
    /// the "feeding" operation is also referred to as "refreshing".
    pub trait Watchdog {
        /// An enumeration of `Watchdog` errors.
        ///
        /// For infallible implementations, will be `Infallible`
        type Error: core::fmt::Debug;

        /// Triggers the watchdog. This must be done once the watchdog is started
        /// to prevent the processor being reset.
        fn feed(&mut self) -> Result<(), Self::Error>;
    }

    impl<T: Watchdog> Watchdog for &mut T {
        type Error = T::Error;

        fn feed(&mut self) -> Result<(), Self::Error> {
            T::feed(self)
        }
    }

    /// Enables A watchdog timer to reset the processor if software is frozen or
    /// stalled.
    pub trait Enable {
        /// An enumeration of `Enable` errors.
        ///
        /// For infallible implementations, will be `Infallible`
        type Error: core::fmt::Debug;

        /// Unit of time used by the watchdog.
        type Time;

        /// The started watchdog that should be `feed()`.
        type Target: Watchdog;

        /// Starts the watchdog with a given period, typically once this is done
        /// the watchdog needs to be `feed()` periodically, or the processor would be
        /// reset.
        ///
        /// This consumes the value and returns the `Watchdog` trait that you must
        /// `feed()`.
        fn start(self, period: Self::Time) -> Result<Self::Target, Self::Error>;
    }

    /// Disables a running watchdog timer so the processor won't be reset.
    ///
    /// Not all watchdog timers support disable operation after they've been enabled.
    /// In this case, hardware support libraries would not implement this trait
    /// and hardware-agnostic libraries should consider not requiring it.
    pub trait Disable {
        /// An enumeration of `Disable` errors.
        ///
        /// For infallible implementations, will be `Infallible`
        type Error: core::fmt::Debug;

        /// Disabled watchdog instance that can be enabled.
        type Target: Enable;

        /// Disables the watchdog.
        ///
        /// This stops the watchdog and returns an instance implementing the
        /// `Enable` trait so that it can be started again.
        fn disable(self) -> Result<Self::Target, Self::Error>;
    }
}
