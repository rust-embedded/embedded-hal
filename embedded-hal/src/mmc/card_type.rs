/// Represents the card type of the peripheral.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CardType {
    /// Represents a SD device.
    Sd,
    /// Represents a MMC device.
    Mmc,
}

impl CardType {
    /// Creates a new [CardType].
    pub const fn new() -> Self {
        Self::Sd
    }

    /// Convenience function to get if the [CardType] is a SD.
    pub const fn is_sd(&self) -> bool {
        matches!(self, Self::Sd)
    }

    /// Convenience function to get if the [CardType] is a MMC.
    pub const fn is_mmc(&self) -> bool {
        matches!(self, Self::Mmc)
    }
}

impl Default for CardType {
    fn default() -> Self {
        Self::new()
    }
}
