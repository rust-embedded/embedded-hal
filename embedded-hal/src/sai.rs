//! Blocking Synchronous Audio Interface API

use crate::private;

/// SAI mode
///
/// Note: This trait is sealed and should not be implemented outside of this crate.
pub trait SaiMode: private::Sealed + 'static {}

/// Standard I2S mode
pub struct I2sMode;

/// I2S mode with left/MSB alignement
pub struct I2sLeftMode;

/// TDM mode
pub struct TdmMode;

impl SaiMode for I2sMode {}
impl SaiMode for I2sLeftMode {}
impl SaiMode for TdmMode {}

/// I2S trait
pub trait I2s<W>: 
    I2sRx<W> +
    I2sTx<W> {}

/// I2S receiver-only trait
pub trait I2sRx<W>: 
    SaiRx<I2sMode, W, 2> +
    SaiRxInterlaced<I2sMode, W, 2> {}

/// I2S transmitter-only trait
pub trait I2sTx<W>: 
    SaiTx<I2sMode, W, 2> +
    SaiTxInterlaced<I2sMode, W, 2> {}


/// I2S left/MSB aligned trait
pub trait I2sLeft<W>:
    I2sLeftRx<W> +
    I2sLeftTx<W> {}

/// I2S left/MSB aligned  receiver-only trait
pub trait I2sLeftRx<W>: 
    SaiRx<I2sLeftMode, W, 2> +
    SaiRxInterlaced<I2sLeftMode, W, 2> {}

/// I2S left/MSB aligned  transmitter-only trait
pub trait I2sLeftTx<W>: 
    SaiTx<I2sLeftMode, W, 2> +
    SaiTxInterlaced<I2sLeftMode, W, 2> {}

    
/// TDM receiver trait
pub trait TdmRx<W, const CHANNELS: usize>:
    SaiRx<TdmMode, W, CHANNELS> +
    SaiRxInterlaced<TdmMode, W, CHANNELS> {}

/// TDM transmitter trait
pub trait TdmTx<W, const CHANNELS: usize>:
    SaiTx<TdmMode, W, CHANNELS> +
    SaiTxInterlaced<TdmMode, W, CHANNELS> {}


/// SAI RX trait
pub trait SaiRx<M: SaiMode, W, const CHANNELS: usize> {
    /// Error type
    type Error: core::fmt::Debug;

    /// Reads enough bytes to fill all `CHANNELS` with `samples`.
    fn read<'w>(&mut self, samples: [&'w mut [W]; CHANNELS]) -> Result<(), Self::Error>;
}

/// SAI RX interlaced trait
pub trait SaiRxInterlaced<M: SaiMode, W, const CHANNELS: usize> {
    /// Error type
    type Error: core::fmt::Debug;
    
    /// Reads enough bytes to fill the interlaced `samples` buffer.
    fn read_interlaced<'w>(&mut self, samples: &'w mut [W]) -> Result<(), Self::Error>;
}

/// SAI TX trait
pub trait SaiTx<M: SaiMode, W, const CHANNELS: usize> {
    /// Error type
    type Error: core::fmt::Debug;

    /// Sends `samples` to the `CHANNELS`.
    fn write<'w>(&mut self, samples: [&'w [W]; CHANNELS]) -> Result<(), Self::Error>;

    /// Sends `samples` to the `CHANNELS`.
    fn write_iter<WI>(&mut self, samples: [WI; CHANNELS]) -> Result<(), Self::Error>
    where WI: core::iter::IntoIterator<Item = W>;
}

/// SAI TX interlaced trait
pub trait SaiTxInterlaced<M: SaiMode, W, const CHANNELS: usize> {
    /// Error type
    type Error: core::fmt::Debug;

    /// Sends `samples` from an interlaced buffer.
    fn write_interlaced<'w>(&mut self, samples: &'w mut [W]) -> Result<(), Self::Error>;

    /// Sends `samples` to the `CHANNELS`.
    fn write_interlaced_iter<WI>(&mut self, samples: WI) -> Result<(), Self::Error>
    where WI: core::iter::IntoIterator<Item = W>;
}

/// SAI error
pub trait Error: core::fmt::Debug {
    /// Convert error to a generic SAI error kind
    ///
    /// By using this method, SAI errors freely defined by HAL implementations
    /// can be converted to a set of generic SAI errors upon which generic
    /// code can act.
    fn kind(&self) -> ErrorKind;
}

impl Error for core::convert::Infallible {
    fn kind(&self) -> ErrorKind {
        match *self {}
    }
}

/// SAI error kind
///
/// This represents a common set of SPI operation errors. HAL implementations are
/// free to define more specific or additional error types. However, by providing
/// a mapping to these common SPI errors, generic code can still react to them.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    // /// The peripheral receive buffer was overrun
    // Overrun,
    // /// Multiple devices on the SPI bus are trying to drive the slave select pin, e.g. in a multi-master setup
    // ModeFault,
    // /// Received data does not conform to the peripheral configuration
    // FrameFormat,
    // /// An error occurred while asserting or deasserting the Chip Select pin.
    // ChipSelectFault,
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
            Self::Other => write!(
                f,
                "A different error occurred. The original error may contain more information"
            ),
        }
    }
}

/// SAI error type trait
///
/// This just defines the error type, to be used by the other SAI traits.
pub trait ErrorType {
    /// Error type
    type Error: Error;
}

impl<T: ErrorType> ErrorType for &mut T {
    type Error = T::Error;
}