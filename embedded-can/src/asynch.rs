//! Async CAN API

use core::future::Future;

/// An async CAN interface that is able to transmit frames.
pub trait CanTx {
    /// Associated frame type.
    type Frame: crate::Frame;

    /// Associated error type.
    type Error: crate::Error;

    /// Puts a frame in the transmit buffer or awaits until space is available
    /// in the transmit buffer.
    fn transmit(&mut self, frame: &Self::Frame) -> impl Future<Output = Result<(), Self::Error>>;
}

/// An async CAN interface that is able to receive frames.
pub trait CanRx {
    /// Associated frame type.
    type Frame: crate::Frame;

    /// Associated error type.
    type Error: crate::Error;

    /// Awaits until a frame was received or an error occurs.
    fn receive(&mut self) -> impl Future<Output = Result<Self::Frame, Self::Error>>;
}
