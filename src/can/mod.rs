//! Controller Area Network

pub mod nb;

mod id;

pub use self::id::*;
pub use self::nb::*;

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

/// CAN error
pub trait Error: core::fmt::Debug {
    /// Convert error to a generic CAN error kind
    ///
    /// By using this method, CAN errors freely defined by HAL implementations
    /// can be converted to a set of generic serial errors upon which generic
    /// code can act.
    fn kind(&self) -> ErrorKind;
}

/// CAN error kind
///
/// This represents a common set of CAN operation errors. HAL implementations are
/// free to define more specific or additional error types. However, by providing
/// a mapping to these common CAN errors, generic code can still react to them.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

impl core::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ErrorKind::Overrun => write!(f, "The peripheral receive buffer was overrun"),
            ErrorKind::Bit => write!(
                f,
                "Bit value that is monitored differs from the bit value sent"
            ),
            ErrorKind::Stuff => write!(f, "Sixth consecutive equal bits detected"),
            ErrorKind::Crc => write!(f, "Calculated CRC sequence does not equal the received one"),
            ErrorKind::Form => write!(
                f,
                "A fixed-form bit field contains one or more illegal bits"
            ),
            ErrorKind::Acknowledge => write!(f, "Transmitted frame was not acknowledged"),
            ErrorKind::Other => write!(
                f,
                "A different error occurred. The original error may contain more information"
            ),
        }
    }
}
