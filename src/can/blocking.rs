//! Blocking CAN API

/// A blocking CAN interface that is able to transmit and receive frames.
pub trait Can {
    /// Associated frame type.
    type Frame: crate::can::Frame;

    /// Associated error type.
    type Error: crate::can::Error;

    /// Puts a frame in the transmit buffer. Blocks until space is available in
    /// the transmit buffer.
    fn transmit(&mut self, frame: &Self::Frame) -> Result<(), Self::Error>;

    /// Blocks until a frame was received or an error occured.
    fn receive(&mut self) -> Result<Self::Frame, Self::Error>;
}
