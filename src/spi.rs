//! Serial Peripheral Interface

use nb;

/// Full duplex (master mode)
///
/// # Notes
///
/// - It's the task of the user of this interface to manage the slave select lines
///
/// - Due to how full duplex SPI works each `read` call must be preceded by a `send` call.
///
/// - Some SPIs can work with 8-bit *and* 16-bit words. You can overload this trait with different
/// `Word` types to allow operation in both modes.
pub trait FullDuplex<Word> {
    /// An enumeration of SPI errors
    type Error;

    /// Reads the word stored in the shift register
    ///
    /// **NOTE** A word must be sent to the slave before attempting to call this
    /// method.
    fn read(&mut self) -> nb::Result<Word, Self::Error>;

    /// Sends a word to the slave
    fn send(&mut self, word: Word) -> nb::Result<(), Self::Error>;
}

/// Write (master mode)
///
/// # Notes
///
/// - It's the task of the user of this interface to manage the slave select lines
///
/// - The slave select line shouldn't be released before the `flush` call succeeds
#[cfg(feature = "unproven")]
pub trait Send<Word> {
    /// An enumeration of SPI errors
    type Error;

    /// Sends a word to the slave
    fn send(&mut self, word: Word) -> nb::Result<(), Self::Error>;

    /// Ensures that none of the previously written words are still buffered
    fn flush(&mut self) -> nb::Result<(), Self::Error>;
}

/// Write (master mode)
#[cfg(feature = "unproven")]
pub mod full_duplex {
    /// Default implementation of `spi::FullDuplex<W>` for implementers of
    /// `spi::Send<W>`
    ///
    /// Also needs a `send` method to return the byte read during the last send
    pub trait Default<W>: ::spi::Send<W> {
        /// Reads the word stored in the shift register
        ///
        /// **NOTE** A word must be sent to the slave before attempting to call this
        /// method.
        fn read(&mut self) -> nb::Result<W, Self::Error>;
    }

    impl<W, S> ::spi::FullDuplex<W> for S
    where
        S: Default<W>,
        W: Clone,
    {
        type Error = S::Error;

        fn read(&mut self) -> nb::Result<W, Self::Error> {
            self.read()
        }

        fn send(&mut self, word: W) -> nb::Result<(), Self::Error> {
            self.send(word)
        }
    }
}

/// Clock polarity
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Polarity {
    /// Clock signal low when idle
    IdleLow,
    /// Clock signal high when idle
    IdleHigh,
}

/// Clock phase
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// Data in "captured" on the first clock transition
    CaptureOnFirstTransition,
    /// Data in "captured" on the second clock transition
    CaptureOnSecondTransition,
}

/// SPI mode
#[derive(Clone, Copy, PartialEq, Eq)]
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
