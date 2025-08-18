//! SD/MMC reset types.

/// Represents the resets to enable on the MMC host controller.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Reset {
    /// Reset the MMC peripheral.
    Mmc = 1,
    /// Reset the FIFO peripheral.
    Fifo = 2,
    /// Reset the DMA peripheral.
    Dma = 4,
    /// Reset the MMC + FIFO peripherals.
    MmcFifo = 3,
    /// Reset the MMC + DMA peripherals.
    MmcDma = 5,
    /// Reset the FIFO + DMA peripherals.
    FifoDma = 6,
    /// Reset all peripherals.
    All = 7,
}

impl Reset {
    /// Creates a new [Reset].
    pub const fn new() -> Self {
        Self::Mmc
    }
}

impl Default for Reset {
    fn default() -> Self {
        Self::new()
    }
}
