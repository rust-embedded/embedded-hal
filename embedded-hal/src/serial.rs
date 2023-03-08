//! Serial traits
//!
//! # Buffered vs Unbuffered
//!
//! There is two main ways a serial receiver can operate: buffered and unbuffered.
//!
//! - **Buffered**: The serial driver has a reasonably-sized buffer in RAM. Incoming bytes
//!   are placed into the buffer automatically without user code having to actively do
//!   anything (using either DMA, or a high-priority interrupt). User code reads bytes
//!   out of the buffer. Bytes are only lost if received while buffer is full.
//! - **Unbuffered**: The serial driver has no buffer (or a buffer so small that it's negligible,
//!   for example nRF chips have a 4-byte buffer in hardware, and some STM32 chips have a 1-byte
//!   buffer). User code does "read" operations that pop bytes directly from the hardware
//!   into the user's buffer. Bytes received while the user code is not actively doing a read
//!   *at that very instant* are lost.
//!
//! For example:
//! - Linux's /dev/ttyX is buffered.
//! - Most Rust HALs offer unbuffered serial drivers at the time of writing.
//! - Some HALs (such as `embassy`) offer both buffered and unbuffered.
//!
//! There are tradeoffs when deciding which one to use. Unbuffered is the simplest, and allows for
//! the lowest memory usage since data can be transferred directly from hardware to the user's buffer, avoiding
//! the need for intermediary drivers. However, with unbuffered it's very easy to **lose data** if the code
//! spends too much time between read calls. This can be solved either by using buffered serial at the cost of
//! more RAM usage, or using hardware flow control (RTS/CTS) at the cost of using more MCU pins.
//!
//! The read traits in this API ([`ReadExact`] and [`ReadUntilIdle`]) are intended to **model unbuffered serial interfaces**.
//! Data that arrives when your code is not running a `read_*()` call **is lost**.
//!
//! Drivers should only use these traits when the use case allows for it. For example, `ReadUntilIdle` can be used
//! for packet-wise communications, but you have to ensure the protocol guarantees enough idle time between
//! packets so that you won't lose data for the next packet while processing the previous one.
//!
//! Drivers that require **buffered** serial ports should use [`embedded-io`](https://docs.rs/embedded-io) instead. These
//! traits allow for a much more `std::io`-like usage, and implementations guarantee data is not lost until the
//! (much bigger) buffer overflows.

/// Serial error
pub trait Error: core::fmt::Debug {
    /// Convert error to a generic serial error kind
    ///
    /// By using this method, serial errors freely defined by HAL implementations
    /// can be converted to a set of generic serial errors upon which generic
    /// code can act.
    fn kind(&self) -> ErrorKind;
}

impl Error for core::convert::Infallible {
    fn kind(&self) -> ErrorKind {
        match *self {}
    }
}

/// Serial error kind
///
/// This represents a common set of serial operation errors. HAL implementations are
/// free to define more specific or additional error types. However, by providing
/// a mapping to these common serial errors, generic code can still react to them.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    /// The peripheral receive buffer was overrun.
    Overrun,
    /// Received data does not conform to the peripheral configuration.
    /// Can be caused by a misconfigured device on either end of the serial line.
    FrameFormat,
    /// Parity check failed.
    Parity,
    /// Serial line is too noisy to read valid data.
    Noise,
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
            Self::Parity => write!(f, "Parity check failed"),
            Self::Noise => write!(f, "Serial line is too noisy to read valid data"),
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

/// Serial error type trait
///
/// This just defines the error type, to be used by the other traits.
pub trait ErrorType {
    /// Error type
    type Error: Error;
}

impl<T: ErrorType> ErrorType for &mut T {
    type Error = T::Error;
}

/// Read an exact amount of words from an *unbuffered* serial interface
///
/// Some serial interfaces support different data sizes (8 bits, 9 bits, etc.);
/// This can be encoded in this trait via the `Word` type parameter.
pub trait ReadExact<Word: 'static + Copy = u8>: ErrorType {
    /// Read an exact amount of words.
    ///
    /// This does not return until exactly `read.len()` words have been read.
    fn read_exact(&mut self, read: &mut [Word]) -> Result<(), Self::Error>;
}

impl<T: ReadExact<Word>, Word: 'static + Copy> ReadExact<Word> for &mut T {
    fn read_exact(&mut self, read: &mut [Word]) -> Result<(), Self::Error> {
        T::read_exact(self, read)
    }
}

/// Read words from an *unbuffered* serial interface, until the line becomes idle.
///
/// Some serial interfaces support different data sizes (8 bits, 9 bits, etc.);
/// This can be encoded in this trait via the `Word` type parameter.
pub trait ReadUntilIdle<Word: 'static + Copy = u8>: ErrorType {
    /// Read words until the line becomes idle.
    ///
    /// Returns the amount of words received.
    ///
    /// This returns at the earliest of either:
    /// - at least 1 word has been received, and then the line becomes idle
    /// - exactly `read.len()` words have been read (the buffer is full)
    ///
    /// The serial line is considered idle after a timeout of it being constantly
    /// at high level. The exact timeout is implementation-defined, but it should be
    /// short, around 1 or 2 words' worth of time.
    fn read_until_idle(&mut self, read: &mut [Word]) -> Result<usize, Self::Error>;
}

impl<T: ReadUntilIdle<Word>, Word: 'static + Copy> ReadUntilIdle<Word> for &mut T {
    fn read_until_idle(&mut self, read: &mut [Word]) -> Result<usize, Self::Error> {
        T::read_until_idle(self, read)
    }
}

/// Write half of a serial interface.
pub trait Write<Word: Copy = u8>: ErrorType {
    /// Writes a slice, blocking until everything has been written
    ///
    /// An implementation can choose to buffer the write, returning `Ok(())`
    /// after the complete slice has been written to a buffer, but before all
    /// words have been sent via the serial interface. To make sure that
    /// everything has been sent, call [`flush`](Write::flush) after this function returns.
    fn write(&mut self, buffer: &[Word]) -> Result<(), Self::Error>;

    /// Block until the serial interface has sent all buffered words
    fn flush(&mut self) -> Result<(), Self::Error>;
}

impl<T: Write<Word>, Word: Copy> Write<Word> for &mut T {
    fn write(&mut self, buffer: &[Word]) -> Result<(), Self::Error> {
        T::write(self, buffer)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        T::flush(self)
    }
}
