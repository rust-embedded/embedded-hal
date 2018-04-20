//! Controller Area Network

use core::convert::TryInto;
use core::convert::TryFrom;

use nb;

/// A type that can either be `BaseId` or `ExtendedId`
#[cfg(feature = "unproven")]
pub trait Id: Sized {
    /// The BaseId variant
    type BaseId: BaseId + Into<Self> + TryFrom<Self>;

    /// The ExtendedId variant
    type ExtendedId: ExtendedId + Into<Self> + TryFrom<Self>;
}

/// A Can (11-bit) ID
#[cfg(feature = "unproven")]
pub trait BaseId: Sized {
    /// A generic ID type that encapsulate this type
    type Id: Id + From<Self> + TryInto<Self>;
}

/// A Can Extended (28-bit) ID
#[cfg(feature = "unproven")]
pub trait ExtendedId: Sized {
    /// A generic ID type that encapsulate this type
    type Id: Id + From<Self> + TryInto<Self>;
}

/// A Can Frame
#[cfg(feature = "unproven")]
pub trait Frame {
    /// The Id type of this Frame
    type Id: Id;

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