//! The prelude is a collection of all the traits in this crate
//!
//! The traits have been renamed to avoid collisions with other items when
//! performing a glob import.

pub use crate::blocking::delay::DelayMs as _embedded_hal_blocking_delay_DelayMs;
pub use crate::blocking::delay::DelayUs as _embedded_hal_blocking_delay_DelayUs;
pub use crate::blocking::digital::InputPin as _embedded_hal_blocking_digital_InputPin;
pub use crate::blocking::digital::OutputPin as _embedded_hal_blocking_digital_OutputPin;
pub use crate::blocking::digital::StatefulOutputPin as _embedded_hal_blocking_digital_StatefulOutputPin;
pub use crate::blocking::digital::ToggleableOutputPin as _embedded_hal_blocking_digital_ToggleableOutputPin;
pub use crate::blocking::i2c::{
    Read as _embedded_hal_blocking_i2c_Read,
    Transactional as _embedded_hal_blocking_i2c_Transactional,
    Write as _embedded_hal_blocking_i2c_Write, WriteIter as _embedded_hal_blocking_i2c_WriteIter,
    WriteIterRead as _embedded_hal_blocking_i2c_WriteIterRead,
    WriteRead as _embedded_hal_blocking_i2c_WriteRead,
};
pub use crate::blocking::pwm::Pwm as _embedded_hal_blocking_Pwm;
pub use crate::blocking::pwm::PwmPin as _embedded_hal_blocking_PwmPin;
pub use crate::blocking::qei::Qei as _embedded_hal_blocking_Qei;
pub use crate::blocking::rng::Read as _embedded_hal_blocking_rng_Read;
pub use crate::blocking::serial::Write as _embedded_hal_blocking_serial_Write;
pub use crate::blocking::spi::{
    Transfer as _embedded_hal_blocking_spi_Transfer, Write as _embedded_hal_blocking_spi_Write,
    WriteIter as _embedded_hal_blocking_spi_WriteIter,
};
pub use crate::blocking::watchdog::Disable as _embedded_hal_blocking_watchdog_Disable;
pub use crate::blocking::watchdog::Enable as _embedded_hal_blocking_watchdog_Enable;
pub use crate::blocking::watchdog::Watchdog as _embedded_hal_blocking_watchdog_Watchdog;
pub use crate::nonblocking::adc::Channel as _embedded_hal_nb_adc_Channel;
pub use crate::nonblocking::adc::OneShot as _embedded_hal_nb_adc_OneShot;
pub use crate::nonblocking::capture::Capture as _embedded_hal_nb_Capture;
pub use crate::nonblocking::rng::Read as _embedded_hal_nb_rng_Read;
pub use crate::nonblocking::serial::Read as _embedded_hal_nb_serial_Read;
pub use crate::nonblocking::serial::Write as _embedded_hal_nb_serial_Write;
pub use crate::nonblocking::spi::FullDuplex as _embedded_hal_nb_spi_FullDuplex;
pub use crate::nonblocking::timer::Cancel as _embedded_hal_nb_timer_Cancel;
pub use crate::nonblocking::timer::CountDown as _embedded_hal_nb_timer_CountDown;
pub use crate::nonblocking::timer::Periodic as _embedded_hal_nb_timer_Periodic;
