//! SPI traits

pub mod blocking;
pub mod nb;

/// Clock polarity
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Polarity {
    /// Clock signal low when idle
    IdleLow,
    /// Clock signal high when idle
    IdleHigh,
}

/// Clock phase
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    /// Data in "captured" on the first clock transition
    CaptureOnFirstTransition,
    /// Data in "captured" on the second clock transition
    CaptureOnSecondTransition,
}

/// SPI mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mode {
    /// Clock polarity
    pub polarity: Polarity,
    /// Clock phase
    pub phase: Phase,
}

/// Helper for CPOL = 0, CPHA = 0
pub const MODE_0: Mode = Mode {
    polarity: Polarity::IdleLow,
    phase: Phase::CaptureOnFirstTransition,
};

/// Helper for CPOL = 0, CPHA = 1
pub const MODE_1: Mode = Mode {
    polarity: Polarity::IdleLow,
    phase: Phase::CaptureOnSecondTransition,
};

/// Helper for CPOL = 1, CPHA = 0
pub const MODE_2: Mode = Mode {
    polarity: Polarity::IdleHigh,
    phase: Phase::CaptureOnFirstTransition,
};

/// Helper for CPOL = 1, CPHA = 1
pub const MODE_3: Mode = Mode {
    polarity: Polarity::IdleHigh,
    phase: Phase::CaptureOnSecondTransition,
};

/// SPI error
pub trait Error: core::fmt::Debug {
    /// Convert error to a generic SPI error kind
    ///
    /// By using this method, SPI errors freely defined by HAL implementations
    /// can be converted to a set of generic SPI errors upon which generic
    /// code can act.
    fn kind(&self) -> ErrorKind;
}

/// SPI error kind
///
/// This represents a common set of SPI operation errors. HAL implementations are
/// free to define more specific or additional error types. However, by providing
/// a mapping to these common SPI errors, generic code can still react to them.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    /// The peripheral receive buffer was overrun
    Overrun,
    /// Multiple devices on the SPI bus are trying to drive the slave select pin, e.g. in a multi-master setup
    ModeFault,
    /// Received data does not conform to the peripheral configuration
    FrameFormat,
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
            Self::Overrun => write!(f, "The peripheral receive buffer was overrun"),
            Self::ModeFault => write!(
                f,
                "Multiple devices on the SPI bus are trying to drive the slave select pin"
            ),
            Self::FrameFormat => write!(
                f,
                "Received data does not conform to the peripheral configuration"
            ),
            Self::Other => write!(
                f,
                "A different error occurred. The original error may contain more information"
            ),
        }
    }
}

/// ManagedChipSelect marker trait indicates the CS pin is managed by the underlying driver.
///
/// This specifies that `spi` operations will be grouped:
/// preceded by asserting the CS pin, and followed by
/// de-asserting the CS pin, prior to returning from the method.
///
/// This is important for shared bus access to ensure that only one CS can be asserted at a given time.
/// To support shared use, drivers should require this (and not manage their own CS pins).
///
/// For usage examples see: [`crate::spi::blocking::SpiWithCs`].
pub trait ManagedChipSelect {}
