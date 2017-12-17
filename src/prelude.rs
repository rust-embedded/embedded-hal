//! The prelude is a collection of all the traits in this crate
//!
//! The traits have been renamed to avoid collisions with other items when
//! performing a glob import.

pub use ::Capture as _embedded_hal_Capture;
pub use ::Pwm as _embedded_hal_Pwm;
pub use ::PwmPin as _embedded_hal_PwmPin;
pub use ::Qei as _embedded_hal_Qei;
pub use ::Timer as _embedded_hal_Timer;
pub use ::digital::OutputPin as _embedded_hal_digital_OutputPin;
pub use ::serial::Read as _embedded_hal_serial_Read;
pub use ::serial::Write as _embedded_hal_serial_Write;
pub use ::spi::FullDuplex as _embedded_hal_spi_FullDuplex;
