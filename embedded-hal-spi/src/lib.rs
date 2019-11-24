//! # embedded-hal-spi
//!
//! Traits for driving a Serial Peripheral Interface (SPI) device for the
//! purposes of accessing other devices on that bus. You should use the
//! [`embedded-hal`](https://crates.io/crates/embedded-hal) crate if you want
//! a stable version.
//!
//! Available in both blocking and non-blocking variants.

#![deny(missing_docs)]
#![no_std]

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

/// Blocking traits
pub mod blocking {
    //! Blocking SPI API

    /// Blocking transfer
    pub trait Transfer<W> {
        /// Error type
        type Error;

        /// Sends `words` to the slave. Returns the `words` received from the slave
        fn transfer<'w>(&mut self, words: &'w mut [W]) -> Result<&'w [W], Self::Error>;
    }

    /// Blocking write
    pub trait Write<W> {
        /// Error type
        type Error;

        /// Sends `words` to the slave, ignoring all the incoming words
        fn write(&mut self, words: &[W]) -> Result<(), Self::Error>;
    }

    /// Blocking write (iterator version)
    pub trait WriteIter<W> {
        /// Error type
        type Error;

        /// Sends `words` to the slave, ignoring all the incoming words
        fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
        where
            WI: IntoIterator<Item = W>;
    }

    /// Blocking transfer
    pub mod transfer {
        /// Default implementation of `blocking::Transfer<W>` for implementers of
        /// `spi::FullDuplex<W>`
        pub trait Default<W>: crate::FullDuplex<W> {}

        impl<W, S> crate::blocking::Transfer<W> for S
        where
            S: Default<W>,
            W: Clone,
        {
            type Error = S::Error;

            fn transfer<'w>(&mut self, words: &'w mut [W]) -> Result<&'w [W], S::Error> {
                use nb::block;
                for word in words.iter_mut() {
                    block!(self.send(word.clone()))?;
                    *word = block!(self.read())?;
                }

                Ok(words)
            }
        }
    }

    /// Blocking write
    pub mod write {
        /// Default implementation of `blocking::Write<W>` for implementers of `spi::FullDuplex<W>`
        pub trait Default<W>: crate::FullDuplex<W> {}

        impl<W, S> crate::blocking::Write<W> for S
        where
            S: Default<W>,
            W: Clone,
        {
            type Error = S::Error;

            fn write(&mut self, words: &[W]) -> Result<(), S::Error> {
                use nb::block;
                for word in words {
                    block!(self.send(word.clone()))?;
                    block!(self.read())?;
                }

                Ok(())
            }
        }
    }

    /// Blocking write (iterator version)
    pub mod write_iter {
        /// Default implementation of `blocking::WriteIter<W>` for implementers of
        /// `spi::FullDuplex<W>`
        pub trait Default<W>: crate::FullDuplex<W> {}

        impl<W, S> crate::blocking::WriteIter<W> for S
        where
            S: Default<W>,
            W: Clone,
        {
            type Error = S::Error;

            fn write_iter<WI>(&mut self, words: WI) -> Result<(), S::Error>
            where
                WI: IntoIterator<Item = W>,
            {
                use nb::block;
                for word in words.into_iter() {
                    block!(self.send(word.clone()))?;
                    block!(self.read())?;
                }

                Ok(())
            }
        }
    }
}
