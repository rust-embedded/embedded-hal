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
pub trait WatchdogEnable {
    /// An enumeration of `WatchdogEnable` errors.
    ///
    /// For infallible implementations, will be `Infallible`
    type Error;

    /// Unit of time used by the watchdog
    type Time;

    /// Starts the watchdog with a given period, typically once this is done
    /// the watchdog needs to be kicked periodically or the processor is reset.
    fn try_start<T>(&mut self, period: T) -> Result<(), Self::Error>
    where
        T: Into<Self::Time>;
}

/// Disables a running watchdog timer so the processor won't be reset.
pub trait WatchdogDisable {
    /// An enumeration of `WatchdogDisable` errors.
    ///
    /// For infallible implementations, will be `Infallible`
    type Error;

    /// Disables the watchdog
    fn try_disable(&mut self) -> Result<(), Self::Error>;
}
