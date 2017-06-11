//! The prelude is a collection of all the traits in this crate
//!
//! The traits have been renamed to avoid collisions with other items when
//! performing a glob import.

pub use ::Capture as _embedded_hal_Capture;
pub use ::Pwm as _embedded_hal_Pwm;
pub use ::Qei as _embedded_hal_Qei;
pub use ::Serial as _embedded_hal_Serial;
pub use ::Spi as _embedded_hal_Spi;
pub use ::Timer as _embedded_hal_Timer;
