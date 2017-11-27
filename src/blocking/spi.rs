//! Blocking SPI API

use Spi;

/// Transfers bytes to the slave, returns the bytes received from the slave
pub fn transfer<'a, S>(spi: &mut S, bytes: &'a mut [u8]) -> Result<&'a [u8], S::Error>
where
    S: Spi<u8>,
{
    for byte in bytes.iter_mut() {
        block!(spi.send(*byte))?;
        *byte = block!(spi.read())?;
    }

    Ok(bytes)
}
