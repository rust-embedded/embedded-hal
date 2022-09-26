//! Non-blocking Hardware Abstraction Layer (HAL) traits for embedded systems, using the `nb` crate.
//!
//! The `embedded-hal-nb` traits make use of the
//! [`nb`][] crate (*please go read that crate documentation before continuing*) to abstract over
//! the asynchronous model and to also provide a blocking operation mode.
//!
//! [`nb`]: https://crates.io/crates/nb
//!
//! Here's how a HAL trait may look like:
//!
//! ```
//! use embedded_hal_nb;
//!
//! /// A serial interface
//! pub trait Serial {
//!     /// Error type associated to this serial interface
//!     type Error: core::fmt::Debug;
//!
//!     /// Reads a single byte
//!     fn read(&mut self) -> nb::Result<u8, Self::Error>;
//!
//!     /// Writes a single byte
//!     fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error>;
//! }
//! ```
//!
//! The `nb::Result` enum is used to add a [`WouldBlock`] variant to the errors
//! of the serial interface. As explained in the documentation of the `nb` crate this single API,
//! when paired with the macros in the `nb` crate, can operate in a blocking manner, or be adapted
//! to other asynchronous execution schemes.
//!
//! [`WouldBlock`]: https://docs.rs/nb/1.0.0/nb/enum.Error.html
//!
//! Some traits, like the one shown below, may expose possibly blocking APIs that can't fail. In
//! those cases `nb::Result<_, Infallible>` is used.
//!
//! ```
//! # use std as core;
//! use ::core::convert::Infallible;
//!
//! /// A count down timer
//! pub trait CountDown {
//!     // ..
//!
//!     /// "waits" until the count down is over
//!     fn wait(&mut self) -> nb::Result<(), Infallible>;
//! }
//!
//! # fn main() {}
//! ```
//!
//! ## Suggested implementation
//!
//! The HAL traits should be implemented for device crates generated via [`svd2rust`] to maximize
//! code reuse.
//!
//! [`svd2rust`]: https://crates.io/crates/svd2rust
//!
//! Shown below is an implementation of some of the HAL traits for the [`stm32f1xx-hal`] crate. This
//! single implementation will work for *any* microcontroller in the STM32F1xx family.
//!
//! [`stm32f1`]: https://crates.io/crates/stm32f1
//!
//! ```no_run
//! // crate: stm32f1xx-hal
//! // An implementation of the `embedded-hal` traits for STM32F1xx microcontrollers
//!
//! use embedded_hal_nb::serial;
//! use nb;
//!
//! // device crate
//! use stm32f1::stm32f103::USART1;
//!
//! /// A serial interface
//! // NOTE generic over the USART peripheral
//! pub struct Serial<USART> { usart: USART }
//!
//! // convenience type alias
//! pub type Serial1 = Serial<USART1>;
//!
//! impl serial::ErrorType for Serial<USART1> {
//!     type Error = serial::ErrorKind;
//! }
//!
//! impl embedded_hal_nb::serial::Read<u8> for Serial<USART1> {
//!     fn read(&mut self) -> nb::Result<u8, Self::Error> {
//!         // read the status register
//!         let isr = self.usart.sr.read();
//!
//!         if isr.ore().bit_is_set() {
//!             // Error: Buffer overrun
//!             Err(nb::Error::Other(Self::Error::Overrun))
//!         }
//!         // omitted: checks for other errors
//!         else if isr.rxne().bit_is_set() {
//!             // Data available: read the data register
//!             Ok(self.usart.dr.read().bits() as u8)
//!         } else {
//!             // No data available yet
//!             Err(nb::Error::WouldBlock)
//!         }
//!     }
//! }
//!
//! impl embedded_hal_nb::serial::Write<u8> for Serial<USART1> {
//!     fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
//!         // Similar to the `read` implementation
//!         # Ok(())
//!     }
//!
//!     fn flush(&mut self) -> nb::Result<(), Self::Error> {
//!         // Similar to the `read` implementation
//!         # Ok(())
//!     }
//! }
//!
//! # fn main() {}
//! ```
//!
//! ## Intended usage
//!
//! Thanks to the [`nb`] crate the HAL API can be used in a blocking manner
//! with the [`block!`] macro or with `futures`.
//!
//! [`block!`]: https://docs.rs/nb/1.0.0/nb/macro.block.html
//!
//! ### Blocking mode
//!
//! An example of writing a string over the serial interface in a blocking
//! fashion:
//!
//! ```
//! use stm32f1xx_hal::Serial1;
//! use embedded_hal_nb::serial::Write;
//! use nb::block;
//!
//! # fn main() {
//! let mut serial: Serial1 = {
//!     // ..
//! #   Serial1
//! };
//!
//! for byte in b"Hello, world!" {
//!     // NOTE `block!` blocks until `serial.write()` completes and returns
//!     // `Result<(), Error>`
//!     block!(serial.write(*byte)).unwrap();
//! }
//! # }
//!
//! # mod stm32f1xx_hal {
//! #     use embedded_hal_nb;
//! #     use core::convert::Infallible;
//! #     pub struct Serial1;
//! #     impl Serial1 {
//! #         pub fn write(&mut self, _: u8) -> nb::Result<(), Infallible> {
//! #             Ok(())
//! #         }
//! #     }
//! # }
//! ```
//!
//! ## Generic programming and higher level abstractions
//!
//! The core of the HAL has been kept minimal on purpose to encourage building **generic** higher
//! level abstractions on top of it. Some higher level abstractions that pick an asynchronous model
//! or that have blocking behavior and that are deemed useful to build other abstractions can be
//! found in the `blocking` module.
//!
//! Some examples:
//!
//! **NOTE** All the functions shown below could have been written as trait
//! methods with default implementation to allow specialization, but they have
//! been written as functions to keep things simple.
//!
//! - Write a whole buffer to a serial device in blocking a fashion.
//!
//! ```
//! use embedded_hal_nb::serial::Write;
//! use nb::block;
//!
//! fn write_all<S>(serial: &mut S, buffer: &[u8]) -> Result<(), S::Error>
//! where
//!     S: Write<u8>
//! {
//!     for &byte in buffer {
//!         block!(serial.write(byte))?;
//!     }
//!
//!     Ok(())
//! }
//!
//! # fn main() {}
//! ```
//!
//! - Buffered serial interface with periodic flushing in interrupt handler
//!
//! ```
//! # use std as core;
//! use embedded_hal_nb::serial::{ErrorKind, Write};
//! use nb::block;
//!
//! fn flush<S>(serial: &mut S, cb: &mut CircularBuffer)
//! where
//!     S: Write<u8, Error = ErrorKind>,
//! {
//!     loop {
//!         if let Some(byte) = cb.peek() {
//!             match serial.write(*byte) {
//!                 Err(nb::Error::Other(_)) => unreachable!(),
//!                 Err(nb::Error::WouldBlock) => return,
//!                 Ok(()) => {}, // keep flushing data
//!             }
//!         }
//!
//!         cb.pop();
//!     }
//! }
//!
//! // The stuff below could be in some other crate
//!
//! /// Global singleton
//! pub struct BufferedSerial1;
//!
//! // NOTE private
//! static BUFFER1: Mutex<CircularBuffer> = {
//!     // ..
//! #   Mutex(CircularBuffer)
//! };
//! static SERIAL1: Mutex<Serial1> = {
//!     // ..
//! #   Mutex(Serial1)
//! };
//!
//! impl BufferedSerial1 {
//!     pub fn write(&self, byte: u8) {
//!         self.write_all(&[byte])
//!     }
//!
//!     pub fn write_all(&self, bytes: &[u8]) {
//!         let mut buffer = BUFFER1.lock();
//!         for byte in bytes {
//!             buffer.push(*byte).expect("buffer overrun");
//!         }
//!         // omitted: pend / enable interrupt_handler
//!     }
//! }
//!
//! fn interrupt_handler() {
//!     let mut serial = SERIAL1.lock();
//!     let mut buffer = BUFFER1.lock();
//!
//!     flush(&mut *serial, &mut buffer);
//! }
//!
//! # struct Mutex<T>(T);
//! # impl<T> Mutex<T> {
//! #     fn lock(&self) -> RefMut<T> { unimplemented!() }
//! # }
//! # struct RefMut<'a, T>(&'a mut T) where T: 'a;
//! # impl<'a, T> ::core::ops::Deref for RefMut<'a, T> {
//! #     type Target = T;
//! #     fn deref(&self) -> &T { self.0 }
//! # }
//! # impl<'a, T> ::core::ops::DerefMut for RefMut<'a, T> {
//! #     fn deref_mut(&mut self) -> &mut T { self.0 }
//! # }
//! # struct Serial1;
//! # impl embedded_hal_nb::serial::ErrorType for Serial1 {
//! #   type Error = ErrorKind;
//! # }
//! # impl embedded_hal_nb::serial::Write<u8> for Serial1 {
//! #   fn write(&mut self, _: u8) -> nb::Result<(), Self::Error> { Err(nb::Error::WouldBlock) }
//! #   fn flush(&mut self) -> nb::Result<(), Self::Error> { Err(nb::Error::WouldBlock) }
//! # }
//! # struct CircularBuffer;
//! # impl CircularBuffer {
//! #   pub fn peek(&mut self) -> Option<&u8> { None }
//! #   pub fn pop(&mut self) -> Option<u8> { None }
//! #   pub fn push(&mut self, _: u8) -> Result<(), ()> { Ok(()) }
//! # }
//!
//! # fn main() {}
//! ```

#![warn(missing_docs)]
#![no_std]

pub use nb;

pub mod serial;
pub mod spi;
