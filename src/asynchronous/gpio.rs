//! General input/output pins.
//!
//! The [`InputPin`] and [`OutputPin`] traits define pins that can be read and written digitally
//! (i.e. either in a low or high state).
//!
//! There are additionally various `Into*` traits that allow users to re-configure pins to switch
//! between different modes of operation, e.g. [`IntoFloatingInputPin`] turns a pin into an
//! [`InputPin`] that does not employ any pull-up or pull-down resistors.
use core::pin;
use core::task;

pub mod get;
pub mod set;

/// A generic pin that can't be interacted with.
pub trait Pin {
    /// The common error type for all pin operations.
    ///
    /// A single error type for all operations is enforced for simplicity.
    type Error;
}

/// A pin that can be read from.
pub trait InputPin: Pin {
    /// Polls a read operation of this pin to completion.
    fn poll_get(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<bool, Self::Error>>;
}

/// Extension functions for instances of [`InputPin`].
pub trait InputPinExt: InputPin {
    /// Gets the current high or low state of this pin.
    fn get(&mut self) -> get::Get<Self>
    where
        Self: Unpin,
    {
        get::get(self)
    }
}

impl<A> InputPinExt for A where A: InputPin {}

/// A pin that can be written to.
pub trait OutputPin: Pin {
    /// Polls a write operation of this pin to completion.
    fn poll_set(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        high: bool,
    ) -> task::Poll<Result<(), Self::Error>>;
}

/// Extension functions for instances of [`OutputPin`].
pub trait OutputPinExt: OutputPin {
    /// Sets the current high or low state of this pin.
    fn set(&mut self, high: bool) -> set::Set<Self>
    where
        Self: Unpin,
    {
        set::set(self, high)
    }
}

impl<A> OutputPinExt for A where A: OutputPin {}

/// A pin that can be turned into an [`InputPin`] that does not employ any pull-up or pull-down
/// resistors.
pub trait IntoFloatingInputPin: Pin {
    /// The type of an [`InputPin`] that does not employ any pull-up or pull-down resistors.
    type FloatingInputPin: InputPin<Error = Self::Error> + Unpin;

    /// Attempts to re-configure this pin into the new mode.
    fn into_floating_input_pin(self) -> Result<Self::FloatingInputPin, Self::Error>;
}

/// A pin that can be turned into an [`InputPin`] that has a pull-up resistor attached.
pub trait IntoPullUpInputPin: Pin {
    /// The type of an [`InputPin`] that has a pull-up resistor attached.
    type PullUpInputPin: InputPin<Error = Self::Error> + Unpin;

    /// Attempts to re-configure this pin into the new mode.
    fn into_pull_up_input_pin(self) -> Result<Self::PullUpInputPin, Self::Error>;
}

/// A pin that can be turned into an [`InputPin`] that has a pull-down resistor attached.
pub trait IntoPullDownInputPin: Pin {
    /// The type of an [`InputPin`] that has a pull-down resistor attached.
    type PullDownInputPin: InputPin<Error = Self::Error> + Unpin;

    /// Attempts to re-configure this pin into the new mode.
    fn into_pull_down_input_pin(self) -> Result<Self::PullDownInputPin, Self::Error>;
}

/// A pin that can be turned into an [`OutputPin`] that is in open drain mode.
pub trait IntoOpenDrainOutputPin: Pin {
    /// The type of an [`OutputPin`] that is in open drain mode.
    type OpenDrainOutputPin: OutputPin<Error = Self::Error> + Unpin;

    /// Attempts to re-configure this pin into the new mode.
    fn into_open_drain_output_pin(
        self,
        initial_high: bool,
    ) -> Result<Self::OpenDrainOutputPin, Self::Error>;
}

/// A pin that can be turned into an [`OutputPin`] that is in push-pull mode.
pub trait IntoPushPullOutputPin: Pin {
    /// The type of an [`OutputPin`] that is in push-pull mode.
    type PushPullOutputPin: OutputPin<Error = Self::Error> + Unpin;

    /// Attempts to re-configure this pin into the new mode.
    fn into_push_pull_output_pin(
        self,
        initial_high: bool,
    ) -> Result<Self::PushPullOutputPin, Self::Error>;
}

/// A virtual pin that is not actually connected to a physical pin.
///
/// The pin will always read a fixed value, can be configured to be in any mode, and will always
/// have writes result in no-ops.
#[derive(Clone, Copy, Debug)]
pub struct NoConnect(bool);

impl NoConnect {
    /// Creates a new [`NoConnect`] that will always read the specified high/low value.
    pub fn new(value: bool) -> Self {
        NoConnect(value)
    }
}

impl Pin for NoConnect {
    type Error = futures::never::Never;
}

impl InputPin for NoConnect {
    fn poll_get(
        self: pin::Pin<&mut Self>,
        _cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<bool, Self::Error>> {
        task::Poll::Ready(Ok(false))
    }
}

impl OutputPin for NoConnect {
    fn poll_set(
        self: pin::Pin<&mut Self>,
        _cx: &mut task::Context<'_>,
        _high: bool,
    ) -> task::Poll<Result<(), Self::Error>> {
        task::Poll::Ready(Ok(()))
    }
}

impl IntoFloatingInputPin for NoConnect {
    type FloatingInputPin = Self;

    fn into_floating_input_pin(self) -> Result<Self::FloatingInputPin, Self::Error> {
        Ok(self)
    }
}

impl IntoPullUpInputPin for NoConnect {
    type PullUpInputPin = Self;

    fn into_pull_up_input_pin(self) -> Result<Self::PullUpInputPin, Self::Error> {
        Ok(self)
    }
}

impl IntoPullDownInputPin for NoConnect {
    type PullDownInputPin = Self;

    fn into_pull_down_input_pin(self) -> Result<Self::PullDownInputPin, Self::Error> {
        Ok(self)
    }
}

impl IntoOpenDrainOutputPin for NoConnect {
    type OpenDrainOutputPin = Self;

    fn into_open_drain_output_pin(
        self,
        _initial_high: bool,
    ) -> Result<Self::OpenDrainOutputPin, Self::Error> {
        Ok(self)
    }
}

impl IntoPushPullOutputPin for NoConnect {
    type PushPullOutputPin = Self;

    fn into_push_pull_output_pin(
        self,
        _initial_high: bool,
    ) -> Result<Self::PushPullOutputPin, Self::Error> {
        Ok(self)
    }
}
