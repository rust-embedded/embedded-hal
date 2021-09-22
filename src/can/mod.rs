//! Controller Area Network

pub mod blocking;
pub mod nb;

mod id;

pub use id::*;

/// A CAN2.0 Frame
pub trait Frame: Sized {
    /// Creates a new frame.
    /// Returns an error when the data slice is too long.
    fn new(id: impl Into<Id>, data: &[u8]) -> Result<Self, ()>;

    /// Creates a new remote frame (RTR bit set).
    /// Returns an error when the data length code (DLC) is not valid.
    fn new_remote(id: impl Into<Id>, dlc: usize) -> Result<Self, ()>;

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
    fn id(&self) -> Id;

    /// Returns the data length code (DLC) which is in the range 0..8.
    ///
    /// For data frames the DLC value always matches the length of the data.
    /// Remote frames do not carry any data, yet the DLC can be greater than 0.
    fn dlc(&self) -> usize;

    /// Returns the frame data (0..8 bytes in length).
    fn data(&self) -> &[u8];
}
