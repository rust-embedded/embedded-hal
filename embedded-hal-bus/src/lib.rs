//! Bus/Device connection mechanisms for [`embedded-hal`], a Hardware Abstraction Layer (HAL) for embedded systems.
//!
//! It is possible to connect several peripherals to a bus like SPI or I2C.
//! To support this, `embedded-hal` provides the `SpiBus` and `SpiDevice` traits in the case of SPI, for example.
//!
//! `embedded-hal` trait implementations for microcontrollers should implement the `...Bus` traits.
//! However, device drivers should use the `...Device` traits, _not the `...Bus` traits_ if at all possible
//! in order to allow for sharing of the bus they are connected to.
//!
//! This crate provides mechanisms to connect a `...Bus` and a `...Device`.
//!
//! For further details on these traits, please consult the [`embedded-hal` documentation](https://docs.rs/embedded-hal).

#![warn(missing_docs)]
#![no_std]

pub mod i2c;
pub mod spi;
