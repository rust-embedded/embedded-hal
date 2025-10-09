//! SD/MMC response types.

mod mode;
mod types;

pub use mode::*;
pub use types::*;

/// Represents common functionality for SD/MMC response types.
pub trait MmcResponse {
    /// Gets the SD/MMC response type.
    fn response_type(&self) -> ResponseType;

    /// Gets the SD/MMC response mode.
    fn response_mode(&self) -> ResponseMode;
}
