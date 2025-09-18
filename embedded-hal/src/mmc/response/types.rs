//! SD/MMC response types.

use super::ResponseMode;

/// Represents the response types used in the SD/MMC protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseType {
    /// No response type.
    None,
    /// The standard response sent for most command types.
    R1,
    /// The same as the `R1` response, but drives a `BUSY` signal on the `DAT` line(s).
    R1b,
    /// 136-bit response that includes the contents of the card `CID` or `CSD` register.
    R2,
    /// Returns the contents of the card `OCR` register.
    R3,
    /// SDIO response to the `IO_SEND_OP_COND` command.
    ///
    /// Returns the card `IO_OCR` register contents, and other operating conditions.
    R4,
    /// SDIO response to the `IO_RW_DIRECT` commands.
    R5,
    /// Response containing the published RCA information.
    R6,
    /// Response containing the card interface condition.
    R7,
}

impl ResponseType {
    /// Represents the byte length for an 8-bit response.
    pub const LEN_8BIT: usize = 1;
    /// Represents the byte length for an 16-bit response.
    pub const LEN_16BIT: usize = 2;
    /// Represents the byte length for an 40-bit response.
    pub const LEN_40BIT: usize = 5;
    /// Represents the byte length for an 48-bit response.
    pub const LEN_48BIT: usize = 6;
    /// Represents the byte length for an 136-bit response.
    pub const LEN_136BIT: usize = 17;
    /// Represents the byte length for no response.
    pub const LEN_NONE: usize = 0;

    /// Creates a new [ResponseType].
    pub const fn new() -> Self {
        Self::R1
    }

    /// Gets the byte length of the [ResponseType] based on the operation mode.
    pub const fn len(&self, mode: ResponseMode) -> usize {
        match (mode, self) {
            (
                ResponseMode::Sd,
                Self::R1 | Self::R1b | Self::R3 | Self::R4 | Self::R6 | Self::R7,
            ) => Self::LEN_48BIT,
            (ResponseMode::Sd | ResponseMode::Sdio, Self::R2) => Self::LEN_136BIT,
            (ResponseMode::Sdio, Self::R1 | Self::R1b | Self::R4 | Self::R5 | Self::R6) => {
                Self::LEN_48BIT
            }
            (ResponseMode::Spi, Self::R1 | Self::R1b) => Self::LEN_8BIT,
            (ResponseMode::Spi, Self::R2 | Self::R5) => Self::LEN_16BIT,
            (ResponseMode::Spi, Self::R3 | Self::R4 | Self::R7) => Self::LEN_40BIT,
            _ => Self::LEN_NONE,
        }
    }

    /// Gets whether the response type includes a `CRC-7` checksum field.
    pub const fn has_crc(&self, mode: ResponseMode) -> bool {
        matches!(
            (mode, self),
            (
                ResponseMode::Sd,
                Self::R1 | Self::R1b | Self::R2 | Self::R4 | Self::R5 | Self::R6 | Self::R7
            )
        ) || matches!((mode, self), (ResponseMode::Sdio, Self::R5 | Self::R6))
    }
}

impl Default for ResponseType {
    fn default() -> Self {
        Self::new()
    }
}
