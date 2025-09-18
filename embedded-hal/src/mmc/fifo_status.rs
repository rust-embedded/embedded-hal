//! FIFO status types.

/// Represents the FIFO status of the host controller.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FifoStatus {
    /// MMC FIFO is empty.
    Empty = 0,
    /// MMC FIFO is full.
    Full = 1,
}

impl FifoStatus {
    /// Creates a new [FifoStatus].
    pub const fn new() -> Self {
        Self::Empty
    }
}

impl Default for FifoStatus {
    fn default() -> Self {
        Self::new()
    }
}
