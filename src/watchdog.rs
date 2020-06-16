//! Traits for interactions with a processors watchdog timer.

/// Feeds an existing watchdog to ensure the processor isn't reset. Sometimes
/// commonly referred to as "kicking" or "refreshing".
pub trait Watchdog {
    /// An enumeration of `Watchdog` errors.
    ///
    /// For infallible implementations, will be `Infallible`
    type Error;

    /// Triggers the watchdog. This must be done once the watchdog is started
    /// to prevent the processor being reset.
    fn try_feed(&mut self) -> Result<(), Self::Error>;
}

/// Enables A watchdog timer to reset the processor if software is frozen or
/// stalled.
pub trait Enable {
    /// An enumeration of `Enable` errors.
    ///
    /// For infallible implementations, will be `Infallible`
    type Error;

    /// Unit of time used by the watchdog
    type Time;

    /// The started watchdog that should be `feed()`
    type Target: Watchdog;

    /// Starts the watchdog with a given period, typically once this is done 
    /// the watchdog needs to be kicked periodically or the processor is reset. 
    ///
    /// This consumes the value and returns the `Watchdog` trait that you must
    /// `feed()`.
    fn try_start<T>(&mut self, period: T) -> Result<Self::Target, Self::Error>
    where
        T: Into<Self::Time>;
}

/// Disables a running watchdog timer so the processor won't be reset.
pub trait Disable {
    /// An enumeration of `Disable` errors.
    ///
    /// For infallible implementations, will be `Infallible`
    type Error;

    /// Disabled watchdog instance that can be enabled.
    type Target: Enable;

    /// Disables the watchdog
    ///
    /// This stops the watchdog and returns the `Enable` trait so that
    /// it can be started again.
    fn try_disable(&mut self) -> Result<Self::Target, Self::Error>;
}
