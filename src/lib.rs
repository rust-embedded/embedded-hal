//! A minimal Hardware Abstraction Layer (HAL) for embedded systems
//!
//! **NOTE** The HAL is in design phase. Expect the API to change in non
//! backward compatible ways without previous notice.
//!
//! # Design goals
//!
//! The HAL
//!
//! - Must *erase* device specific details. Neither register, register blocks or
//!   magic values should appear in the API.
//!
//! - Must be generic *within* a device and *across* devices. The API to use a
//!   serial interface must be the same regardless of whether the
//!   implementation uses the USART1 or UART4 peripheral of a device or the
//!   UART0 peripheral of another device.
//!
//! - Must *not* be tied to a specific asynchronous model. The API should be
//!   usable in blocking mode, with the `futures` model, with an async/await
//!   model or with a callback model.
//!
//! - Must be minimal, and thus easy to implement and zero cost, yet highly
//!   composable. People that want higher level abstraction should *prefer
//!   to use this HAL* rather than *re-implement* register manipulation code.
//!
//! # Out of scope
//!
//! - Initialization and configuration stuff like "ensure this serial interface
//!   and that SPI interface are not using the same pins". The HAL will focus on
//!   *doing I/O.*
//!
//! # Reference implementation
//!
//! The [`blue-pill`] crate contains a reference implementation of this HAL.
//!
//! [`blue-pill`]: https://github.com/japaric/blue-pill
//!
//! # Detailed design
//!
//! ## Traits
//!
//! The HAL is specified using traits to allow generic programming. These traits
//! traits will make use of the [`nb`][] [crate][] (*please go read that crate
//! documentation before continuing*) to abstract over the asynchronous model
//! and to also provide a blocking operation mode.
//!
//! [`nb`]: https://github.com/japaric/nb
//! [crate]: https://japaric.github.io/nb/nb/
//!
//! Here's how a HAL trait may look like:
//!
//! ``` rust
//! extern crate nb;
//!
//! /// A serial interface
//! pub trait Serial {
//!     /// Error type associated to this serial interface
//!     type Error;
//!
//!     /// Reads a single byte
//!     fn read(&self) -> nb::Result<u8, Self::Error>;
//!
//!     /// Writes a single byte
//!     fn write(&self, byte: u8) -> nb::Result<(), Self::Error>;
//! }
//! ```
//!
//! The `nb::Result` enum is used to add a [`WouldBlock`] variant to the errors
//! of the serial interface. As we'll see below this extra error variant lets
//! this single API operate in a blocking manner, or in a non-blocking manner
//! compatible with `futures` and with the `await` operator.
//!
//! [`WouldBlock`]: https://japaric.github.io/nb/nb/enum.Error.html
//!
//! Some traits, like the one shown below, may expose possibly blocking APIs
//! that can't fail. In those cases `nb::Result<_, !>` should be used.
//!
//! ``` ignore
//! /// A timer used for timeouts
//! pub trait Timer {
//!     /// A time unit that can be convert to a human time unit
//!     type Time;
//!
//!     /// Gets the current timer timeout
//!     fn get_timeout(&self) -> Self::Time;
//!
//!     /// Pauses the timer
//!     fn pause(&mut self);
//!
//!     /// Restarts the timer count
//!     fn restart(&mut self);
//!
//!     /// Resumes the timer count
//!     fn resume(&mut self);
//!
//!     /// Sets a new timeout
//!     fn set_timeout<T>(&mut self, ticks: T) where T: Into<Self::Time>;
//!
//!     /// "waits" until the timer times out
//!     fn wait(&self) -> nb::Result<(), !>;
//! }
//! ```
//!
//! ## Implementation
//!
//! The HAL traits should be implemented for device crates generated via
//! [`svd2rust`] to maximize code reuse.
//!
//! [`svd2rust`]: https://crates.io/crates/svd2rust
//!
//! Shown below is an implementation of the HAL traits for the [`stm32f103xx`]
//! crate. This single implementation will work for *any* microcontroller in the
//! STM32F103 family.
//!
//! [`stm32f103xx`]: https://crates.io/crates/stm32f103xx
//!
//! ``` ignore
//! //! An implementation of the `embedded-hal` for STM32F103xx microcontrollers
//!
//! extern crate core;
//! extern crate embedded_hal as hal;
//! extern crate nb;
//!
//! // device crate
//! extern crate stm32f103xx;
//!
//! use core::ops::Deref;
//!
//! /// A serial interface
//! // NOTE generic over the USART peripheral. This works with USART1, USART2
//! // and USART3
//! pub struct Serial<'a, U>(pub &'a U)
//! where
//!     U: Deref<Target=stm32f103xx::usart1::RegisterBlock> + 'static;
//!
//! /// Serial interface error
//! pub enum Error {
//!     /// Buffer overrun
//!     Overrun,
//!     // add more error variants here
//! }
//!
//! impl<'a, U> hal::serial::Read<u8> for Serial<'a, U>
//!     where
//!         U: Deref<Target=stm32f103xx::usart1::RegisterBlock> + 'static
//! {
//!     type Error = Error;
//!
//!     fn read(&mut self) -> nb::Result<u8, Error> {
//!         // read the status register
//!         let sr = self.0.sr.read();
//!
//!         if sr.ore().bit_is_set() {
//!             // Error: Buffer overrun
//!             Err(nb::Error::Other(Error::Overrun))
//!         }
//!         // Add additional `else if` statements to check for other errors
//!         else if sr.rxne().bit_is_set() {
//!             // Data available: read the data register
//!             Ok(self.0.dr.read().bits() as u8)
//!         }
//!         else {
//!             // No data available yet
//!             Err(nb::Error::WouldBlock)
//!         }
//!     }
//! }
//!
//! impl<'a, U> hal::serial::Write<u8> for Serial<'a, U>
//!     where
//!         U: Deref<Target=stm32f103xx::usart1::RegisterBlock> + 'static
//! {
//!     type Error = Error;
//!
//!     fn write(&mut self, byte: u8) -> nb::Result<(), Error> {
//!         // Very similar to the above implementation
//!         Ok(())
//!     }
//! }
//! ```
//!
//! Furthermore the above implementation of `hal::Serial` is generic over the
//! USART peripheral instance and will work with peripherals USART1, USART2,
//! USART3, etc.
//!
//! Note that the above implementation uses a newtype over a *reference* to the
//! USART register block. This pushes the concern of synchronization to the user
//! of the `Serial` abstraction. However it's also possible to *erase* that
//! reference by handling the synchronization within the `hal::Serial`
//! implementation:
//!
//! ``` ignore
//! extern crate embedded_hal as hal;
//! extern crate nb;
//!
//! /// A synchronized serial interface
//! // NOTE This is a global singleton
//! pub struct Serial1;
//!
//! // NOTE private
//! static USART1: Mutex<_> = Mutex::new(..);
//!
//! impl hal::serial::Read<u8> for Serial1 {
//!     type Error = !;
//!
//!     fn read(&mut self) -> Result<u8, nb::Error<Self::Error>> {
//!         hal::serial::Read::read(&Serial(&*USART1.lock()))
//!     }
//! }
//! ```
//!
//! ## Intended usage
//!
//! Thanks to the [`nb`] crate the HAL API can be used in a blocking manner,
//! with `futures` or with the `await` operator using the [`block!`],
//! [`try_nb!`] and [`await!`] macros respectively.
//!
//! [`block!`]: https://japaric.github.io/nb/nb/macro.block.html
//! [`try_nb!`]: https://japaric.github.io/nb/nb/macro.try_nb.html
//! [`await!`]: https://japaric.github.io/nb/nb/macro.await.html
//!
//! ### Blocking mode
//!
//! An example of sending a string over the serial interface in a blocking
//! fashion:
//!
//! ``` ignore
//! extern crate embedded_hal as hal;
//!
//! #[macro_use]
//! extern crate nb;
//!
//! extern crate stm32f103xx_hal_impl;
//!
//! use stm32f103xx_hal_impl::Serial;
//!
//! let serial = Serial(usart1);
//!
//! for byte in b"Hello, world!" {
//!     // NOTE `block!` blocks until `serial.write()` completes and returns
//!     // `Result<(), Error>`
//!     block!(serial.write()).unwrap();
//! }
//! ```
//!
//! ### `futures`
//!
//! An example of running two tasks concurrently. First task: blink an LED every
//! second. Second task: loop back data over the serial interface.
//!
//! ``` ignore
//! extern crate embedded_hal as hal;
//!
//! extern crate futures;
//! extern crate stm32f103xx_hal_impl;
//!
//! #[macro_use]
//! extern crate nb;
//!
//! use hal::prelude::*;
//! use futures::{
//!     future,
//!     Async,
//!     Future,
//! };
//! use futures::future::Loop;
//! use stm32f103xx_hal_impl::{Serial, Timer};
//!
//! /// `futures` version of `Timer.wait`
//! ///
//! /// This returns a future that must be polled to completion
//! fn wait<T>(timer: T) -> impl Future<Item = T, Error = !>
//! where
//!     T: hal::Timer,
//! {
//!     future::loop_fn(timer, |timer| {
//!         match timer.wait() {
//!             Ok(())                     => Ok(Loop::Break(timer)),
//!             Err(nb::Error::WouldBlock) => Ok(Loop::Continue(timer)),
//!         }
//!     })
//! }
//!
//! /// `futures` version of `Serial.read`
//! ///
//! /// This returns a future that must be polled to completion
//! fn read<S>(serial: S) -> impl Future<Item = (S, u8), Error = S::Error>
//! where
//!     S: hal::serial::Read<u8>,
//! {
//!     future::loop_fn(serial, |mut serial| {
//!         match serial.read() {
//!             Ok(byte)                     => Ok(Loop::Break((serial, byte))),
//!             Err(nb::Error::WouldBlock)   => Ok(Loop::Continue(serial)),
//!             Err(nb::Error::Other(error)) => Err(error),
//!         }
//!     })
//! }
//!
//! /// `futures` version of `Serial.write`
//! ///
//! /// This returns a future that must be polled to completion
//! fn write<S>(serial: S, byte: u8) -> impl Future<Item = S, Error = S::Error>
//! where
//!     S: hal::serial::Write<u8>,
//! {
//!     future::loop_fn(serial, move |mut serial| {
//!         match serial.write(byte) {
//!             Ok(())                       => Ok(Loop::Break(serial)),
//!             Err(nb::Error::WouldBlock)   => Ok(Loop::Continue(serial)),
//!             Err(nb::Error::Other(error)) => Err(error),
//!         }
//!     })
//! }
//!
//! // HAL implementers
//! let timer = Timer(tim3);
//! let serial = Serial(usart1);
//!
//! // Tasks
//! let mut blinky = future::loop_fn(true, |state| {
//!     wait(timer).map(|_| {
//!         if state {
//!             Led.on();
//!         } else {
//!             Led.off();
//!         }
//!
//!         Loop::Continue(!state)
//!     });
//! });
//!
//! let mut loopback = future::loop_fn((), |_| {
//!     read(serial).and_then(|byte| {
//!         write(serial, byte)
//!     }).map(|_| {
//!         Loop::Continue(())
//!     });
//! });
//!
//! // Event loop
//! loop {
//!     blinky().poll().unwrap(); // NOTE(unwrap) E = !
//!     loopback().poll().unwrap();
//! }
//! ```
//!
//! ### `await`
//!
//! Same example as above but using `await!` instead of `futures`.
//!
//! **NOTE** The `await!` macro requires language support for generators, which
//! is not yet in the compiler.
//!
//! ``` ignore
//! extern crate embedded_hal as hal;
//!
//! extern crate stm32f103xx_hal_impl;
//!
//! #[macro_use]
//! extern crate nb;
//!
//! use hal::prelude::*;
//! use stm32f103xx_hal_impl::{Serial, Timer};
//!
//! // HAL implementers
//! let timer = Timer(tim3);
//! let serial = Serial(usart1);
//!
//! // Tasks
//! let mut blinky = (|| {
//!     let mut state = false;
//!     loop {
//!         // `await!` means "suspend / yield here" instead of "block until
//!         // completion"
//!         await!(timer.wait()).unwrap(); // NOTE(unwrap) E = !
//!
//!         state = !state;
//!
//!         if state {
//!              Led.on();
//!         } else {
//!              Led.off();
//!         }
//!     }
//! })();
//!
//! let mut loopback = (|| {
//!     loop {
//!         let byte = await!(serial.read()).unwrap();
//!         await!(serial.write(byte)).unwrap();
//!     }
//! })();
//!
//! // Event loop
//! loop {
//!     blinky.resume();
//!     serial.resume();
//! }
//! ```
//!
//! ## Generic programming and higher level abstractions
//!
//! The HAL has been kept minimal on purpose to encourage building **generic**
//! higher level abstractions on top of it.
//!
//! Some examples:
//!
//! **NOTE** All the functions shown below could have been written as trait
//! methods with default implementation to allow specialization, but they have
//! been written as functions to keep things simple.
//!
//! - Write a whole buffer in blocking fashion.
//!
//! ``` ignore
//! extern crate embedded_hal as hal;
//! #[macro_use]
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
//! ```
//!
//! - Blocking read with timeout
//!
//! ``` ignore
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
//!     T: hal::Timer,
//!     S: hal::serial::Read<u8>,
//! {
//!     timer.pause();
//!     timer.restart();
//!     timer.set_timeout(timeout);
//!     timer.resume();
//!
//!     loop {
//!         match serial.read() {
//!             Err(nb::Error::Other(e)) => return Err(Error::Serial(e)),
//!             Err(nb::Error::WouldBlock) => {
//!                 // no data available, check the timer below
//!             },
//!             Ok(byte) => return Ok(byte),
//!         }
//!
//!         match timer.wait() {
//!             Err(nb::Error::Other(e)) => {
//!                 // The error type specified by `timer.wait()` is `!`, which
//!                 // means no error can actually occur. The Rust compiler
//!                 // still forces us to provide this match arm, though.
//!                 e
//!             },
//!             Err(nb::Error::WouldBlock) => continue,
//!             Ok(()) => {
//!                 timer.pause();
//!                 return Err(Error::TimedOut);
//!             },
//!         }
//!     }
//! }
//! ```
//!
//! - Asynchronous SPI transfer
//!
//! ``` ignore
//! extern crate embedded_hal as hal;
//!
//! /// Transfers a byte buffer of size N
//! ///
//! /// Returns the same byte buffer but filled with the data received from the
//! /// slave device
//! #[async]
//! fn transfer<const N: usize, S>(
//!     spi: &S,
//!     mut buffer: [u8; N],
//! ) -> Result<[u8; N], S::Error>
//! where
//!     S: hal::Spi,
//! {
//!     for byte in &mut buffer {
//!         await!(spi.send(byte))?;
//!         *byte = await!(spi.receive())?;
//!     }
//!
//!     buffer
//! }
//! ```
//!
//! - Buffered serial interface with periodic flushing in interrupt handler
//!
//! ``` ignore
//! extern crate embedded_hal as hal;
//! extern crate nb;
//!
//! use hal::prelude::*;
//!
//! fn flush<S>(serial: &mut S, cb: &mut CircularBuffer) -> Result<(), S::Error>
//! where
//!     S: hal::serial::Write<u8>,
//! {
//!     loop {
//!         if let Some(byte) = cb.peek() {
//!             match serial.write(*byte) {
//!                 Err(nb::Error::Other(e)) => return Err(e),
//!                 Err(nb::Error::WouldBlock) => return Ok(()),
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
//! pub struct BufferedSerial;
//!
//! // NOTE private
//! static BUFFER: Mutex<CircularBuffer> = ..;
//! static SERIAL: Mutex<impl hal::serial::Write<u8>> = ..;
//!
//! impl BufferedSerial {
//!     pub fn write(&self, bytes: &[u8]) {
//!         let mut buffer = BUFFER.lock();
//!         for byte in bytes {
//!             buffer.push(*byte).unwrap();
//!         }
//!     }
//!
//!     pub fn write_all(&self, bytes: &[u8]) {
//!         let mut buffer = BUFFER.lock();
//!         for byte in bytes {
//!             buffer.push(*byte).unwrap();
//!         }
//!     }
//! }
//!
//! fn interrupt_handler() {
//!     let serial = SERIAL.lock();
//!     let buffer = BUFFER.lock();
//!
//!     flush(&mut serial, &mut buffer).unwrap();
//! }
//! ```
//!
//! # A note on time units
//!
//! Implementers of this HAL are encouraged to use *application agnostic* time
//! units in their implementations. Is it usually the case that the application
//! will want to pick the clock frequencies of the different peripherals so
//! using a unit like `apb1::Ticks` instead of `Seconds` lets the application
//! perform the conversions cheaply without register / memory accesses.
//!
//! For example: a `Timer` implementation uses the peripheral TIM1 which is
//! connected to the APB1 bus. This implementation uses the time unit
//! `apb1::Ticks` where 1 tick is equal to `1 / apb1::FREQUENCY` seconds where
//! `apb1::FREQUENCY` is picked by the application.
//!
//! Now each application can declare the `apb1::FREQUENCY` using a macro that
//! expands into conversions (implementations of [the `From` trait]) from human
//! time units like `Milliseconds` to `apb1::Ticks`.
//!
//! [the `From` trait]: https://doc.rust-lang.org/core/convert/trait.From.html
//!
//! With this setup the application can use human time units with the `Timer`
//! API:
//!
//! ``` ignore
//! frequency!(apb1, 8_000_000); // Hz
//!
//! let timer: impl Timer = ..;
//!
//! // All these are equivalent
//! timer.set_timeout(apb1::Ticks(8_000));
//! timer.set_timeout(Milliseconds(1));
//! timer.set_timeout(1.ms());
//! ```
//!
//! See the [`blue-pill`] crate for an example implementation of this approach.

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(never_type)]
#![no_std]

extern crate nb;

pub mod prelude;
pub mod serial;

/// Input capture
///
/// # Examples
///
/// You can use this interface to measure the period of (quasi) periodic signals
/// / events
///
/// ``` ignore
/// let capture: impl Capture = ..;
///
/// capture.set_resolution(1.ms());
///
/// let before = block!(capture.capture()).unwrap();
/// let after = block!(capture.capture()).unwrap();
///
/// let period = after.wrapping_sub(before);
///
/// println!("Period: {} ms", period);
/// ```
pub trait Capture {
    /// Enumeration of `Capture` errors
    ///
    /// Possible errors:
    ///
    /// - *overcapture*, the previous capture value was overwritten because it
    ///   was not read in a timely manner
    type Error;

    /// Enumeration of channels that can be used with this `Capture` interface
    ///
    /// If your `Capture` interface has no channels you can use the type `()`
    /// here
    type Channel;

    /// A time unit that can be converted into a human time unit (e.g. seconds)
    type Time;

    /// The type of the value returned by `capture`
    type Capture;

    /// "Waits" for a transition in the capture `channel` and returns the value
    /// of counter at that instant
    ///
    /// NOTE that you must multiply the returned value by the *resolution* of
    /// this `Capture` interface to get a human time unit (e.g. seconds)
    fn capture(
        &mut self,
        channel: Self::Channel,
    ) -> nb::Result<Self::Capture, Self::Error>;

    /// Disables a capture `channel`
    fn disable(&mut self, channel: Self::Channel);

    /// Enables a capture `channel`
    fn enable(&mut self, channel: Self::Channel);

    /// Returns the current resolution
    fn get_resolution(&self) -> Self::Time;

    /// Sets the resolution of the capture timer
    fn set_resolution<R>(&mut self, resolution: R)
    where
        R: Into<Self::Time>;
}

/// Pulse Width Modulation
///
/// # Examples
///
/// Use this interface to control the power output of some actuator
///
/// ``` ignore
/// let pwm: impl Pwm = ..;
///
/// pwm.set_period(1.khz().invert());
///
/// let max_duty = pwm.get_max_duty();
///
/// // brightest LED
/// pwm.set_duty(Channel::_1, max_duty);
///
/// // dimmer LED
/// pwm.set_duty(Channel::_2, max_duty / 4);
/// ```
pub trait Pwm {
    /// Enumeration of channels that can be used with this `Pwm` interface
    ///
    /// If your `Pwm` interface has no channels you can use the type `()`
    /// here
    type Channel;

    /// A time unit that can be converted into a human time unit (e.g. seconds)
    type Time;

    /// Type for the `duty` methods
    ///
    /// The implementer is free to choose a float / percentage representation
    /// (e.g. `0.0 .. 1.0`) or an integer representation (e.g. `0 .. 65535`)
    type Duty;

    /// Disables a PWM `channel`
    fn disable(&mut self, channel: Self::Channel);

    /// Enables a PWM `channel`
    fn enable(&mut self, channel: Self::Channel);

    /// Returns the current PWM period
    fn get_period(&self) -> Self::Time;

    /// Returns the current duty cycle
    fn get_duty(&self, channel: Self::Channel) -> Self::Duty;

    /// Returns the maximum duty cycle value
    fn get_max_duty(&self) -> Self::Duty;

    /// Sets a new duty cycle
    fn set_duty(&mut self, channel: Self::Channel, duty: Self::Duty);

    /// Sets a new PWM period
    fn set_period<P>(&mut self, period: P)
    where
        P: Into<Self::Time>;
}

/// Quadrature encoder interface
///
/// # Examples
///
/// You can use this interface to measure the speed of a motor
///
/// ``` ignore
/// let qei: impl Qei = ..;
/// let timer: impl Timer = ..;
///
/// timer.pause();
/// timer.restart();
/// timer.set_timeout(1.s());
///
/// let before = qei.count();
/// timer.resume();
/// block!(timer.wait());
/// let after = qei.count();
///
/// let speed = after.wrapping_sub(before);
/// println!("Speed: {} pulses per second", speed);
/// ```
pub trait Qei {
    /// The type of the value returned by `count`
    type Count;

    /// Returns the current pulse count of the encoder
    fn count(&self) -> Self::Count;

    /// Returns the count direction
    fn direction(&self) -> Direction;
}

/// Count direction
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    /// 3, 2, 1
    Downcounting,
    /// 1, 2, 3
    Upcounting,
}

/// Serial Peripheral Interface (full duplex master mode)
///
/// # Notes
///
/// - It's the task of the user of this interface to manage the slave select
///   lines
///
/// - Due to how full duplex SPI works each `send` call must be followed by a
///   `read` call to avoid overruns.
///
/// - Some SPIs can work with 8-bit *and* 16-bit words. You can overload this
///   trait with different `Word` types to allow operation in both modes.
pub trait Spi<Word> {
    /// An enumeration of SPI errors
    ///
    /// Possible errors
    ///
    /// - *overrun*, the shift register was not `read` between two consecutive
    ///   `send` calls.
    type Error;

    /// Reads the word stored in the shift register
    ///
    /// **NOTE** A word must be sent to the slave before attempting to call this
    /// method.
    fn read(&mut self) -> nb::Result<Word, Self::Error>;

    /// Sends a word to the slave
    fn send(&mut self, word: Word) -> nb::Result<(), Self::Error>;
}

/// Timer used for timeouts
///
/// # Examples
///
/// You can use this timer to create delays
///
/// ``` ignore
/// let timer: impl Timer = ..;
///
/// timer.pause();
/// timer.restart();
/// timer.set_timeout(1.s());
///
/// Led.on();
/// timer.resume();
/// block!(timer.wait()); // blocks for 1 second
/// Led.off();
/// ```
pub trait Timer {
    /// A time unit that can be converted into a human time unit (e.g. seconds)
    type Time;

    /// Returns the current timeout
    fn get_timeout(&self) -> Self::Time;

    /// Pauses the timer
    fn pause(&mut self);

    /// Restarts the timer count to zero
    fn restart(&mut self);

    /// Resumes the timer count
    fn resume(&mut self);

    /// Sets a new timeout
    fn set_timeout<T>(&mut self, timeout: T)
    where
        T: Into<Self::Time>;

    /// "Waits" until the timer times out
    fn wait(&self) -> nb::Result<(), !>;
}
