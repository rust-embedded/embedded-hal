//! Controller Area Network (CAN) traits

#![warn(missing_docs)]
#![no_std]

pub mod blocking;
pub mod nb;

mod id;

pub use id::*;

/// A CAN2.0 Frame
pub trait Frame: Sized {
    /// Creates a new frame.
    ///
    /// This will return `None` if the data slice is too long.
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self>;

    /// Creates a new remote frame (RTR bit set).
    ///
    /// This will return `None` if the data length code (DLC) is not valid.
    fn new_remote(id: impl Into<Id>, dlc: usize) -> Option<Self>;

    /// Returns true if this frame is an extended frame.
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

/// A CAN FD Frame
pub trait FdFrame: Sized {
    /// Creates a new fd frame.
    ///
    /// This will return `None` if the data slice is not 0-8, 12, 16, 20, 24, 32, 48, or 64 bytes long.
    fn new(id: impl Into<Id>, brs: bool, data: &[u8]) -> Option<Self>;

    /// Returns true if this frame uses bit rate switching (BRS).
    fn is_brs(&self) -> bool;

    /// Returns true if this frame is an extended frame.
    fn is_extended(&self) -> bool;

    /// Returns true if this frame is a standard frame.
    fn is_standard(&self) -> bool {
        !self.is_extended()
    }

    /// Returns the frame identifier.
    fn id(&self) -> Id;

    /// Returns the data length code (DLC) which is in the range 0..15.
    ///
    /// For frame lengths:
    /// - 0..8 bytes: DLC = length
    /// - 12 bytes: DLC = 9
    /// - 16 bytes: DLC = 10
    /// - 20 bytes: DLC = 11
    /// - 24 bytes: DLC = 12
    /// - 32 bytes: DLC = 13
    /// - 48 bytes: DLC = 14
    /// - 64 bytes: DLC = 15
    fn dlc(&self) -> usize;

    /// Returns the length of the frame data in bytes.
    fn length(&self) -> usize {
        match self.dlc() {
            0..=8 => self.dlc(),
            9 => 12,
            10 => 16,
            11 => 20,
            12 => 24,
            13 => 32,
            14 => 48,
            15 => 64,
            _ => 0, // Should never happen
        }
    }

    /// Returns the frame data (0..64 bytes in length).
    fn data(&self) -> &[u8];
}

/// CAN error
pub trait Error: core::fmt::Debug {
    /// Convert error to a generic CAN error kind
    ///
    /// By using this method, CAN errors freely defined by HAL implementations
    /// can be converted to a set of generic serial errors upon which generic
    /// code can act.
    fn kind(&self) -> ErrorKind;
}

impl Error for core::convert::Infallible {
    fn kind(&self) -> ErrorKind {
        match *self {}
    }
}

/// CAN error kind
///
/// This represents a common set of CAN operation errors. HAL implementations are
/// free to define more specific or additional error types. However, by providing
/// a mapping to these common CAN errors, generic code can still react to them.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum ErrorKind {
    /// The peripheral receive buffer was overrun.
    Overrun,

    // MAC sublayer errors
    /// A bit error is detected at that bit time when the bit value that is
    /// monitored differs from the bit value sent.
    Bit,

    /// A stuff error is detected at the bit time of the sixth consecutive
    /// equal bit level in a frame field that shall be coded by the method
    /// of bit stuffing.
    Stuff,

    /// Calculated CRC sequence does not equal the received one.
    Crc,

    /// A form error shall be detected when a fixed-form bit field contains
    /// one or more illegal bits.
    Form,

    /// An ACK  error shall be detected by a transmitter whenever it does not
    /// monitor a dominant bit during the ACK slot.
    Acknowledge,

    /// A different error occurred. The original error may contain more information.
    Other,
}

impl Error for ErrorKind {
    fn kind(&self) -> ErrorKind {
        *self
    }
}

impl core::error::Error for ErrorKind {}

impl core::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Overrun => write!(f, "The peripheral receive buffer was overrun"),
            Self::Bit => write!(
                f,
                "Bit value that is monitored differs from the bit value sent"
            ),
            Self::Stuff => write!(f, "Sixth consecutive equal bits detected"),
            Self::Crc => write!(f, "Calculated CRC sequence does not equal the received one"),
            Self::Form => write!(
                f,
                "A fixed-form bit field contains one or more illegal bits"
            ),
            Self::Acknowledge => write!(f, "Transmitted frame was not acknowledged"),
            Self::Other => write!(
                f,
                "A different error occurred. The original error may contain more information"
            ),
        }
    }
}
