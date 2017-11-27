//! Blocking SPI API

pub use spi::{FullDuplex, Mode, Phase, Polarity};

/// Transfers bytes to the slave, returns the bytes received from the slave
pub fn transfer<'a, S>(spi: &mut S, bytes: &'a mut [u8]) -> Result<&'a [u8], S::Error>
where
    S: FullDuplex<u8>,
{
    for byte in bytes.iter_mut() {
        block!(spi.send(*byte))?;
        *byte = block!(spi.read())?;
    }

    Ok(bytes)
}
