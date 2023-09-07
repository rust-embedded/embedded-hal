//! SPI master mode traits.

pub use embedded_hal::spi::{
    Error, ErrorKind, ErrorType, Mode, Operation, Phase, Polarity, MODE_0, MODE_1, MODE_2, MODE_3,
};

/// SPI device trait.
///
/// `SpiDevice` represents ownership over a single SPI device on a (possibly shared) bus, selected
/// with a CS (Chip Select) pin.
///
/// See [the docs on embedded-hal](embedded_hal::spi) for important information on SPI Bus vs Device traits.
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
    async fn transaction(
        &mut self,
        operations: &mut [Operation<'_, Word>],
    ) -> Result<(), Self::Error>;

    /// Do a read within a transaction.
    ///
    /// This is a convenience method equivalent to `device.read_transaction(&mut [buf])`.
    ///
    /// See also: [`SpiDevice::transaction`], [`SpiDevice::read`]
    #[inline]
    async fn read(&mut self, buf: &mut [Word]) -> Result<(), Self::Error> {
        self.transaction(&mut [Operation::Read(buf)]).await
    }

    /// Do a write within a transaction.
    ///
    /// This is a convenience method equivalent to `device.write_transaction(&mut [buf])`.
    ///
    /// See also: [`SpiDevice::transaction`], [`SpiDevice::write`]
    #[inline]
    async fn write(&mut self, buf: &[Word]) -> Result<(), Self::Error> {
        self.transaction(&mut [Operation::Write(buf)]).await
    }

    /// Do a transfer within a transaction.
    ///
    /// This is a convenience method equivalent to `device.transaction(|bus| bus.transfer(read, write))`.
    ///
    /// See also: [`SpiDevice::transaction`], [`SpiBus::transfer`]
    #[inline]
    async fn transfer(&mut self, read: &mut [Word], write: &[Word]) -> Result<(), Self::Error> {
        self.transaction(&mut [Operation::Transfer(read, write)])
            .await
    }

    /// Do an in-place transfer within a transaction.
    ///
    /// This is a convenience method equivalent to `device.transaction(|bus| bus.transfer_in_place(buf))`.
    ///
    /// See also: [`SpiDevice::transaction`], [`SpiBus::transfer_in_place`]
    #[inline]
    async fn transfer_in_place(&mut self, buf: &mut [Word]) -> Result<(), Self::Error> {
        self.transaction(&mut [Operation::TransferInPlace(buf)])
            .await
    }
}

impl<Word: Copy + 'static, T: SpiDevice<Word> + ?Sized> SpiDevice<Word> for &mut T {
    #[inline]
    async fn transaction(
        &mut self,
        operations: &mut [Operation<'_, Word>],
    ) -> Result<(), T::Error> {
        T::transaction(self, operations).await
    }

    #[inline]
    async fn read(&mut self, buf: &mut [Word]) -> Result<(), T::Error> {
        T::read(self, buf).await
    }

    #[inline]
    async fn write(&mut self, buf: &[Word]) -> Result<(), T::Error> {
        T::write(self, buf).await
    }

    #[inline]
    async fn transfer(&mut self, read: &mut [Word], write: &[Word]) -> Result<(), T::Error> {
        T::transfer(self, read, write).await
    }

    #[inline]
    async fn transfer_in_place(&mut self, buf: &mut [Word]) -> Result<(), T::Error> {
        T::transfer_in_place(self, buf).await
    }
}

/// SPI bus.
///
/// `SpiBus` represents **exclusive ownership** over the whole SPI bus, with SCK, MOSI and MISO pins.
///
/// See [the docs on embedded-hal][embedded_hal::spi] for important information on SPI Bus vs Device traits.
pub trait SpiBus<Word: 'static + Copy = u8>: ErrorType {
    /// Read `words` from the slave.
    ///
    /// The word value sent on MOSI during reading is implementation-defined,
    /// typically `0x00`, `0xFF`, or configurable.
    ///
    /// Implementations are allowed to return before the operation is
    /// complete. See [the docs on embedded-hal][embedded_hal::spi] for details on flushing.
    async fn read(&mut self, words: &mut [Word]) -> Result<(), Self::Error>;

    /// Write `words` to the slave, ignoring all the incoming words.
    ///
    /// Implementations are allowed to return before the operation is
    /// complete. See [the docs on embedded-hal][embedded_hal::spi] for details on flushing.
    async fn write(&mut self, words: &[Word]) -> Result<(), Self::Error>;

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
    /// complete. See [the docs on embedded-hal][embedded_hal::spi] for details on flushing.
    async fn transfer(&mut self, read: &mut [Word], write: &[Word]) -> Result<(), Self::Error>;

    /// Write and read simultaneously. The contents of `words` are
    /// written to the slave, and the received words are stored into the same
    /// `words` buffer, overwriting it.
    ///
    /// Implementations are allowed to return before the operation is
    /// complete. See [the docs on embedded-hal][embedded_hal::spi] for details on flushing.
    async fn transfer_in_place(&mut self, words: &mut [Word]) -> Result<(), Self::Error>;

    /// Wait until all operations have completed and the bus is idle.
    ///
    /// See [the docs on embedded-hal][embedded_hal::spi] for information on flushing.
    async fn flush(&mut self) -> Result<(), Self::Error>;
}

impl<T: SpiBus<Word> + ?Sized, Word: 'static + Copy> SpiBus<Word> for &mut T {
    #[inline]
    async fn read(&mut self, words: &mut [Word]) -> Result<(), T::Error> {
        T::read(self, words).await
    }

    #[inline]
    async fn write(&mut self, words: &[Word]) -> Result<(), T::Error> {
        T::write(self, words).await
    }

    #[inline]
    async fn transfer(&mut self, read: &mut [Word], write: &[Word]) -> Result<(), T::Error> {
        T::transfer(self, read, write).await
    }

    #[inline]
    async fn transfer_in_place(&mut self, words: &mut [Word]) -> Result<(), T::Error> {
        T::transfer_in_place(self, words).await
    }

    #[inline]
    async fn flush(&mut self) -> Result<(), T::Error> {
        T::flush(self).await
    }
}
