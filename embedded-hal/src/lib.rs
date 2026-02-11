#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![no_std]

pub mod delay;
pub mod digital;
pub mod i2c;
pub mod pwm;
pub mod spi;

mod private {
    use crate::i2c::{SevenBitAddress, TenBitAddress};
    pub trait Sealed {}

    impl Sealed for SevenBitAddress {}
    impl Sealed for TenBitAddress {}
}
