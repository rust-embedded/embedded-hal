//! `SpiDevice` implementations.

mod exclusive;
pub use exclusive::*;

pub use crate::spi::NoDelay;

impl embedded_hal_async::delay::DelayUs for NoDelay {
    async fn delay_ms(&mut self, _ms: u32) {
        panic!("You've tried to execute a SPI transaction containing a `Operation::Delay` in a `SpiDevice` created with `new_no_delay()`. Create it with `new()` instead, passing a `DelayUs` implementation.")
    }

    async fn delay_us(&mut self, _us: u32) {
        panic!("You've tried to execute a SPI transaction containing a `Operation::Delay` in a `SpiDevice` created with `new_no_delay()`. Create it with `new()` instead, passing a `DelayUs` implementation.")
    }
}
