//! Non-blocking CAN API

/// A CAN interface that is able to transmit and receive frames.
pub trait Can {
    /// Associated frame type.
    type Frame: crate::can::Frame;

    /// Associated error type.
    type Error: crate::can::Error;

    /// Puts a frame in the transmit buffer to be sent on the bus.
    ///
    /// If the transmit buffer is full, this function will try to replace a pending
    /// lower priority frame and return the frame that was replaced.
    /// Returns `Err(WouldBlock)` if the transmit buffer is full and no frame can be
    /// replaced.
    ///
    /// # Notes for implementers
    ///
    /// * Frames of equal identifier shall be transmited in FIFO fashion when more
    ///   than one transmit buffer is available.
    /// * When replacing pending frames make sure the frame is not in the process of
    ///   being send to the bus.
    fn transmit(&mut self, frame: &Self::Frame) -> nb::Result<Option<Self::Frame>, Self::Error>;

    /// Returns a received frame if available.
    fn receive(&mut self) -> nb::Result<Self::Frame, Self::Error>;
}
