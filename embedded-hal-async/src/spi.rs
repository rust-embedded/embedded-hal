//! SPI master mode traits.

use core::{fmt::Debug, future::Future};

use embedded_hal::digital::OutputPin;
use embedded_hal::spi as blocking;
pub use embedded_hal::spi::{
    Error, ErrorKind, ErrorType, Mode, Phase, Polarity, MODE_0, MODE_1, MODE_2, MODE_3,
};

type ReadFuture<'a, T, Word>
where
    T: SpiDevice + ?Sized + 'a,
    T::Bus: SpiBusRead<Word>,
    Word: Copy + 'static,
= impl Future<Output = Result<(), T::Error>>;

type WriteFuture<'a, T, Word>
where
    T: SpiDevice + ?Sized + 'a,
    T::Bus: SpiBusWrite<Word>,
    Word: Copy + 'static,
= impl Future<Output = Result<(), T::Error>>;

type TransferFuture<'a, T, Word>
where
    T: SpiDevice + ?Sized + 'a,
    T::Bus: SpiBus<Word>,
    Word: Copy + 'static,
= impl Future<Output = Result<(), T::Error>>;

type TransferInPlaceFuture<'a, T, Word>
where
    T: SpiDevice + ?Sized + 'a,
    T::Bus: SpiBus<Word>,
    Word: Copy + 'static,
= impl Future<Output = Result<(), T::Error>>;

#[macro_export]
/// Do an SPI transaction on a bus.
/// This is a safe wrapper for [SpiDevice::transaction], which handles dereferencing the raw pointer for you.
///
/// # Examples
///
/// ```
/// use embedded_hal_async::spi::{transaction, SpiBus, SpiBusRead, SpiBusWrite, SpiDevice};
///
/// pub async fn transaction_example<SPI>(mut device: SPI) -> Result<u32, SPI::Error>
/// where
///     SPI: SpiDevice,
///     SPI::Bus: SpiBus,
/// {
///     transaction!(&mut device, move |bus| async move {
///         // Unlike `SpiDevice::transaction`, we don't need to
///         // manually dereference a pointer in order to use the bus.
///         bus.write(&[42]).await?;
///         let mut data = [0; 4];
///         bus.read(&mut data).await?;
///         Ok(u32::from_be_bytes(data))
///     })
///     .await
/// }
/// ```
///
/// Note that the compiler will prevent you from moving the bus reference outside of the closure
/// ```compile_fail
/// # use embedded_hal_async::spi::{transaction, SpiBus, SpiBusRead, SpiBusWrite, SpiDevice};
/// #
/// # pub async fn smuggle_test<SPI>(mut device: SPI) -> Result<(), SPI::Error>
/// # where
/// #     SPI: SpiDevice,
/// #     SPI::Bus: SpiBus,
/// # {
///     let mut bus_smuggler: Option<&mut SPI::Bus> = None;
///     transaction!(&mut device, move |bus| async move {
///         bus_smuggler = Some(bus);
///         Ok(())
///     })
///     .await
/// # }
/// ```
macro_rules! spi_transaction {
    ($device:expr, move |$bus:ident| async move $closure_body:expr) => {
        $crate::spi::SpiDevice::transaction($device, move |$bus| {
            // Safety: Implementers of the `SpiDevice` trait guarantee that the pointer is
            // valid and dereferencable for the entire duration of the closure.
            let $bus = unsafe { &mut *$bus };
            async move {
                let result = $closure_body;
                let $bus = $bus; // Ensure that the bus reference was not moved out of the closure
                let _ = $bus; // Silence the "unused variable" warning from prevous line
                result
            }
        })
    };
    ($device:expr, move |$bus:ident| async $closure_body:expr) => {
        $crate::spi::SpiDevice::transaction($device, move |$bus| {
            // Safety: Implementers of the `SpiDevice` trait guarantee that the pointer is
            // valid and dereferencable for the entire duration of the closure.
            let $bus = unsafe { &mut *$bus };
            async {
                let result = $closure_body;
                let $bus = $bus; // Ensure that the bus reference was not moved out of the closure
                let _ = $bus; // Silence the "unused variable" warning from prevous line
                result
            }
        })
    };
    ($device:expr, |$bus:ident| async move $closure_body:expr) => {
        $crate::spi::SpiDevice::transaction($device, |$bus| {
            // Safety: Implementers of the `SpiDevice` trait guarantee that the pointer is
            // valid and dereferencable for the entire duration of the closure.
            let $bus = unsafe { &mut *$bus };
            async move {
                let result = $closure_body;
                let $bus = $bus; // Ensure that the bus reference was not moved out of the closure
                let _ = $bus; // Silence the "unused variable" warning from prevous line
                result
            }
        })
    };
    ($device:expr, |$bus:ident| async $closure_body:expr) => {
        $crate::spi::SpiDevice::transaction($device, |$bus| {
            // Safety: Implementers of the `SpiDevice` trait guarantee that the pointer is
            // valid and dereferencable for the entire duration of the closure.
            let $bus = unsafe { &mut *$bus };
            async {
                let result = $closure_body;
                let $bus = $bus; // Ensure that the bus reference was not moved out of the closure
                let _ = $bus; // Silence the "unused variable" warning from prevous line
                result
            }
        })
    };
}

#[doc(inline)]
pub use spi_transaction as transaction;

/// SPI device trait
///
/// `SpiDevice` represents ownership over a single SPI device on a (possibly shared) bus, selected
/// with a CS (Chip Select) pin.
///
/// See (the docs on embedded-hal)[embedded_hal::spi::blocking] for important information on SPI Bus vs Device traits.
///
/// # Safety
///
/// See [`SpiDevice::transaction`] for details.
pub unsafe trait SpiDevice: ErrorType {
    /// SPI Bus type for this device.
    type Bus: ErrorType;

    /// Future returned by the `transaction` method.
    type TransactionFuture<'a, R, F, Fut>: Future<Output = Result<R, Self::Error>>
    where
        Self: 'a,
        R: 'a,
        F: FnOnce(*mut Self::Bus) -> Fut + 'a,
        Fut: Future<Output = Result<R, <Self::Bus as ErrorType>::Error>> + 'a;

    /// Perform a transaction against the device.
    ///
    /// **NOTE:**
    /// It is not recommended to use this method directly, because it requires `unsafe` code to dereference the raw pointer.
    /// Instead, the [`transaction!`] macro should be used, which handles this safely inside the macro.
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
    /// critical sections, blocking mutexes, async mutexes, returning an error or panicking if the bus is already busy.
    ///
    /// On bus errors the implementation should try to deassert CS.
    /// If an error occurs while deasserting CS the bus error should take priority as the return value.
    ///
    /// # Safety
    ///
    /// The current state of the Rust typechecker doesn't allow expressing the necessary lifetime constraints, so
    /// the `f` closure receives a lifetime-less `*mut Bus` raw pointer instead.
    ///
    /// Implementers of the `SpiDevice` trait must guarantee that the pointer is valid and dereferencable
    /// for the entire duration of the closure.
    fn transaction<'a, R, F, Fut>(&'a mut self, f: F) -> Self::TransactionFuture<'a, R, F, Fut>
    where
        F: FnOnce(*mut Self::Bus) -> Fut + 'a,
        Fut: Future<Output = Result<R, <Self::Bus as ErrorType>::Error>> + 'a;

    /// Do a read within a transaction.
    ///
    /// This is a convenience method equivalent to `device.transaction(|bus| bus.read(buf))`.
    ///
    /// See also: [`SpiDevice::transaction`], [`SpiBusRead::read`]
    fn read<'a, Word>(&'a mut self, buf: &'a mut [Word]) -> ReadFuture<'a, Self, Word>
    where
        Self::Bus: SpiBusRead<Word>,
        Word: Copy + 'static,
    {
        transaction!(self, move |bus| async move { bus.read(buf).await })
    }

    /// Do a write within a transaction.
    ///
    /// This is a convenience method equivalent to `device.transaction(|bus| bus.write(buf))`.
    ///
    /// See also: [`SpiDevice::transaction`], [`SpiBusWrite::write`]
    fn write<'a, Word>(&'a mut self, buf: &'a [Word]) -> WriteFuture<'a, Self, Word>
    where
        Self::Bus: SpiBusWrite<Word>,
        Word: Copy + 'static,
    {
        transaction!(self, move |bus| async move { bus.write(buf).await })
    }

    /// Do a transfer within a transaction.
    ///
    /// This is a convenience method equivalent to `device.transaction(|bus| bus.transfer(read, write))`.
    ///
    /// See also: [`SpiDevice::transaction`], [`SpiBus::transfer`]
    fn transfer<'a, Word>(
        &'a mut self,
        read: &'a mut [Word],
        write: &'a [Word],
    ) -> TransferFuture<'a, Self, Word>
    where
        Self::Bus: SpiBus<Word>,
        Word: Copy + 'static,
    {
        transaction!(
            self,
            move |bus| async move { bus.transfer(read, write).await }
        )
    }

    /// Do an in-place transfer within a transaction.
    ///
    /// This is a convenience method equivalent to `device.transaction(|bus| bus.transfer_in_place(buf))`.
    ///
    /// See also: [`SpiDevice::transaction`], [`SpiBus::transfer_in_place`]
    fn transfer_in_place<'a, Word>(
        &'a mut self,
        buf: &'a mut [Word],
    ) -> TransferInPlaceFuture<'a, Self, Word>
    where
        Self::Bus: SpiBus<Word>,
        Word: Copy + 'static,
    {
        transaction!(
            self,
            move |bus| async move { bus.transfer_in_place(buf).await }
        )
    }
}

unsafe impl<T: SpiDevice> SpiDevice for &mut T {
    type Bus = T::Bus;

    type TransactionFuture<'a, R, F, Fut> = T::TransactionFuture<'a, R, F, Fut>
    where
        Self: 'a, R: 'a, F: FnOnce(*mut Self::Bus) -> Fut + 'a,
        Fut: Future<Output = Result<R, <Self::Bus as ErrorType>::Error>> + 'a;

    fn transaction<'a, R, F, Fut>(&'a mut self, f: F) -> Self::TransactionFuture<'a, R, F, Fut>
    where
        F: FnOnce(*mut Self::Bus) -> Fut + 'a,
        Fut: Future<Output = Result<R, <Self::Bus as ErrorType>::Error>> + 'a,
    {
        T::transaction(self, f)
    }
}

/// Flush support for SPI bus
pub trait SpiBusFlush: ErrorType {
    /// Future returned by the `flush` method.
    type FlushFuture<'a>: Future<Output = Result<(), Self::Error>>
    where
        Self: 'a;

    /// Wait until all operations have completed and the bus is idle.
    ///
    /// See (the docs on embedded-hal)[embedded_hal::spi::blocking] for information on flushing.
    fn flush(&mut self) -> Self::FlushFuture<'_>;
}

impl<T: SpiBusFlush> SpiBusFlush for &mut T {
    type FlushFuture<'a> = T::FlushFuture<'a> where Self: 'a;

    fn flush(&mut self) -> Self::FlushFuture<'_> {
        T::flush(self)
    }
}

/// Read-only SPI bus
pub trait SpiBusRead<Word: 'static + Copy = u8>: SpiBusFlush {
    /// Future returned by the `read` method.
    type ReadFuture<'a>: Future<Output = Result<(), Self::Error>>
    where
        Self: 'a;

    /// Read `words` from the slave.
    ///
    /// The word value sent on MOSI during reading is implementation-defined,
    /// typically `0x00`, `0xFF`, or configurable.
    ///
    /// Implementations are allowed to return before the operation is
    /// complete. See (the docs on embedded-hal)[embedded_hal::spi::blocking] for details on flushing.
    fn read<'a>(&'a mut self, words: &'a mut [Word]) -> Self::ReadFuture<'a>;
}

impl<T: SpiBusRead<Word>, Word: 'static + Copy> SpiBusRead<Word> for &mut T {
    type ReadFuture<'a> = T::ReadFuture<'a> where Self: 'a;

    fn read<'a>(&'a mut self, words: &'a mut [Word]) -> Self::ReadFuture<'a> {
        T::read(self, words)
    }
}

/// Write-only SPI
pub trait SpiBusWrite<Word: 'static + Copy = u8>: SpiBusFlush {
    /// Future returned by the `write` method.
    type WriteFuture<'a>: Future<Output = Result<(), Self::Error>>
    where
        Self: 'a;

    /// Write `words` to the slave, ignoring all the incoming words
    ///
    /// Implementations are allowed to return before the operation is
    /// complete. See (the docs on embedded-hal)[embedded_hal::spi::blocking] for details on flushing.
    fn write<'a>(&'a mut self, words: &'a [Word]) -> Self::WriteFuture<'a>;
}

impl<T: SpiBusWrite<Word>, Word: 'static + Copy> SpiBusWrite<Word> for &mut T {
    type WriteFuture<'a> = T::WriteFuture<'a> where Self: 'a;

    fn write<'a>(&'a mut self, words: &'a [Word]) -> Self::WriteFuture<'a> {
        T::write(self, words)
    }
}

/// Read-write SPI bus
///
/// `SpiBus` represents **exclusive ownership** over the whole SPI bus, with SCK, MOSI and MISO pins.
///
/// See (the docs on embedded-hal)[embedded_hal::spi::blocking] for important information on SPI Bus vs Device traits.
pub trait SpiBus<Word: 'static + Copy = u8>: SpiBusRead<Word> + SpiBusWrite<Word> {
    /// Future returned by the `transfer` method.
    type TransferFuture<'a>: Future<Output = Result<(), Self::Error>>
    where
        Self: 'a;

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
    /// complete. See (the docs on embedded-hal)[embedded_hal::spi::blocking] for details on flushing.
    fn transfer<'a>(
        &'a mut self,
        read: &'a mut [Word],
        write: &'a [Word],
    ) -> Self::TransferFuture<'a>;

    /// Future returned by the `transfer_in_place` method.
    type TransferInPlaceFuture<'a>: Future<Output = Result<(), Self::Error>>
    where
        Self: 'a;

    /// Write and read simultaneously. The contents of `words` are
    /// written to the slave, and the received words are stored into the same
    /// `words` buffer, overwriting it.
    ///
    /// Implementations are allowed to return before the operation is
    /// complete. See (the docs on embedded-hal)[embedded_hal::spi::blocking] for details on flushing.
    fn transfer_in_place<'a>(
        &'a mut self,
        words: &'a mut [Word],
    ) -> Self::TransferInPlaceFuture<'a>;
}

impl<T: SpiBus<Word>, Word: 'static + Copy> SpiBus<Word> for &mut T {
    type TransferFuture<'a> = T::TransferFuture<'a> where Self: 'a;

    fn transfer<'a>(
        &'a mut self,
        read: &'a mut [Word],
        write: &'a [Word],
    ) -> Self::TransferFuture<'a> {
        T::transfer(self, read, write)
    }

    type TransferInPlaceFuture<'a> = T::TransferInPlaceFuture<'a> where Self: 'a;

    fn transfer_in_place<'a>(
        &'a mut self,
        words: &'a mut [Word],
    ) -> Self::TransferInPlaceFuture<'a> {
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

impl<BUS, CS> blocking::SpiDevice for ExclusiveDevice<BUS, CS>
where
    BUS: blocking::SpiBusFlush,
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

unsafe impl<BUS, CS> SpiDevice for ExclusiveDevice<BUS, CS>
where
    BUS: SpiBusFlush,
    CS: OutputPin,
{
    type Bus = BUS;

    type TransactionFuture<'a, R, F, Fut> = impl Future<Output = Result<R, Self::Error>>
    where
        Self: 'a, R: 'a, F: FnOnce(*mut Self::Bus) -> Fut + 'a,
        Fut: Future<Output =  Result<R, <Self::Bus as ErrorType>::Error>> + 'a;

    fn transaction<'a, R, F, Fut>(&'a mut self, f: F) -> Self::TransactionFuture<'a, R, F, Fut>
    where
        R: 'a,
        F: FnOnce(*mut Self::Bus) -> Fut + 'a,
        Fut: Future<Output = Result<R, <Self::Bus as ErrorType>::Error>> + 'a,
    {
        async move {
            self.cs.set_low().map_err(ExclusiveDeviceError::Cs)?;

            let f_res = f(&mut self.bus).await;

            // On failure, it's important to still flush and deassert CS.
            let flush_res = self.bus.flush().await;
            let cs_res = self.cs.set_high();

            let f_res = f_res.map_err(ExclusiveDeviceError::Spi)?;
            flush_res.map_err(ExclusiveDeviceError::Spi)?;
            cs_res.map_err(ExclusiveDeviceError::Cs)?;

            Ok(f_res)
        }
    }
}
