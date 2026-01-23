//! Blocking CAN API

/// A blocking CAN interface that is able to transmit and receive frames.
pub trait Can {
    /// Associated frame type.
    type Frame: crate::Frame;

    /// Associated error type.
    type Error: crate::Error;

    /// Puts a frame in the transmit buffer. Blocks until space is available in
    /// the transmit buffer.
    fn transmit(&mut self, frame: &Self::Frame) -> Result<(), Self::Error>;

    /// Blocks until a frame was received or an error occurred.
    fn receive(&mut self) -> Result<Self::Frame, Self::Error>;
}

/// A blocking CAN FD interface that is able to transmit and receive fd frames.
pub trait CanFd {
    /// Associated frame type.
    type FdFrame: crate::FdFrame;

    /// Associated error type.
    type Error: crate::Error;

    /// Puts a fd frame in the transmit buffer. Blocks until space is available in
    /// the transmit buffer.
    fn transmit_fd(&mut self, frame: &Self::FdFrame) -> Result<(), Self::Error>;

    /// Blocks until a fd frame was received or an error occurred.
    fn receive_fd(&mut self) -> Result<Self::FdFrame, Self::Error>;
}
