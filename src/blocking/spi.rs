//! Blocking SPI API

pub use spi::{Mode, Phase, Polarity};

/// Blocking full duplex
pub trait FullDuplex<W> {
    /// An enumeration of SPI errors
    type Error;

    /// Sends `words` to the slave. Returns the `words` received from the slave
    fn transfer<'w>(&mut self, words: &'w mut [W]) -> Result<&'w [W], Self::Error>;

    /// Sends `words` to the slave, ignoring all the incoming words
    fn write(&mut self, words: &[W]) -> Result<(), Self::Error>;
}

/// Transfers words to the slave, returns the words received from the slave
pub fn transfer<'w, S, W>(spi: &mut S, words: &'w mut [W]) -> Result<&'w [W], S::Error>
where
    S: ::spi::FullDuplex<W>,
    W: Clone,
{
    for word in words.iter_mut() {
        block!(spi.send(word.clone()))?;
        *word = block!(spi.read())?;
    }

    Ok(words)
}
