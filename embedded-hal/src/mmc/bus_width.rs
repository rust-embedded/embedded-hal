/// Represents the variants of the `bus width` field of the [Argument](super::Argument).
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BusWidth {
    /// Represents the selection of a 1-bit bus width.
    Bits1 = 0b00,
    /// Represents the selection of a 4-bit bus width.
    Bits4 = 0b10,
    /// Represents the selection of a 8-bit bus width.
    Bits8 = 0b11,
}

impl BusWidth {
    /// Creates a new [BusWidth].
    pub const fn new() -> Self {
        Self::Bits1
    }
}

impl Default for BusWidth {
    fn default() -> Self {
        Self::new()
    }
}
