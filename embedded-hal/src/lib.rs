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

// needed to prevent defmt macros from breaking, since they emit code that does `defmt::blahblah`.
#[cfg(feature = "defmt-03")]
use defmt_03 as defmt;
