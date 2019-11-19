//! # embedded-hal
//!
//! See README.md.

pub mod digital {
    pub use embedded_hal_digital::{InputPin, OutputPin, StatefulOutputPin};
}

pub mod spi {
	pub use embedded_hal_spi::{FullDuplex, Polarity, Phase, Mode, MODE_0, MODE_1, MODE_2, MODE_3};
}
