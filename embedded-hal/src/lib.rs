//! A Hardware Abstraction Layer (HAL) for embedded systems
//!
//! **NOTE** This HAL is still is active development. Expect the traits presented here to be
//! tweaked, split or be replaced wholesale before being stabilized, i.e. before hitting the 1.0.0
//! release. That being said there's a part of the HAL that's currently considered unproven and is
//! hidden behind an "unproven" Cargo feature. This API is even more volatile and it's exempt from
//! semver rules: it can change in a non-backward compatible fashion or even disappear in between
//! patch releases.
//!
//! # Design goals
//!
//! The HAL
//!
//! - Must *erase* device specific details. Neither register, register blocks or magic values should
//! appear in the API.
//!
//! - Must be generic *within* a device and *across* devices. The API to use a serial interface must
//! be the same regardless of whether the implementation uses the USART1 or UART4 peripheral of a
//! device or the UART0 peripheral of another device.
//!
//! - Where possible must *not* be tied to a specific asynchronous model. The API should be usable
//! in blocking mode, with the `futures` model, with an async/await model or with a callback model.
//! (cf. the [`nb`] crate)
//!
//! - Must be minimal, and thus easy to implement and zero cost, yet highly composable. People that
//! want higher level abstraction should *prefer to use this HAL* rather than *re-implement*
//! register manipulation code.
//!
//! - Serve as a foundation for building an ecosystem of platform agnostic drivers. Here driver
//! means a library crate that lets a target platform interface an external device like a digital
//! sensor or a wireless transceiver. The advantage of this system is that by writing the driver as
//! a generic library on top of `embedded-hal` driver authors can support any number of target
//! platforms (e.g. Cortex-M microcontrollers, AVR microcontrollers, embedded Linux, etc.). The
//! advantage for application developers is that by adopting `embedded-hal` they can unlock all
//! these drivers for their platform.
//!
//! # Out of scope
//!
//! - Initialization and configuration stuff like "ensure this serial interface and that SPI
//! interface are not using the same pins". The HAL will focus on *doing I/O*.
//!
//! # Reference implementation
//!
//! The [`stm32f30x-hal`] crate contains a reference implementation of this HAL.
//!
//! [`stm32f30x-hal`]: https://crates.io/crates/stm32f30x-hal/0.1.0
//!
//! # Platform agnostic drivers
//!
//! You can find platform agnostic drivers built on top of `embedded-hal` on crates.io by [searching
//! for the *embedded-hal* keyword](https://crates.io/keywords/embedded-hal).
//!
//! If you are writing a platform agnostic driver yourself you are highly encouraged to [add the
//! embedded-hal keyword](https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata)
//! to your crate before publishing it!
//!
//! # Detailed design
//!
//! ## Traits
//!
//! The HAL is specified as traits to allow generic programming. These traits make use of the
//! [`nb`][] crate (*please go read that crate documentation before continuing*) to abstract over
//! the asynchronous model and to also provide a blocking operation mode.
//!
//! [`nb`]: https://crates.io/crates/nb
//!
//! Here's what a HAL trait may look like:
//!
//! ```
//! extern crate nb;
//!
//! /// A serial interface
//! pub trait Serial {
//!     /// Error type associated to this serial interface
//!     type Error;
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
//! when paired with the macros in the `nb` crate, can operate in a blocking manner, or in a
//! non-blocking manner compatible with `futures` and with the `await!` operator.
//!
//! [`WouldBlock`]: https://docs.rs/nb/0.1.0/nb/enum.Error.html
//!
//! Some traits, like the one shown below, may expose possibly blocking APIs that can't fail. In
//! those cases `nb::Result<_, Infallible>` is used.
//!
//! ```
//! extern crate nb;
//!
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
//! Shown below is an implementation of some of the HAL traits for the [`stm32f30x`] crate. This
//! single implementation will work for *any* microcontroller in the STM32F30x family.
//!
//! [`stm32f3`]: https://crates.io/crates/stm32f3
//!
//! ```
//! // crate: stm32f3xx-hal
//! // An implementation of the `embedded-hal` traits for STM32F3xx microcontrollers
//!
//! extern crate embedded_hal as hal;
//! extern crate nb;
//!
//! // device crate
//! extern crate stm32f3;
//!
//! use stm32f3::stm32f303::USART1;
//!
//! /// A serial interface
//! // NOTE generic over the USART peripheral
//! pub struct Serial<USART> { usart: USART }
//!
//! // convenience type alias
//! pub type Serial1 = Serial<USART1>;
//!
//! /// Serial interface error
//! pub enum Error {
//!     /// Buffer overrun
//!     Overrun,
//!     // omitted: other error variants
//! }
//!
//! impl hal::serial::Read<u8> for Serial<USART1> {
//!     type Error = Error;
//!
//!     fn read(&mut self) -> nb::Result<u8, Error> {
//!         // read the status register
//!         let isr = self.usart.isr.read();
//!
//!         if isr.ore().bit_is_set() {
//!             // Error: Buffer overrun
//!             Err(nb::Error::Other(Error::Overrun))
//!         }
//!         // omitted: checks for other errors
//!         else if isr.rxne().bit_is_set() {
//!             // Data available: read the data register
//!             Ok(self.usart.rdr.read().bits() as u8)
//!         } else {
//!             // No data available yet
//!             Err(nb::Error::WouldBlock)
//!         }
//!     }
//! }
//!
//! impl hal::serial::Write<u8> for Serial<USART1> {
//!     type Error = Error;
//!
//!     fn write(&mut self, byte: u8) -> nb::Result<(), Error> {
//!         // Similar to the `read` implementation
//!         # Ok(())
//!     }
//!
//!     fn flush(&mut self) -> nb::Result<(), Error> {
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
//! Thanks to the [`nb`] crate the HAL API can be used in a blocking manner,
//! with `futures` or with the `await` operator using the [`block!`],
//! [`try_nb!`] and [`await!`] macros respectively.
//!
//! [`block!`]: https://docs.rs/nb/0.1.0/nb/macro.block.html
//! [`try_nb!`]: https://docs.rs/nb/0.1.0/nb/index.html#how-to-use-this-crate
//! [`await!`]: https://docs.rs/nb/0.1.0/nb/index.html#how-to-use-this-crate
//!
//! ### Blocking mode
//!
//! An example of sending a string over the serial interface in a blocking
//! fashion:
//!
//! ```
//! extern crate embedded_hal;
//! #[macro_use(block)]
//! extern crate nb;
//!
//! use stm32f30x_hal::Serial1;
//! use embedded_hal::serial::Write;
//!
//! # fn main() {
//! let mut serial: Serial1 = {
//!     // ..
//! #   Serial1
//! };
//!
//! for byte in b"Hello, world!" {
//!     // NOTE `block!` keeps retrying `serial.write()` until it completes, then returns
//!     // `Result<(), Error>`
//!     block!(serial.write(*byte)).unwrap();
//! }
//! # }
//!
//! # mod stm32f30x_hal {
//! #     use std::convert::Infallible;
//! #     pub struct Serial1;
//! #     impl Serial1 {
//! #         pub fn write(&mut self, _: u8) -> ::nb::Result<(), Infallible> {
//! #             Ok(())
//! #         }
//! #     }
//! # }
//! ```
//!
//! ### `futures`
//!
//! An example of running two tasks concurrently. First task: blink a LED every
//! second. Second task: loop back data over the serial interface. The target
//! must provide the `libstd` in order to be able to use `futures`, which is not
//! the case for many embedded targets.
//!
//! ```not_run
//! extern crate embedded_hal as hal;
//! extern crate futures;
//!
//! #[macro_use(try_nb)]
//! extern crate nb;
//!
//! use hal::prelude::*;
//! use futures::{
//!     future,
//!     Async,
//!     Future,
//! };
//! use futures::future::Loop;
//! use stm32f30x_hal::{Led, Serial1, Timer6};
//! use std::convert::Infallible;
//!
//! /// `futures` version of `CountDown.wait`
//! ///
//! /// This returns a future that must be polled to completion
//! fn wait<T>(mut timer: T) -> impl Future<Item = T, Error = Infallible>
//! where
//!     T: hal::timer::CountDown,
//! {
//!     let mut timer = Some(timer);
//!     future::poll_fn(move || {
//!         try_nb!(timer.as_mut().unwrap().wait());
//!
//!         Ok(Async::Ready(timer.take().unwrap()))
//!     })
//! }
//!
//! /// `futures` version of `Serial.read`
//! ///
//! /// This returns a future that must be polled to completion
//! fn read<S>(mut serial: S) -> impl Future<Item = (S, u8), Error = S::Error>
//! where
//!     S: hal::serial::Read<u8>,
//! {
//!     let mut serial = Some(serial);
//!     future::poll_fn(move || {
//!         let byte = try_nb!(serial.as_mut().unwrap().read());
//!
//!         Ok(Async::Ready((serial.take().unwrap(), byte)))
//!     })
//! }
//!
//! /// `futures` version of `Serial.write`
//! ///
//! /// This returns a future that must be polled to completion
//! fn write<S>(mut serial: S, byte: u8) -> impl Future<Item = S, Error = S::Error>
//! where
//!     S: hal::serial::Write<u8>,
//! {
//!     let mut serial = Some(serial);
//!     future::poll_fn(move || {
//!         try_nb!(serial.as_mut().unwrap().write(byte));
//!
//!         Ok(Async::Ready(serial.take().unwrap()))
//!     })
//! }
//!
//! fn main() {
//!     // HAL implementers
//!     let timer: Timer6 = {
//!         // ..
//! #       Timer6
//!     };
//!     let serial: Serial1 = {
//!         // ..
//! #       Serial1
//!     };
//!     let led: Led = {
//!         // ..
//! #       Led
//!     };
//!
//!     // Tasks
//!     let mut blinky = future::loop_fn::<_, (), _, _>(
//!         (led, timer, true),
//!         |(mut led, mut timer, state)| {
//!             wait(timer).map(move |timer| {
//!                 if state {
//!                     led.on();
//!                 } else {
//!                     led.off();
//!                 }
//!
//!                 Loop::Continue((led, timer, !state))
//!             })
//!         });
//!
//!     let mut loopback = future::loop_fn::<_, (), _, _>(serial, |mut serial| {
//!         read(serial).and_then(|(serial, byte)| {
//!             write(serial, byte)
//!         }).map(|serial| {
//!             Loop::Continue(serial)
//!         })
//!     });
//!
//!     // Event loop
//!     loop {
//!         blinky.poll().unwrap(); // NOTE(unwrap) E = Infallible
//!         loopback.poll().unwrap();
//! #       break;
//!     }
//! }
//!
//! # mod stm32f30x_hal {
//! #     use std::convert::Infallible;
//! #     pub struct Timer6;
//! #     impl ::hal::timer::CountDown for Timer6 {
//! #         type Time = ();
//! #
//! #         fn start<T>(&mut self, _: T) where T: Into<()> {}
//! #         fn wait(&mut self) -> ::nb::Result<(), Infallible> { Err(::nb::Error::WouldBlock) }
//! #     }
//! #
//! #     pub struct Serial1;
//! #     impl ::hal::serial::Read<u8> for Serial1 {
//! #         type Error = Infallible;
//! #         fn read(&mut self) -> ::nb::Result<u8, Infallible> { Err(::nb::Error::WouldBlock) }
//! #     }
//! #     impl ::hal::serial::Write<u8> for Serial1 {
//! #         type Error = Infallible;
//! #         fn flush(&mut self) -> ::nb::Result<(), Infallible> { Err(::nb::Error::WouldBlock) }
//! #         fn write(&mut self, _: u8) -> ::nb::Result<(), Infallible> { Err(::nb::Error::WouldBlock) }
//! #     }
//! #
//! #     pub struct Led;
//! #     impl Led {
//! #         pub fn off(&mut self) {}
//! #         pub fn on(&mut self) {}
//! #     }
//! # }
//! ```
//!
//! ### `await`
//!
//! Same example as above but using `await!` instead of `futures`
//! (same remark concerning the availability of `libstd` on the
//! target).
//!
//! ```not_run
//! #![feature(generator_trait)]
//! #![feature(generators)]
//!
//! extern crate embedded_hal as hal;
//!
//! #[macro_use(r#await)]
//! extern crate nb;
//!
//! use std::ops::Generator;
//! use std::pin::Pin;
//!
//! use hal::prelude::*;
//! use stm32f30x_hal::{Led, Serial1, Timer6};
//!
//! fn main() {
//!     // HAL implementers
//!     let mut timer: Timer6 = {
//!         // ..
//! #       Timer6
//!     };
//!     let mut serial: Serial1 = {
//!         // ..
//! #       Serial1
//!     };
//!     let mut led: Led = {
//!         // ..
//! #       Led
//!     };
//!
//!     // Tasks
//!     let mut blinky = (move || {
//!         let mut state = false;
//!         loop {
//!             // `await!` means "suspend / yield here" instead of "block until
//!             // completion"
//!             nb::r#await!(timer.wait()).unwrap(); // NOTE(unwrap) E = Infallible
//!
//!             state = !state;
//!
//!             if state {
//!                 led.on();
//!             } else {
//!                 led.off();
//!             }
//!         }
//!     });
//!
//!     let mut loopback = (move || {
//!         loop {
//!             let byte = nb::r#await!(serial.read()).unwrap();
//!             nb::r#await!(serial.write(byte)).unwrap();
//!         }
//!     });
//!
//!     // Event loop
//!     loop {
//!         Pin::new(&mut blinky).resume();
//!         Pin::new(&mut loopback).resume();
//!         # break;
//!     }
//! }
//!
//! # mod stm32f30x_hal {
//! #   use std::convert::Infallible;
//! #   pub struct Serial1;
//! #   impl Serial1 {
//! #       pub fn read(&mut self) -> ::nb::Result<u8, Infallible> { Err(::nb::Error::WouldBlock) }
//! #       pub fn write(&mut self, _: u8) -> ::nb::Result<(), Infallible> { Err(::nb::Error::WouldBlock) }
//! #   }
//! #   pub struct Timer6;
//! #   impl Timer6 {
//! #       pub fn wait(&mut self) -> ::nb::Result<(), Infallible> { Err(::nb::Error::WouldBlock) }
//! #   }
//! #   pub struct Led;
//! #   impl Led {
//! #       pub fn off(&mut self) {}
//! #       pub fn on(&mut self) {}
//! #   }
//! # }
//! ```
//!
//! ## Generic programming and higher level abstractions
//!
//! The core of the HAL has been kept minimal on purpose to encourage building **generic** higher
//! level abstractions on top of it. Some higher level abstractions that pick an asynchronous model
//! or that have blocking behavior and that are deemed useful to build other abstractions can be
//! found in the `blocking` module and, in the future, in the `futures` and `async` modules.
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
//! extern crate embedded_hal as hal;
//! #[macro_use(block)]
//! extern crate nb;
//!
//! use hal::prelude::*;
//!
//! fn write_all<S>(serial: &mut S, buffer: &[u8]) -> Result<(), S::Error>
//! where
//!     S: hal::serial::Write<u8>
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
//! - Blocking serial read with timeout
//!
//! ```
//! extern crate embedded_hal as hal;
//! extern crate nb;
//!
//! use hal::prelude::*;
//!
//! enum Error<E> {
//!     /// Serial interface error
//!     Serial(E),
//!     TimedOut,
//! }
//!
//! fn read_with_timeout<S, T>(
//!     serial: &mut S,
//!     timer: &mut T,
//!     timeout: T::Time,
//! ) -> Result<u8, Error<S::Error>>
//! where
//!     T: hal::timer::CountDown,
//!     S: hal::serial::Read<u8>,
//! {
//!     timer.start(timeout);
//!
//!     loop {
//!         match serial.read() {
//!             // raise error
//!             Err(nb::Error::Other(e)) => return Err(Error::Serial(e)),
//!             Err(nb::Error::WouldBlock) => {
//!                 // no data available yet, check the timer below
//!             },
//!             Ok(byte) => return Ok(byte),
//!         }
//!
//!         match timer.wait() {
//!             Err(nb::Error::Other(e)) => {
//!                 // The error type specified by `timer.wait()` is `!`, which
//!                 // means no error can actually occur. The Rust compiler
//!                 // still forces us to provide this match arm, though.
//!                 unreachable!()
//!             },
//!             // no timeout yet, try again
//!             Err(nb::Error::WouldBlock) => continue,
//!             Ok(()) => return Err(Error::TimedOut),
//!         }
//!     }
//! }
//!
//! # fn main() {}
//! ```
//!
//! - Asynchronous SPI transfer
//!
//! ```not_run
//! #![feature(conservative_impl_trait)]
//! #![feature(generators)]
//! #![feature(generator_trait)]
//!
//! extern crate embedded_hal as hal;
//! #[macro_use(r#await)]
//! extern crate nb;
//!
//! use std::ops::Generator;
//!
//! /// Transfers a byte buffer of size N
//! ///
//! /// Returns the same byte buffer but filled with the data received from the
//! /// slave device
//! fn transfer<S, B>(
//!     mut spi: S,
//!     mut buffer: [u8; 16], // NOTE this should be generic over the size of the array
//! ) -> impl Generator<Return = Result<(S, [u8; 16]), S::Error>, Yield = ()>
//! where
//!     S: hal::spi::FullDuplex<u8>,
//! {
//!     move || {
//!         let n = buffer.len();
//!         for i in 0..n {
//!             nb::r#await!(spi.send(buffer[i]))?;
//!             buffer[i] = nb::r#await!(spi.read())?;
//!         }
//!
//!         Ok((spi, buffer))
//!     }
//! }
//!
//! # fn main() {}
//! ```
//!
//! - Buffered serial interface with periodic flushing in interrupt handler
//!
//! ```
//! # use std as core;
//! extern crate embedded_hal as hal;
//! extern crate nb;
//!
//! use hal::prelude::*;
//! use ::core::convert::Infallible;
//!
//! fn flush<S>(serial: &mut S, cb: &mut CircularBuffer)
//! where
//!     S: hal::serial::Write<u8, Error = Infallible>,
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
//! # impl<'a, T> ::std::ops::Deref for RefMut<'a, T> {
//! #     type Target = T;
//! #     fn deref(&self) -> &T { self.0 }
//! # }
//! # impl<'a, T> ::std::ops::DerefMut for RefMut<'a, T> {
//! #     fn deref_mut(&mut self) -> &mut T { self.0 }
//! # }
//! # struct Serial1;
//! # impl ::hal::serial::Write<u8> for Serial1 {
//! #   type Error = Infallible;
//! #   fn write(&mut self, _: u8) -> nb::Result<(), Infallible> { Err(::nb::Error::WouldBlock) }
//! #   fn flush(&mut self) -> nb::Result<(), Infallible> { Err(::nb::Error::WouldBlock) }
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
