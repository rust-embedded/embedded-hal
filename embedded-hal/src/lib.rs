//! # embedded-hal
//!
//! See README.md.

#![deny(missing_docs)]
#![no_std]

/// A collection of items we expect most people to want to have in scope.
///
/// We re-name items imported here to avoid collisions with any local items
/// users may have.
pub mod prelude {
    pub use crate::blocking::delay::DelayMs as _embedded_hal_blocking_delay_DelayMs;
    pub use crate::blocking::delay::DelayUs as _embedded_hal_blocking_delay_DelayUs;
    pub use crate::blocking::i2c::{
        Read as _embedded_hal_blocking_i2c_Read, Write as _embedded_hal_blocking_i2c_Write,
        WriteRead as _embedded_hal_blocking_i2c_WriteRead,
    };
    pub use crate::blocking::serial::Write as _embedded_hal_blocking_serial_Write;
    pub use crate::blocking::spi::{
        Transfer as _embedded_hal_blocking_spi_Transfer, Write as _embedded_hal_blocking_spi_Write,
    };
    pub use crate::digital::InputPin as _embedded_hal_digital_InputPin;
    pub use crate::digital::OutputPin as _embedded_hal_digital_OutputPin;
    pub use crate::serial::Read as _embedded_hal_serial_Read;
    pub use crate::serial::Write as _embedded_hal_serial_Write;
    pub use crate::spi::FullDuplex as _embedded_hal_spi_FullDuplex;
    pub use crate::timer::CountDown as _embedded_hal_timer_CountDown;
}

/// A sub-set of traits which can block until the specific work is done. Blocking is usually
/// performed by busy-waiting, so this may not be ideal in a performance-sensitive system.
pub mod blocking {
    /// Busy-waiting for fixed time periods
    pub mod delay {
        pub use embedded_hal_delay::blocking::{DelayMs, DelayUs};
    }

    /// Busy-waiting I2C transactions
    pub mod i2c {
        pub use embedded_hal_i2c::blocking::{Read, Write, WriteRead};
    }

    /// Busy-waiting serial transactions
    pub mod serial {
        pub use embedded_hal_serial::blocking::Write;
    }

    /// Busy-waiting SPI transactions
    pub mod spi {
        pub use embedded_hal_spi::blocking::{Transfer, Write};
    }
}

/// Traits for digital I/O pins.
pub mod digital {
    pub use embedded_hal_digital::{InputPin, OutputPin, StatefulOutputPin};
}

/// Traits for hardware which implements the Serial Peripheral Interface as a
/// master.
pub mod spi {
    pub use embedded_hal_spi::{FullDuplex, Mode, Phase, Polarity, MODE_0, MODE_1, MODE_2, MODE_3};
}

/// Traits for devices which can transmit and/or receive data one 'word' at a
/// time. Typically these are Universal Asynchronous Receiver/Transmitters
/// (UARTs).
pub mod serial {
    pub use embedded_hal_serial::{Read, Write};
}

/// Traits for timers - objects that count time and raise interrupts at
/// specific times.
pub mod timer {
    pub use embedded_hal_timer::CountDown;
}
