/// Represents the card mode of the peripheral.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CardMode {
    /// Represents a device in SD mode.
    Sd,
    /// Represents a device in SDIO mode.
    Sdio,
    /// Represents a device in SPI mode.
    Spi,
}

impl CardMode {
    /// Creates a new [CardMode].
    pub const fn new() -> Self {
        Self::Sd
    }

    /// Convenience function to get if the [CardMode] is SD.
    pub const fn is_sd(&self) -> bool {
        matches!(self, Self::Sd)
    }

    /// Convenience function to get if the [CardMode] is SDIO.
    pub const fn is_sdio(&self) -> bool {
        matches!(self, Self::Sdio)
    }

    /// Convenience function to get if the [CardMode] is SPI.
    pub const fn is_spi(&self) -> bool {
        matches!(self, Self::Spi)
    }
}

impl Default for CardMode {
    fn default() -> Self {
        Self::new()
    }
}
