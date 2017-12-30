//! Serial Peripheral Interface
use core::marker::Unsize;
use nb;
use dma;

/// Full duplex (master mode)
///
/// # Notes
///
/// - It's the task of the user of this interface to manage the slave select
///   lines
///
/// - Due to how full duplex SPI works each `send` call must be followed by a
///   `read` call to avoid overruns.
///
/// - Some SPIs can work with 8-bit *and* 16-bit words. You can overload this
///   trait with different `Word` types to allow operation in both modes.
pub trait FullDuplex<Word> {
    /// An enumeration of SPI errors
    ///
    /// Possible errors
    ///
    /// - *overrun*, the shift register was not `read` between two consecutive
    ///   `send` calls.
    type Error;

    /// Reads the word stored in the shift register
    ///
    /// **NOTE** A word must be sent to the slave before attempting to call this
    /// method.
    fn read(&mut self) -> nb::Result<Word, Self::Error>;

    /// Sends a word to the slave
    fn send(&mut self, word: Word) -> nb::Result<(), Self::Error>;
}

/// DMA Write mode
pub trait DmaWrite<B, Word>
where
    Self: Sized,
    B: Unsize<[Word]> + 'static
{
    /// Sends `words` to the slave.
    type Transfer: dma::Transfer<Item = &'static mut B, Payload = Self>;

    /// Sends `words` to the slave.
    fn send_dma(self, words: &'static mut B) -> Self::Transfer;
}


/// DMA Write mode
pub trait DmaRead<Word> {
    /// Return type
    type Transfer: dma::Transfer + ?Sized;

    /// Recieve `words` from the slave.
    fn recieve_dma<Buffer, Payload>(self, words: &'static mut Buffer) -> Self::Transfer
    where
        Buffer: Unsize<[Word]>;
}

/// DMA Write mode
pub trait DmaReadWrite<Word> {
    /// Return type
    type Transfer: dma::Transfer + ?Sized;

    /// Send and recieve from the slave.
    fn transfer_dma<Buffer, Payload>(
        self,
        tx_words: &'static mut Buffer,
        rx_words: &'static mut Buffer,
    ) -> Self::Transfer
    where
        Buffer: Unsize<[Word]>;
}

/// Clock polarity
#[derive(Clone, Copy, PartialEq)]
pub enum Polarity {
    /// Clock signal low when idle
    IdleLow,
    /// Clock signal high when idle
    IdleHigh,
}

/// Clock phase
#[derive(Clone, Copy, PartialEq)]
pub enum Phase {
    /// Data in "captured" on the first clock transition
    CaptureOnFirstTransition,
    /// Data in "captured" on the second clock transition
    CaptureOnSecondTransition,
}

/// SPI mode
#[derive(Clone, Copy, PartialEq)]
pub struct Mode {
    /// Clock polarity
    pub polarity: Polarity,
    /// Clock phase
    pub phase: Phase,
}
