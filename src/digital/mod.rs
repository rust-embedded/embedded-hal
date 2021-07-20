//! Digital I/O traits

pub mod blocking;

use core::{convert::From, ops::Not};

/// Digital output pin state
///
/// Conversion from `bool` and logical negation are also implemented
/// for this type.
/// ```rust
/// # use embedded_hal::digital::blocking::PinState;
/// let state = PinState::from(false);
/// assert_eq!(state, PinState::Low);
/// assert_eq!(!state, PinState::High);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PinState {
    /// Low pin state
    Low,
    /// High pin state
    High,
}

impl From<bool> for PinState {
    fn from(value: bool) -> Self {
        match value {
            false => PinState::Low,
            true => PinState::High,
        }
    }
}

impl Not for PinState {
    type Output = PinState;

    fn not(self) -> Self::Output {
        match self {
            PinState::High => PinState::Low,
            PinState::Low => PinState::High,
        }
    }
}
