//! Async CAN API

/// An async CAN interface that is able to transmit and receive frames.
pub trait Can {
    /// Associated frame type.
    type Frame: crate::Frame;

    /// Associated error type.
    type Error: crate::Error;

    /// Puts a frame in the transmit buffer.
    /// Awaits until space is available in the transmit buffer.
    async fn transmit(&mut self, frame: &Self::Frame) -> Result<(), Self::Error>;

    /// Awaits until a frame was received or an error occurred.
    async fn receive(&mut self) -> Result<Self::Frame, Self::Error>;
}
