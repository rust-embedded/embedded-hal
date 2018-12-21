//! The prelude is a collection of all the traits in this crate
//!
//! The traits have been renamed to avoid collisions with other items when
//! performing a glob import.

#[cfg(feature = "unproven")]
pub use adc::OneShot as _embedded_hal_adc_OneShot;
pub use blocking::delay::DelayMs as _embedded_hal_blocking_delay_DelayMs;
pub use blocking::delay::DelayUs as _embedded_hal_blocking_delay_DelayUs;
pub use blocking::i2c::{
    Read as _embedded_hal_blocking_i2c_Read, Write as _embedded_hal_blocking_i2c_Write,
    WriteRead as _embedded_hal_blocking_i2c_WriteRead,
};
#[cfg(feature = "unproven")]
pub use blocking::rng::Read as _embedded_hal_blocking_rng_Read;
pub use blocking::serial::Write as _embedded_hal_blocking_serial_Write;
pub use blocking::spi::{
    Transfer as _embedded_hal_blocking_spi_Transfer, Write as _embedded_hal_blocking_spi_Write,
};
#[allow(deprecated)]
#[cfg(feature = "unproven")]
pub use digital::InputPin as _embedded_hal_digital_InputPin;
#[allow(deprecated)]
pub use digital::OutputPin as _embedded_hal_digital_OutputPin;
#[cfg(feature = "unproven")]
#[allow(deprecated)]
pub use digital::ToggleableOutputPin as _embedded_hal_digital_ToggleableOutputPin;
pub use serial::Read as _embedded_hal_serial_Read;
pub use serial::Write as _embedded_hal_serial_Write;
pub use spi::FullDuplex as _embedded_hal_spi_FullDuplex;
pub use timer::CountDown as _embedded_hal_timer_CountDown;
#[cfg(feature = "unproven")]
pub use watchdog::Watchdog as _embedded_hal_watchdog_Watchdog;
#[cfg(feature = "unproven")]
pub use watchdog::WatchdogDisable as _embedded_hal_watchdog_WatchdogDisable;
#[cfg(feature = "unproven")]
pub use watchdog::WatchdogEnable as _embedded_hal_watchdog_WatchdogEnable;
#[cfg(feature = "unproven")]
pub use Capture as _embedded_hal_Capture;
#[cfg(feature = "unproven")]
pub use Pwm as _embedded_hal_Pwm;
pub use PwmPin as _embedded_hal_PwmPin;
#[cfg(feature = "unproven")]
pub use Qei as _embedded_hal_Qei;
