//! Async CAN API

#![allow(async_fn_in_trait)]

/// An async CAN interface that is able to transmit and receive frames.
pub trait Can {
    /// Associated frame type.
    type Frame: crate::Frame;

    /// Associated error type.
    type Error: crate::Error;

    /// Puts a frame in the transmit buffer. Waits until space is available in
    /// the transmit buffer.
    async fn transmit(&mut self, frame: &Self::Frame) -> Result<(), Self::Error>;

    /// Waits until a frame was received or an error occurred.
    async fn receive(&mut self) -> Result<Self::Frame, Self::Error>;
}
