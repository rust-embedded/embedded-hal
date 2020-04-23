//! Timer support.
use core::fmt;
use core::pin;
use core::task;

pub mod start;
pub mod tick;
pub mod ticks;

/// A timer peripheral.
pub trait Timer: fmt::Debug {
    /// The type of error that can emerge from timer operations.
    type Error;

    /// Starts the timer, meaning that ticks will start being produced.
    fn poll_start(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), Self::Error>>;

    /// Awaits the next tick of the timer.
    fn poll_tick(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), Self::Error>>;
}

/// Utility methods for types implementing [`Timer`].
pub trait TimerExt: Timer {
    /// Starts the timer, meaning that ticks will start being produced.
    fn start(&mut self) -> start::Start<Self>
    where
        Self: Unpin,
    {
        start::start(self)
    }

    /// Awaits the next tick of the timer.
    fn tick(&mut self) -> tick::Tick<Self>
    where
        Self: Unpin,
    {
        tick::tick(self)
    }

    /// A stream of all the ticks produced by this timer, which for unbounded timers implies an
    /// infinite stream of ticks.
    fn ticks(&mut self) -> ticks::Ticks<Self>
    where
        Self: Unpin,
    {
        ticks::ticks(self)
    }
}

impl<T> TimerExt for T where T: Timer {}

/// A timer that can be ran in a periodic mode.
pub trait IntoPeriodicTimer: Timer {
    /// The version of this timer that runs in a periodic mode.
    type PeriodicTimer: Timer<Error = Self::Error> + Unpin;
    /// The type for measuring the periodic tick rate; usually this is a type measuring Hertz (Hz).
    type Rate;

    /// Re-configures this timer to be periodic.
    fn into_periodic_timer(self, period: Self::Rate) -> Result<Self::PeriodicTimer, Self::Error>;
}

/// A timer that can be ran in a one shot mode.
pub trait IntoOneshotTimer: Timer {
    /// The version of this timer that runs in a one shot mode.
    type OneshotTimer: Timer<Error = Self::Error> + Unpin;
    /// The type for measuring the one shot duration; usually this is a type measuring seconds.
    type Duration;

    /// Re-configures this timer to be a one shot timer.
    fn into_oneshot_timer(self, delay: Self::Duration) -> Result<Self::OneshotTimer, Self::Error>;
}
