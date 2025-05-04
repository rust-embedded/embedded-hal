//! Types and traits for SD/MMC peripherals.

mod bus_width;
mod card_mode;
mod card_type;
mod fifo_status;
mod reset;

pub mod command;
pub mod response;
pub mod tuning;

pub use bus_width::BusWidth;
pub use card_mode::CardMode;
pub use card_type::CardType;
pub use fifo_status::FifoStatus;
pub use reset::Reset;

use command::MmcCommand;
use response::MmcResponse;
use tuning::{TuningMode, TuningWidth};

/// Common operations for DesignWare MMC controllers on JH7110 SoCs.
pub trait MmcOps {
    /// Associated error type for the SD/MMC trait.
    type Error;

    /// Gets the device [CardType].
    fn card_type(&self) -> CardType;

    /// Gets the device [CardMode].
    fn card_mode(&self) -> CardMode;

    /// Performs bus setup for the SD/MMC device.
    fn setup_bus(&mut self) -> Result<(), Self::Error>;

    /// Performs device initialization sequence.
    fn init(&mut self) -> Result<(), Self::Error>;

    /// Sets the sample phase for the MMC controller.
    fn set_sample_phase(&mut self, sample_phase: u8);

    /// Waits for the FIFO to indicate readiness for read/write operations.
    fn fifo_ready(&self, fifo_status: FifoStatus) -> Result<(), Self::Error>;

    /// Waits for the CMD line to reset (usually during power-up).
    fn wait_for_reset(&mut self, reset: Reset, timeout: u64) -> Result<(), Self::Error>;

    /// Waits for the busy signal to clear for maximum `timeout_us` microseconds.
    fn wait_while_busy(&mut self, timout_us: u64) -> Result<(), Self::Error>;

    /// Writes a SD/MMC command to the card.
    fn write_command<C: MmcCommand>(&mut self, cmd: &C) -> Result<(), Self::Error>;

    /// Reads a SD/MMC response based on the provided command argument.
    ///
    /// # Note
    ///
    /// `cmd` should match the last call to `write_command`.
    fn read_response<C: MmcCommand, R: MmcResponse>(&mut self, cmd: &C) -> Result<R, Self::Error>;

    /// Reads the raw response bytes from the MMC controller.
    ///
    /// # Note
    ///
    /// Set `exp_crc` to true if a CRC checksum is expected in the response.
    ///
    /// The generic `N` parameter is for the expected length (in bytes) of the response.
    fn response_bytes<const N: usize>(&mut self, exp_crc: bool) -> Result<[u8; N], Self::Error>;

    /// Reads data from the MMC data lines.
    fn read_data(&mut self, data: &mut [u8]) -> Result<(), Self::Error>;

    /// Writes data to the MMC data lines.
    fn write_data(&mut self, data: &[u8]) -> Result<(), Self::Error>;

    /// Requests the card to send a tuning block.
    fn send_tuning(&mut self, mode: TuningMode, width: TuningWidth) -> Result<(), Self::Error>;

    /// Executes MMC tuning.
    fn execute_tuning(&mut self, mode: TuningMode, width: TuningWidth) -> Result<(), Self::Error>;

    /// Gets the interrupts status as a 32-bit bitfield.
    fn interrupt(&self) -> u32;

    /// Sets the interrupts based on a 32-bit bitfield.
    fn set_interrupt(&mut self, int: u32);

    /// Clear all interrupts.
    fn clear_all_interrupt(&mut self);

    /// Gets the response interrupts status as a 32-bit bitfield.
    fn response_interrupt(&self) -> u32;

    /// Sets the response interrupts based on a 32-bit bitfield.
    fn set_response_interrupt(&mut self, int: u32);

    /// Clear all interrupts.
    fn clear_all_response_interrupt(&mut self);
}
