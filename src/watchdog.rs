//! Traits for interactions with a processors watchdog timer.

/// Feeds an existing watchdog to ensure the processor isn't reset. Sometimes
/// commonly referred to as "kicking" or "refreshing".
#[cfg(feature = "unproven")]
pub trait Watchdog {
    /// Triggers the watchdog. This must be done once the watchdog is started
    /// to prevent the processor being reset.
    fn feed(&mut self);
}

/// Enables A watchdog timer to reset the processor if software is frozen or
/// stalled.
#[cfg(feature = "unproven")]
pub trait WatchdogEnable {
    /// Unit of time used by the watchdog
    type Time;
    /// Starts the watchdog with a given period, typically once this is done
    /// the watchdog needs to be kicked periodically or the processor is reset.
    fn start<T>(&mut self, period: T)
    where
        T: Into<Self::Time>;
}

/// Disables a running watchdog timer so the processor won't be reset.
#[cfg(feature = "unproven")]
pub trait WatchdogDisable {
    /// Disables the watchdog
    fn disable(&mut self);
}
