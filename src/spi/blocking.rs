//! Blocking SPI API
//!
//! # Bus vs Device
//!
//! SPI allows sharing a single bus between many SPI devices. The SCK, MOSI and MISO lines are
//! wired in parallel to all the devices, and each device gets a dedicated chip-select (CS) line from the MCU, like this:
//!
#![doc= include_str!("shared-bus.svg")]
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
//! SPI bus traits represent **exclusive ownership** over the whole SPI bus. This is usually the entire
//! SPI MCU peripheral, plus the SCK, MOSI and MISO pins.
//!
//! Owning an instance of an SPI bus guarantees exclusive access, this is, we have the guarantee no other
//! piece of code will try to use the bus while we own it.
//!
//! There's 3 bus traits, depending on the bus capabilities.
//!
//! - [`SpiBus`]: Read-write access. This is the most commonly used.
//! - [`SpiBusRead`]: Read-only access, for example a bus with a MISO pin but no MOSI pin.
//! - [`SpiBusWrite`]: Read-write access, for example a bus with a MOSI pin but no MISO pin.
//!
//! ## Device
//!
//! [`SpiDevice`] represents **ownership over a single SPI device selected by a CS pin** in a (possibly shared) bus. This is typically:
//!
//! - Exclusive ownership of the **CS pin**.
//! - Access to the **underlying SPI bus**. If shared, it'll be behind some kind of lock/mutex.
//!
//! An [`SpiDevice`] allows initiating [transactions](SpiDevice::transaction) against the target device on the bus. A transaction
//! consists of asserting CS, then doing one or more transfers, then deasserting CS. For the entire duration of the transaction, the [`SpiDevice`]
//! implementation will ensure no other transaction can be opened on the same bus. This is the key that allows correct sharing of the bus.
//!
//! The capabilities of the bus (read-write, read-only or write-only) are determined by which of the [`SpiBus`], [`SpiBusRead`] [`SpiBusWrite`] traits
//! are implemented for the [`Bus`](SpiDevice::Bus) associated type.
//!
//! # For driver authors
//!
//! When implementing a driver, it's crucial to pick the right trait, to ensure correct operation
//! with maximum interoperability. Here are some guidelines depending on the device you're implementing a driver for:
//!
//! If your device **has a CS pin**, use [`SpiDevice`]. Do not manually manage the CS pin, the [`SpiDevice`] implementation will do it for you.
//! Add bounds like `where T::Bus: SpiBus`, `where T::Bus: SpiBusRead`, `where T::Bus: SpiBusWrite` to specify the kind of access you need.
//! By using [`SpiDevice`], your driver will cooperate nicely with other drivers for other devices in the same shared SPI bus.
//!
//! ```
//! # use embedded_hal::spi::blocking::{SpiBus, SpiBusRead, SpiBusWrite, SpiDevice};
//! pub struct MyDriver<SPI> {
//!     spi: SPI,
//! }
//!
//! impl<SPI> MyDriver<SPI>
//! where
//!     SPI: SpiDevice,
//!     SPI::Bus: SpiBus, // or SpiBusRead/SpiBusWrite if you only need to read or only write.
//! {
//!     pub fn new(spi: SPI) -> Self {
//!         Self { spi }
//!     }
//!
//!     pub fn read_foo(&mut self) -> Result<[u8; 2], MyError<SPI::Error>> {
//!         let mut buf = [0; 2];
//!
//!         // `transaction` asserts and deasserts CS for us. No need to do it manually!
//!         self.spi.transaction(|bus| {
//!             bus.write(&[0x90])?;
//!             bus.read(&mut buf)
//!         }).map_err(MyError::Spi)?;
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
//! If your device **does not have a CS pin**, use [`SpiBus`] (or [`SpiBusRead`], [`SpiBusWrite`]). This will ensure
//! your driver has exclusive access to the bus, so no other drivers can interfere. It's not possible to safely share
//! a bus without CS pins. By requiring [`SpiBus`] you disallow sharing, ensuring correct operation.
//!
//! ```
//! # use embedded_hal::spi::blocking::{SpiBus, SpiBusRead, SpiBusWrite};
//! pub struct MyDriver<SPI> {
//!     spi: SPI,
//! }
//!
//! impl<SPI> MyDriver<SPI>
//! where
//!     SPI: SpiBus, // or SpiBusRead/SpiBusWrite if you only need to read or only write.
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
//! HALs **must** implement [`SpiBus`], [`SpiBusRead`] and [`SpiBusWrite`]. Users can combine the bus together with the CS pin (which should
//! implement [`OutputPin`]) using HAL-independent [`SpiDevice`] implementations such as [`ExclusiveDevice`].
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
//! idle.
//!
//! When using a [`SpiBus`], call [`flush`](SpiBusFlush::flush) to wait for operations to actually finish. Examples of situations
//! where this is needed are:
//! - To synchronize SPI activity and GPIO activity, for example before deasserting a CS pin.
//! - Before deinitializing the hardware SPI peripheral.
//!
//! When using a [`SpiDevice`], you can still call [`flush`](SpiBusFlush::flush) on the bus within a transaction.
//! It's very rarely needed, because [`transaction`](SpiDevice::transaction) already flushes for you
//! before deasserting CS. For example, you may need it to synchronize with GPIOs other than CS, such as DCX pins
//! sometimes found in SPI displays.
//!
//! For example, for [`write`](SpiBusWrite::write) operations, it is common for hardware SPI peripherals to have a small
//! FIFO buffer, usually 1-4 bytes. Software writes data to the FIFO, and the peripheral sends it on MOSI at its own pace,
//! at the specified SPI frequency. It is allowed for an implementation of [`write`](SpiBusWrite::write) to return as soon
//! as all the data has been written to the FIFO, before it is actually sent. Calling [`flush`](SpiBusFlush::flush) would
//! wait until all the bits have actually been sent, the FIFO is empty, and the bus is idle.
//!
//! This still applies to other operations such as [`read`](SpiBusRead::read) or [`transfer`](SpiBus::transfer). It is less obvious
//! why, because these methods can't return before receiving all the read data. However it's still technically possible
//! for them to return before the bus is idle. For example, assuming SPI mode 0, the last bit is sampled on the first (rising) edge
//! of SCK, at which point a method could return, but the second (falling) SCK edge still has to happen before the bus is idle.

use core::fmt::Debug;

use crate::{digital::blocking::OutputPin, spi::ErrorType};

use super::{Error, ErrorKind};

/// SPI device trait
///
/// `SpiDevice` represents ownership over a single SPI device on a (possibly shared) bus, selected
/// with a CS (Chip Select) pin.
///
/// See the [module-level documentation](self) for important usage information.
pub trait SpiDevice: ErrorType {
    /// SPI Bus type for this device.
    type Bus: ErrorType;

    /// Perform a transaction against the device.
    ///
    /// - Locks the bus
    /// - Asserts the CS (Chip Select) pin.
    /// - Calls `f` with an exclusive reference to the bus, which can then be used to do transfers against the device.
    /// - [Flushes](SpiBusFlush::flush) the bus.
    /// - Deasserts the CS pin.
    /// - Unlocks the bus.
    ///
    /// The locking mechanism is implementation-defined. The only requirement is it must prevent two
    /// transactions from executing concurrently against the same bus. Examples of implementations are:
    /// critical sections, blocking mutexes, returning an error or panicking if the bus is already busy.
    fn transaction<R>(
        &mut self,
        f: impl FnOnce(&mut Self::Bus) -> Result<R, <Self::Bus as ErrorType>::Error>,
    ) -> Result<R, Self::Error>;

    /// Do a write within a transaction.
    ///
    /// This is a convenience method equivalent to `device.transaction(|bus| bus.write(buf))`.
    ///
    /// See also: [`SpiDevice::transaction`], [`SpiBusWrite::write`]
    fn write<Word>(&mut self, buf: &[Word]) -> Result<(), Self::Error>
    where
        Self::Bus: SpiBusWrite<Word>,
        Word: Copy,
    {
        self.transaction(|bus| bus.write(buf))
    }

    /// Do a read within a transaction.
    ///
    /// This is a convenience method equivalent to `device.transaction(|bus| bus.read(buf))`.
    ///
    /// See also: [`SpiDevice::transaction`], [`SpiBusRead::read`]
    fn read<Word>(&mut self, buf: &mut [Word]) -> Result<(), Self::Error>
    where
        Self::Bus: SpiBusRead<Word>,
        Word: Copy,
    {
        self.transaction(|bus| bus.read(buf))
    }

    /// Do a transfer within a transaction.
    ///
    /// This is a convenience method equivalent to `device.transaction(|bus| bus.transfer(read, write))`.
    ///
    /// See also: [`SpiDevice::transaction`], [`SpiBus::transfer`]
    fn transfer<Word>(&mut self, read: &mut [Word], write: &[Word]) -> Result<(), Self::Error>
    where
        Self::Bus: SpiBus<Word>,
        Word: Copy,
    {
        self.transaction(|bus| bus.transfer(read, write))
    }

    /// Do an in-place transfer within a transaction.
    ///
    /// This is a convenience method equivalent to `device.transaction(|bus| bus.transfer_in_place(buf))`.
    ///
    /// See also: [`SpiDevice::transaction`], [`SpiBus::transfer_in_place`]
    fn transfer_in_place<Word>(&mut self, buf: &mut [Word]) -> Result<(), Self::Error>
    where
        Self::Bus: SpiBus<Word>,
        Word: Copy,
    {
        self.transaction(|bus| bus.transfer_in_place(buf))
    }
}

impl<T: SpiDevice> SpiDevice for &mut T {
    type Bus = T::Bus;
    fn transaction<R>(
        &mut self,
        f: impl FnOnce(&mut Self::Bus) -> Result<R, <Self::Bus as ErrorType>::Error>,
    ) -> Result<R, Self::Error> {
        T::transaction(self, f)
    }
}

/// Flush support for SPI bus
pub trait SpiBusFlush: ErrorType {
    /// Wait until all operations have completed and the bus is idle.
    ///
    /// See the [module-level documentation](self) for important usage information.
    fn flush(&mut self) -> Result<(), Self::Error>;
}

impl<T: SpiBusFlush> SpiBusFlush for &mut T {
    fn flush(&mut self) -> Result<(), Self::Error> {
        T::flush(self)
    }
}

/// Read-only SPI bus
pub trait SpiBusRead<Word: Copy = u8>: SpiBusFlush {
    /// Read `words` from the slave.
    ///
    /// The word value sent on MOSI during reading is implementation-defined,
    /// typically `0x00`, `0xFF`, or configurable.
    ///
    /// Implementations are allowed to return before the operation is
    /// complete. See the [module-level documentation](self) for details.
    fn read(&mut self, words: &mut [Word]) -> Result<(), Self::Error>;
}

impl<T: SpiBusRead<Word>, Word: Copy> SpiBusRead<Word> for &mut T {
    fn read(&mut self, words: &mut [Word]) -> Result<(), Self::Error> {
        T::read(self, words)
    }
}

/// Write-only SPI bus
pub trait SpiBusWrite<Word: Copy = u8>: SpiBusFlush {
    /// Write `words` to the slave, ignoring all the incoming words
    ///
    /// Implementations are allowed to return before the operation is
    /// complete. See the [module-level documentation](self) for details.
    fn write(&mut self, words: &[Word]) -> Result<(), Self::Error>;
}

impl<T: SpiBusWrite<Word>, Word: Copy> SpiBusWrite<Word> for &mut T {
    fn write(&mut self, words: &[Word]) -> Result<(), Self::Error> {
        T::write(self, words)
    }
}

/// Read-write SPI bus
///
/// `SpiBus` represents **exclusive ownership** over the whole SPI bus, with SCK, MOSI and MISO pins.
///
/// See the [module-level documentation](self) for important information on SPI Bus vs Device traits.
pub trait SpiBus<Word: Copy = u8>: SpiBusRead<Word> + SpiBusWrite<Word> {
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
}

impl<T: SpiBus<Word>, Word: Copy> SpiBus<Word> for &mut T {
    fn transfer(&mut self, read: &mut [Word], write: &[Word]) -> Result<(), Self::Error> {
        T::transfer(self, read, write)
    }

    fn transfer_in_place(&mut self, words: &mut [Word]) -> Result<(), Self::Error> {
        T::transfer_in_place(self, words)
    }
}

/// Error type for [`ExclusiveDevice`] operations.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ExclusiveDeviceError<BUS, CS> {
    /// An inner SPI bus operation failed
    Spi(BUS),
    /// Asserting or deasserting CS failed
    Cs(CS),
}

impl<BUS, CS> Error for ExclusiveDeviceError<BUS, CS>
where
    BUS: Error + Debug,
    CS: Debug,
{
    fn kind(&self) -> ErrorKind {
        match self {
            Self::Spi(e) => e.kind(),
            Self::Cs(_) => ErrorKind::ChipSelectFault,
        }
    }
}

/// [`SpiDevice`] implementation with exclusive access to the bus (not shared).
///
/// This is the most straightforward way of obtaining an [`SpiDevice`] from an [`SpiBus`],
/// ideal for when no sharing is required (only one SPI device is present on the bus).
pub struct ExclusiveDevice<BUS, CS> {
    bus: BUS,
    cs: CS,
}

impl<BUS, CS> ExclusiveDevice<BUS, CS> {
    /// Create a new ExclusiveDevice
    pub fn new(bus: BUS, cs: CS) -> Self {
        Self { bus, cs }
    }
}

impl<BUS, CS> ErrorType for ExclusiveDevice<BUS, CS>
where
    BUS: ErrorType,
    CS: OutputPin,
{
    type Error = ExclusiveDeviceError<BUS::Error, CS::Error>;
}

impl<BUS, CS> SpiDevice for ExclusiveDevice<BUS, CS>
where
    BUS: SpiBusFlush,
    CS: OutputPin,
{
    type Bus = BUS;

    fn transaction<R>(
        &mut self,
        f: impl FnOnce(&mut Self::Bus) -> Result<R, <Self::Bus as ErrorType>::Error>,
    ) -> Result<R, Self::Error> {
        self.cs.set_low().map_err(ExclusiveDeviceError::Cs)?;

        let f_res = f(&mut self.bus);

        // On failure, it's important to still flush and deassert CS.
        let flush_res = self.bus.flush();
        let cs_res = self.cs.set_high();

        let f_res = f_res.map_err(ExclusiveDeviceError::Spi)?;
        flush_res.map_err(ExclusiveDeviceError::Spi)?;
        cs_res.map_err(ExclusiveDeviceError::Cs)?;

        Ok(f_res)
    }
}
