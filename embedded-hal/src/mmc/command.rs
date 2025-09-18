//! SD/MMC command types.

use super::response::ResponseType;

mod types;

pub use types::*;

/// Represents common functionality for SD/MMC command types.
pub trait MmcCommand {
    /// Gets the SD/MMC command type.
    fn command_type(&self) -> CommandType;

    /// Gets the SD/MMC response type expected for the command.
    fn response_type(&self) -> ResponseType;

    /// Gets the SD/MMC command argument.
    ///
    /// # Note
    ///
    /// Returns `0` for commands that do not expect an argument.
    fn argument(&self) -> u32;

    /// Gets the SD/MMC command argument.
    ///
    /// # Note
    ///
    /// No effect for commands that do not expect an argument.
    fn set_argument(&mut self, arg: u32);

    /// Gets the CRC-7 of the command.
    fn crc(&self) -> u8;

    /// Sets the CRC-7 of the command.
    fn set_crc(&mut self, crc: u8);
}
