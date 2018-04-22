//! Controller Area Network

use nb;

/// A type that can either be `BaseId` or `ExtendedId`
#[cfg(feature = "unproven")]
pub trait Id {
    /// The (11-bit) BaseId variant.
    type BaseId;

    /// The (29-bit) ExtendedId variant.
    type ExtendedId;

    /// Returns `Some(base_id)` if this Can-ID is 11-bit.
    /// Returns `None` if this Can-ID is 29-bit.
    fn base_id(&self) -> Option<Self::BaseId>;

    /// Returns `Some(extended_id)` if this Can-ID is 29-bit.
    /// Returns `None` if this Can-ID is 11-bit.
    fn extended_id(&self) -> Option<Self::ExtendedId>;
}


/// A Can Frame
#[cfg(feature = "unproven")]
pub trait Frame {
    /// The Id type of this Frame
    type Id: Id;

    /// Returns true if this `Frame` is a remote frame
    fn is_remote_frame(&self) -> bool;

    /// Returns true if this `Frame` is a data frame
    fn is_data_frame(&self) -> bool;

    /// Returns true if this `Frame` is a extended id frame
    fn is_base_id_frame(&self) -> bool {
        self.id().base_id().is_some()
    }

    /// Returns true if this `Frame` is a extended id frame
    fn is_extended_id_frame(&self) -> bool {
        self.id().extended_id().is_some()
    }

    /// Returns the Can-ID
    fn id(&self) -> Self::Id;

    /// Returns `Some(Data)` if data frame.
    /// Returns `None` if remote frame.
    fn data(&self) -> Option<&[u8]>;
}

/// A Can-FD Frame
///
/// A "ordinary" Can-Frame must also be representable by this type.
#[cfg(feature = "unproven")]
pub trait FdFrame {
    /// The Id type of this Frame
    type Id: Id;

    /// Returns true if this frame would/has be(en) transmitted as a Can-Fd frame.
    /// Returns false if this frame would/has be(en) transmitted as a "ordinary" Can frame.
    fn is_fd_frame(&self) -> bool;

    /// Returns true if this `Frame` is a remote frame
    fn is_remote_frame(&self) -> bool;

    /// Returns true if this `Frame` is a data frame
    fn is_data_frame(&self) -> bool;

    /// Returns true if this `Frame` is a extended id frame
    fn is_base_id_frame(&self) -> bool {
        self.id().base_id().is_some()
    }

    /// Returns true if this `Frame` is a extended id frame
    fn is_extended_id_frame(&self) -> bool {
        self.id().extended_id().is_some()
    }

    /// Returns the Can-ID
    fn id(&self) -> Self::Id;

    /// Returns `Some(Data)` if data frame.
    /// Returns `None` if remote frame.
    fn data(&self) -> Option<&[u8]>;
}


/// A CAN interface
#[cfg(feature = "unproven")]
pub trait Interface {
    /// The Can Frame this Interface operates on
    type Frame: Frame;

    /// The Interface Error type
    type Error;

    /// Return the available `Frame` with the highest priority (lowest ID).
    ///
    /// NOTE: Can-FD Frames will not be received using this function.
    fn receive(&mut self) -> nb::Result<Self::Frame, Self::Error>;

    /// Put a `Frame` in the transmit buffer (or a free mailbox).
    ///
    /// If the buffer is full, this function will try to replace a lower priority `Frame`
    /// and return it. This is to avoid the priority inversion problem.
    fn transmit(&mut self, frame: &Self::Frame) -> nb::Result<Option<Self::Frame>, Self::Error>;

    /// Returns true if a call to `transmit(frame)` (and if the interface supports Can-FD)
    /// `transmit_fd(fd_frame)` would return a `Frame` or `WouldBlock`.
    fn transmit_buffer_full(&self) -> bool;

}

/// A CAN interface also supporting Can-FD
#[cfg(feature = "unproven")]
pub trait FdInterface: Interface {
    /// The Can Frame this Interface operates on
    type FdFrame: FdFrame;

    /// Read the available `FdFrame` with the highest priority (lowest ID).
    fn receive(&mut self) -> nb::Result<Self::FdFrame, Self::Error>;

    /// Put a `FdFrame` in the transmit buffer (or a free mailbox).
    ///
    /// If the buffer is full, this function will try to replace a lower priority `FdFrame`
    /// and return it. This is to avoid the priority inversion problem.
    fn transmit(&mut self, frame: &Self::FdFrame) -> nb::Result<Option<Self::FdFrame>, Self::Error>;

    /// Returns true if a call to `transmit(frame)` or `transmit_fd(fd_frame)`
    /// would return a `FdFrame` or `WouldBlock`.
    fn transmit_buffer_full(&self) -> bool;
}