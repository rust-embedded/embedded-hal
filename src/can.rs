//! Controller Area Network

/// A CAN2.0 Frame
pub trait Frame {
    /// Creates a new frame with a standard identifier.
    fn new_standard(id: u32, data: &[u8]) -> Self;

    /// Creates a new frame with an extended identifier.
    fn new_extended(id: u32, data: &[u8]) -> Self;

    /// Marks this frame as a remote frame (by setting the RTR bit).
    fn with_rtr(&mut self, dlc: usize) -> &mut Self;

    /// Returns true if this frame is a extended frame.
    fn is_extended(&self) -> bool;

    /// Returns true if this frame is a standard frame.
    fn is_standard(&self) -> bool {
        !self.is_extended()
    }

    /// Returns true if this frame is a remote frame.
    fn is_remote_frame(&self) -> bool;

    /// Returns true if this frame is a data frame.
    fn is_data_frame(&self) -> bool {
        !self.is_remote_frame()
    }

    /// Returns the frame identifier.
    fn id(&self) -> u32;

    /// Returns the data length code (DLC) which is in the range 0..8.
    ///
    /// For data frames the DLC value always matches the length of the data.
    /// Remote frames do not carry any data, yet the DLC can be greater than 0.
    fn dlc(&self) -> usize;

    /// Returns the frame data (0..8 bytes in length).
    fn data(&self) -> &[u8];
}

/// A CAN interface that is able to transmit frames.
pub trait Transmitter {
    /// Associated frame type.
    type Frame: Frame;

    /// Associated error type.
    type Error;

    /// Puts a frame in the transmit buffer.
    ///
    /// If the buffer is full, this function will try to replace a lower priority frame
    /// and return it. This is to avoid the priority inversion problem.
    fn transmit(&mut self, frame: &Self::Frame) -> nb::Result<Option<Self::Frame>, Self::Error>;
}

/// A CAN interface that is able to receive frames.
pub trait Receiver {
    /// Associated frame type.
    type Frame: Frame;

    /// Associated error type.
    type Error;

    /// Returns a received frame if available.
    fn receive(&mut self) -> nb::Result<Self::Frame, Self::Error>;
}
