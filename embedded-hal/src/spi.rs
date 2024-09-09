//! Blocking SPI master mode traits.
//!
//! # Bus vs Device
//!
//! SPI allows sharing a single bus between many SPI devices. The SCK, MOSI and MISO lines are
//! wired in parallel to all the devices, and each device gets a dedicated chip-select (CS) line from the MCU, like this:
//!
#![doc= include_str!("spi-shared-bus.svg")]
//!
//! CS is usually active-low. When CS is high (not asserted), SPI devices ignore all incoming data, and
//! don't drive MISO. When CS is low (asserted), the device is active: reacts to incoming data on MOSI and
//! drives MISO with the response data. By asserting one CS or another, the MCU can choose to which
//! SPI device it "talks" to on the (possibly shared) bus.
//!
//! This bus sharing is common when having multiple SPI devices in the same board, since it uses fewer MCU
//! pins (`n+3` instead of `4*n`), and fewer MCU SPI peripherals (`1` instead of `n`).
//!
//! However, it poses a challenge when building portable drivers for SPI devices. The driver needs to
//! be able to talk to its device on the bus, while not interfering with other drivers talking to other
//! devices.
//!
//! To solve this, `embedded-hal` has two kinds of SPI traits: **SPI bus** and **SPI device**.
//!
//! ## Bus
//!
//! The [`SpiBus`] trait represents **exclusive ownership** over the whole SPI bus. This is usually the entire
//! SPI MCU peripheral, plus the SCK, MOSI and MISO pins.
//!
//! Owning an instance of an SPI bus guarantees exclusive access, this is, we have the guarantee no other
//! piece of code will try to use the bus while we own it.
//!
//! ## Device
//!
//! The [`SpiDevice`] trait represents **ownership over a single SPI device selected by a CS pin** in a (possibly shared) bus. This is typically:
//!
//! - Exclusive ownership of the **CS pin**.
//! - Access to the **underlying SPI bus**. If shared, it'll be behind some kind of lock/mutex.
//!
//! An [`SpiDevice`] allows initiating [transactions](SpiDevice::transaction) against the target device on the bus. A transaction
//! consists of asserting CS, then doing one or more transfers, then deasserting CS. For the entire duration of the transaction, the [`SpiDevice`]
//! implementation will ensure no other transaction can be opened on the same bus. This is the key that allows correct sharing of the bus.
//!
//! # For driver authors
//!
//! When implementing a driver, it's crucial to pick the right trait, to ensure correct operation
//! with maximum interoperability. Here are some guidelines depending on the device you're implementing a driver for:
//!
//! If your device **has a CS pin**, use [`SpiDevice`]. Do not manually
//! manage the CS pin, the [`SpiDevice`] implementation will do it for you.
//! By using [`SpiDevice`], your driver will cooperate nicely with other drivers for other devices in the same shared SPI bus.
//!
//! ```
//! # use embedded_hal::spi::{SpiBus, SpiDevice, Operation};
//! pub struct MyDriver<SPI> {
//!     spi: SPI,
//! }
//!
//! impl<SPI> MyDriver<SPI>
//! where
//!     SPI: SpiDevice,
//! {
//!     pub fn new(spi: SPI) -> Self {
//!         Self { spi }
//!     }
//!
//!     pub fn read_foo(&mut self) -> Result<[u8; 2], MyError<SPI::Error>> {
//!         let mut buf = [0; 2];
//!
//!         // `transaction` asserts and deasserts CS for us. No need to do it manually!
//!         self.spi.transaction(&mut [
//!             Operation::Write(&[0x90]),
//!             Operation::Read(&mut buf),
//!         ]).map_err(MyError::Spi)?;
//!
//!         Ok(buf)
//!     }
//! }
//!
//! #[derive(Copy, Clone, Debug)]
//! enum MyError<SPI> {
//!     Spi(SPI),
//!     // Add other errors for your driver here.
//! }
//! ```
//!
//! If your device **does not have a CS pin**, use [`SpiBus`]. This will ensure
//! your driver has exclusive access to the bus, so no other drivers can interfere. It's not possible to safely share
//! a bus without CS pins. By requiring [`SpiBus`] you disallow sharing, ensuring correct operation.
//!
//! ```
//! # use embedded_hal::spi::SpiBus;
//! pub struct MyDriver<SPI> {
//!     spi: SPI,
//! }
//!
//! impl<SPI> MyDriver<SPI>
//! where
//!     SPI: SpiBus,
//! {
//!     pub fn new(spi: SPI) -> Self {
//!         Self { spi }
//!     }
//!
//!     pub fn read_foo(&mut self) -> Result<[u8; 2], MyError<SPI::Error>> {
//!         let mut buf = [0; 2];
//!         self.spi.write(&[0x90]).map_err(MyError::Spi)?;
//!         self.spi.read(&mut buf).map_err(MyError::Spi)?;
//!         Ok(buf)
//!     }
//! }
//!
//! #[derive(Copy, Clone, Debug)]
//! enum MyError<SPI> {
//!     Spi(SPI),
//!     // Add other errors for your driver here.
//! }
//! ```
//!
//! If you're (ab)using SPI to **implement other protocols** by bitbanging (WS2812B, onewire, generating arbitrary waveforms...), use [`SpiBus`].
//! SPI bus sharing doesn't make sense at all in this case. By requiring [`SpiBus`] you disallow sharing, ensuring correct operation.
//!
//! # For HAL authors
//!
//! HALs **must** implement [`SpiBus`]. Users can combine the bus together with the CS pin (which should
//! implement [`OutputPin`](crate::digital::OutputPin)) using HAL-independent [`SpiDevice`] implementations such as the ones in [`embedded-hal-bus`](https://crates.io/crates/embedded-hal-bus).
//!
//! HALs may additionally implement [`SpiDevice`] to **take advantage of hardware CS management**, which may provide some performance
//! benefits. (There's no point in a HAL implementing [`SpiDevice`] if the CS management is software-only, this task is better left to
//! the HAL-independent implementations).
//!
//! HALs **must not** add infrastructure for sharing at the [`SpiBus`] level. User code owning a [`SpiBus`] must have the guarantee
//! of exclusive access.
//!
//! # Flushing
//!
//! To improve performance, [`SpiBus`] implementations are allowed to return before the operation is finished, i.e. when the bus is still not
//! idle. This allows pipelining SPI transfers with CPU work.
//!
//! When calling another method when a previous operation is still in progress, implementations can either wait for the previous operation
//! to finish, or enqueue the new one, but they must not return a "busy" error. Users must be able to do multiple method calls in a row
//! and have them executed "as if" they were done sequentially, without having to check for "busy" errors.
//!
//! When using a [`SpiBus`], call [`flush`](SpiBus::flush) to wait for operations to actually finish. Examples of situations
//! where this is needed are:
//! - To synchronize SPI activity and GPIO activity, for example before deasserting a CS pin.
//! - Before deinitializing the hardware SPI peripheral.
//!
//! When using a [`SpiDevice`], you can still call [`flush`](SpiBus::flush) on the bus within a transaction.
//! It's very rarely needed, because [`transaction`](SpiDevice::transaction) already flushes for you
//! before deasserting CS. For example, you may need it to synchronize with GPIOs other than CS, such as DCX pins
//! sometimes found in SPI displays.
//!
//! For example, for [`write`](SpiBus::write) operations, it is common for hardware SPI peripherals to have a small
//! FIFO buffer, usually 1-4 bytes. Software writes data to the FIFO, and the peripheral sends it on MOSI at its own pace,
//! at the specified SPI frequency. It is allowed for an implementation of [`write`](SpiBus::write) to return as soon
//! as all the data has been written to the FIFO, before it is actually sent. Calling [`flush`](SpiBus::flush) would
//! wait until all the bits have actually been sent, the FIFO is empty, and the bus is idle.
//!
//! This still applies to other operations such as [`read`](SpiBus::read) or [`transfer`](SpiBus::transfer). It is less obvious
//! why, because these methods can't return before receiving all the read data. However it's still technically possible
//! for them to return before the bus is idle. For example, assuming SPI mode 0, the last bit is sampled on the first (rising) edge
//! of SCK, at which point a method could return, but the second (falling) SCK edge still has to happen before the bus is idle.
//!
//! # CS-to-clock delays
//!
//! Many chips require a minimum delay between asserting CS and the first SCK edge, and the last SCK edge and deasserting CS.
//! Drivers should *NOT* use [`Operation::DelayNs`] for this, they should instead document that the user should configure the
//! delays when creating the `SpiDevice` instance, same as they have to configure the SPI frequency and mode. This has a few advantages:
//!
//! - Allows implementations that use hardware-managed CS to program the delay in hardware
//! - Allows the end user more flexibility. For example, they can choose to not configure any delay if their MCU is slow
//!   enough to "naturally" do the delay (very common if the delay is in the order of nanoseconds).

use core::fmt::Debug;

#[cfg(feature = "defmt-03")]
use crate::defmt;

/// Clock polarity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Polarity {
    /// Clock signal low when idle.
    IdleLow,
    /// Clock signal high when idle.
    IdleHigh,
}

/// Clock phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Phase {
    /// Data in "captured" on the first clock transition.
    CaptureOnFirstTransition,
    /// Data in "captured" on the second clock transition.
    CaptureOnSecondTransition,
}

/// SPI mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct Mode {
    /// Clock polarity.
    pub polarity: Polarity,
    /// Clock phase.
    pub phase: Phase,
}

/// Helper for CPOL = 0, CPHA = 0.
pub const MODE_0: Mode = Mode {
    polarity: Polarity::IdleLow,
    phase: Phase::CaptureOnFirstTransition,
};

/// Helper for CPOL = 0, CPHA = 1.
pub const MODE_1: Mode = Mode {
    polarity: Polarity::IdleLow,
    phase: Phase::CaptureOnSecondTransition,
};

/// Helper for CPOL = 1, CPHA = 0.
pub const MODE_2: Mode = Mode {
    polarity: Polarity::IdleHigh,
    phase: Phase::CaptureOnFirstTransition,
};

/// Helper for CPOL = 1, CPHA = 1.
pub const MODE_3: Mode = Mode {
    polarity: Polarity::IdleHigh,
    phase: Phase::CaptureOnSecondTransition,
};

/// SPI error.
pub trait Error: Debug {
    /// Convert error to a generic SPI error kind.
    ///
    /// By using this method, SPI errors freely defined by HAL implementations
    /// can be converted to a set of generic SPI errors upon which generic
    /// code can act.
    fn kind(&self) -> ErrorKind;
}

impl core::fmt::Display for dyn Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl core::error::Error for dyn Error {}

impl Error for core::convert::Infallible {
    #[inline]
    fn kind(&self) -> ErrorKind {
        match *self {}
    }
}

/// SPI error kind.
///
/// This represents a common set of SPI operation errors. HAL implementations are
/// free to define more specific or additional error types. However, by providing
/// a mapping to these common SPI errors, generic code can still react to them.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[non_exhaustive]
pub enum ErrorKind {
    /// The peripheral receive buffer was overrun.
    Overrun,
    /// Multiple devices on the SPI bus are trying to drive the slave select pin, e.g. in a multi-master setup.
    ModeFault,
    /// Received data does not conform to the peripheral configuration.
    FrameFormat,
    /// An error occurred while asserting or deasserting the Chip Select pin.
    ChipSelectFault,
    /// A different error occurred. The original error may contain more information.
    Other,
}

impl Error for ErrorKind {
    #[inline]
    fn kind(&self) -> ErrorKind {
        *self
    }
}

impl core::fmt::Display for ErrorKind {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Overrun => write!(f, "The peripheral receive buffer was overrun"),
            Self::ModeFault => write!(
                f,
                "Multiple devices on the SPI bus are trying to drive the slave select pin"
            ),
            Self::FrameFormat => write!(
                f,
                "Received data does not conform to the peripheral configuration"
            ),
            Self::ChipSelectFault => write!(
                f,
                "An error occurred while asserting or deasserting the Chip Select pin"
            ),
            Self::Other => write!(
                f,
                "A different error occurred. The original error may contain more information"
            ),
        }
    }
}

/// SPI error type trait.
///
/// This just defines the error type, to be used by the other SPI traits.
pub trait ErrorType {
    /// Error type.
    type Error: Error;
}

impl<T: ErrorType + ?Sized> ErrorType for &mut T {
    type Error = T::Error;
}

/// SPI transaction operation.
///
/// This allows composition of SPI operations into a single bus transaction.
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Operation<'a, Word: 'static> {
    /// Read data into the provided buffer.
    ///
    /// Equivalent to [`SpiBus::read`].
    Read(&'a mut [Word]),
    /// Write data from the provided buffer, discarding read data.
    ///
    /// Equivalent to [`SpiBus::write`].
    Write(&'a [Word]),
    /// Read data into the first buffer, while writing data from the second buffer.
    ///
    /// Equivalent to [`SpiBus::transfer`].
    Transfer(&'a mut [Word], &'a [Word]),
    /// Write data out while reading data into the provided buffer.
    ///
    /// Equivalent to [`SpiBus::transfer_in_place`].
    TransferInPlace(&'a mut [Word]),
    /// Delay for at least the specified number of nanoseconds.
    DelayNs(u32),
}

/// SPI device trait.
///
/// `SpiDevice` represents ownership over a single SPI device on a (possibly shared) bus, selected
/// with a CS (Chip Select) pin.
///
/// See the [module-level documentation](self) for important usage information.
pub trait SpiDevice<Word: Copy + 'static = u8>: ErrorType {
    /// Perform a transaction against the device.
    ///
    /// - Locks the bus
    /// - Asserts the CS (Chip Select) pin.
    /// - Performs all the operations.
    /// - [Flushes](SpiBus::flush) the bus.
    /// - Deasserts the CS pin.
    /// - Unlocks the bus.
    ///
    /// The locking mechanism is implementation-defined. The only requirement is it must prevent two
    /// transactions from executing concurrently against the same bus. Examples of implementations are:
    /// critical sections, blocking mutexes, returning an error or panicking if the bus is already busy.
    ///
    /// On bus errors the implementation should try to deassert CS.
    /// If an error occurs while deasserting CS the bus error should take priority as the return value.
    fn transaction(&mut self, operations: &mut [Operation<'_, Word>]) -> Result<(), Self::Error>;

    /// Do a read within a transaction.
    ///
    /// This is a convenience method equivalent to `device.transaction(&mut [Operation::Read(buf)])`.
    ///
    /// See also: [`SpiDevice::transaction`], [`SpiBus::read`]
    #[inline]
    fn read(&mut self, buf: &mut [Word]) -> Result<(), Self::Error> {
        self.transaction(&mut [Operation::Read(buf)])
    }

    /// Do a write within a transaction.
    ///
    /// This is a convenience method equivalent to `device.transaction(&mut [Operation::Write(buf)])`.
    ///
    /// See also: [`SpiDevice::transaction`], [`SpiBus::write`]
    #[inline]
    fn write(&mut self, buf: &[Word]) -> Result<(), Self::Error> {
        self.transaction(&mut [Operation::Write(buf)])
    }

    /// Do a transfer within a transaction.
    ///
    /// This is a convenience method equivalent to `device.transaction(&mut [Operation::Transfer(read, write)]`.
    ///
    /// See also: [`SpiDevice::transaction`], [`SpiBus::transfer`]
    #[inline]
    fn transfer(&mut self, read: &mut [Word], write: &[Word]) -> Result<(), Self::Error> {
        self.transaction(&mut [Operation::Transfer(read, write)])
    }

    /// Do an in-place transfer within a transaction.
    ///
    /// This is a convenience method equivalent to `device.transaction(&mut [Operation::TransferInPlace(buf)]`.
    ///
    /// See also: [`SpiDevice::transaction`], [`SpiBus::transfer_in_place`]
    #[inline]
    fn transfer_in_place(&mut self, buf: &mut [Word]) -> Result<(), Self::Error> {
        self.transaction(&mut [Operation::TransferInPlace(buf)])
    }
}

impl<Word: Copy + 'static, T: SpiDevice<Word> + ?Sized> SpiDevice<Word> for &mut T {
    #[inline]
    fn transaction(&mut self, operations: &mut [Operation<'_, Word>]) -> Result<(), Self::Error> {
        T::transaction(self, operations)
    }

    #[inline]
    fn read(&mut self, buf: &mut [Word]) -> Result<(), Self::Error> {
        T::read(self, buf)
    }

    #[inline]
    fn write(&mut self, buf: &[Word]) -> Result<(), Self::Error> {
        T::write(self, buf)
    }

    #[inline]
    fn transfer(&mut self, read: &mut [Word], write: &[Word]) -> Result<(), Self::Error> {
        T::transfer(self, read, write)
    }

    #[inline]
    fn transfer_in_place(&mut self, buf: &mut [Word]) -> Result<(), Self::Error> {
        T::transfer_in_place(self, buf)
    }
}

/// SPI bus.
///
/// `SpiBus` represents **exclusive ownership** over the whole SPI bus, with SCK, MOSI and MISO pins.
///
/// See the [module-level documentation](self) for important information on SPI Bus vs Device traits.
pub trait SpiBus<Word: Copy + 'static = u8>: ErrorType {
    /// Read `words` from the slave.
    ///
    /// The word value sent on MOSI during reading is implementation-defined,
    /// typically `0x00`, `0xFF`, or configurable.
    ///
    /// Implementations are allowed to return before the operation is
    /// complete. See the [module-level documentation](self) for details.
    fn read(&mut self, words: &mut [Word]) -> Result<(), Self::Error>;

    /// Write `words` to the slave, ignoring all the incoming words.
    ///
    /// Implementations are allowed to return before the operation is
    /// complete. See the [module-level documentation](self) for details.
    fn write(&mut self, words: &[Word]) -> Result<(), Self::Error>;

    /// Write and read simultaneously. `write` is written to the slave on MOSI and
    /// words received on MISO are stored in `read`.
    ///
    /// It is allowed for `read` and `write` to have different lengths, even zero length.
    /// The transfer runs for `max(read.len(), write.len())` words. If `read` is shorter,
    /// incoming words after `read` has been filled will be discarded. If `write` is shorter,
    /// the value of words sent in MOSI after all `write` has been sent is implementation-defined,
    /// typically `0x00`, `0xFF`, or configurable.
    ///
    /// Implementations are allowed to return before the operation is
    /// complete. See the [module-level documentation](self) for details.
    fn transfer(&mut self, read: &mut [Word], write: &[Word]) -> Result<(), Self::Error>;

    /// Write and read simultaneously. The contents of `words` are
    /// written to the slave, and the received words are stored into the same
    /// `words` buffer, overwriting it.
    ///
    /// Implementations are allowed to return before the operation is
    /// complete. See the [module-level documentation](self) for details.
    fn transfer_in_place(&mut self, words: &mut [Word]) -> Result<(), Self::Error>;

    /// Wait until all operations have completed and the bus is idle.
    ///
    /// See the [module-level documentation](self) for important usage information.
    fn flush(&mut self) -> Result<(), Self::Error>;
}

impl<T: SpiBus<Word> + ?Sized, Word: Copy + 'static> SpiBus<Word> for &mut T {
    #[inline]
    fn read(&mut self, words: &mut [Word]) -> Result<(), Self::Error> {
        T::read(self, words)
    }

    #[inline]
    fn write(&mut self, words: &[Word]) -> Result<(), Self::Error> {
        T::write(self, words)
    }

    #[inline]
    fn transfer(&mut self, read: &mut [Word], write: &[Word]) -> Result<(), Self::Error> {
        T::transfer(self, read, write)
    }

    #[inline]
    fn transfer_in_place(&mut self, words: &mut [Word]) -> Result<(), Self::Error> {
        T::transfer_in_place(self, words)
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        T::flush(self)
    }
}
