//! Controller Area Network
/// Standard 11bit Identifier (0..=0x7FF)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct StandardId(u16);

impl StandardId {
    /// Creates a new standard identifier.
    pub fn new(id: u16) -> Result<StandardId, ()> {
        if id <= 0x7FF {
            Ok(StandardId(id))
        } else {
            Err(())
        }
    }
}

impl core::convert::From<StandardId> for u16 {
    fn from(id: StandardId) -> u16 {
        id.0
    }
}

impl core::convert::From<StandardId> for u32 {
    fn from(id: StandardId) -> u32 {
        id.0 as u32
    }
}

impl ExtendedId {
    /// Creates a new extended identifier.
    pub fn new(id: u32) -> Result<ExtendedId, ()> {
        if id <= 0x1FFF_FFFF {
            Ok(ExtendedId(id))
        } else {
            Err(())
        }
    }
}

impl core::convert::From<ExtendedId> for u32 {
    fn from(id: ExtendedId) -> u32 {
        id.0
    }
}

/// Extended 29bit Identifier (0..=0x1FFF_FFFF)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ExtendedId(u32);

/// CAN Identifier
///
/// The variants are wrapped in newtypes so they can only be costructed with valid values.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Id {
    /// Standard 11bit Identifier (0..=0x7FF)
    Standard(StandardId),

    /// Extended 29bit Identifier (0..=0x1FFF_FFFF)
    Extended(ExtendedId),
}

impl Id {
    /// Creates a new standard identifier.
    pub fn new_standard(id: u16) -> Result<Id, ()> {
        Ok(StandardId::new(id)?.into())
    }

    /// Creates a new extended identifier.
    pub fn new_extended(id: u32) -> Result<Id, ()> {
        Ok(ExtendedId::new(id)?.into())
    }
}

impl core::convert::From<StandardId> for Id {
    fn from(id: StandardId) -> Id {
        Id::Standard(id)
    }
}

impl core::convert::From<ExtendedId> for Id {
    fn from(id: ExtendedId) -> Id {
        Id::Extended(id)
    }
}

/// A CAN2.0 Frame
pub trait Frame: Sized {
    /// Creates a new frame.
    /// Returns an error when the data slice is too long.
    fn new(id: Id, data: &[u8]) -> Result<Self, ()>;

    /// Creates a new remote frame (RTR bit set).
    /// Returns an error when the data length code (DLC) is not valid.
    fn new_remote(id: Id, dlc: usize) -> Result<Self, ()>;

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

/// A CAN interface that is able to transmit and receive frames.
pub trait Can {
    /// Associated frame type.
    type Frame: Frame;

    /// Associated error type.
    type Error;

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
    fn try_transmit(&mut self, frame: &Self::Frame)
        -> nb::Result<Option<Self::Frame>, Self::Error>;

    /// Returns a received frame if available.
    fn try_receive(&mut self) -> nb::Result<Self::Frame, Self::Error>;
}
