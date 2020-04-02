//! The prelude is a collection of all the traits in this crate
//!
//! The traits have been renamed to avoid collisions with other items when
//! performing a glob import.

pub use crate::adc::OneShot as _embedded_hal_adc_OneShot;
pub use crate::blocking::delay::DelayMs as _embedded_hal_blocking_delay_DelayMs;
pub use crate::blocking::delay::DelayUs as _embedded_hal_blocking_delay_DelayUs;
pub use crate::blocking::i2c::{
    Read as _embedded_hal_blocking_i2c_Read, Write as _embedded_hal_blocking_i2c_Write,
    WriteRead as _embedded_hal_blocking_i2c_WriteRead,
};
pub use crate::blocking::rng::Read as _embedded_hal_blocking_rng_Read;
pub use crate::blocking::serial::Write as _embedded_hal_blocking_serial_Write;
pub use crate::blocking::spi::{
    Transfer as _embedded_hal_blocking_spi_Transfer, Write as _embedded_hal_blocking_spi_Write,
};
pub use crate::capture::Capture as _embedded_hal_Capture;
pub use crate::digital::InputPin as _embedded_hal_digital_InputPin;
pub use crate::digital::OutputPin as _embedded_hal_digital_OutputPin;
pub use crate::digital::ToggleableOutputPin as _embedded_hal_digital_ToggleableOutputPin;
pub use crate::pwm::Pwm as _embedded_hal_Pwm;
pub use crate::pwm::PwmPin as _embedded_hal_PwmPin;
pub use crate::qei::Qei as _embedded_hal_Qei;
pub use crate::rng::Read as _embedded_hal_rng_Read;
pub use crate::serial::Read as _embedded_hal_serial_Read;
pub use crate::serial::Write as _embedded_hal_serial_Write;
pub use crate::spi::FullDuplex as _embedded_hal_spi_FullDuplex;
pub use crate::timer::CountDown as _embedded_hal_timer_CountDown;
pub use crate::watchdog::Watchdog as _embedded_hal_watchdog_Watchdog;
pub use crate::watchdog::WatchdogDisable as _embedded_hal_watchdog_WatchdogDisable;
pub use crate::watchdog::WatchdogEnable as _embedded_hal_watchdog_WatchdogEnable;
