//! Blocking SPI API

use super::ErrorType;

/// Blocking transfer with separate buffers
pub trait Transfer<Word = u8>: ErrorType {
    /// Writes and reads simultaneously. `write` is written to the slave on MOSI and
    /// words received on MISO are stored in `read`.
    ///
    /// It is allowed for `read` and `write` to have different lengths, even zero length.
    /// The transfer runs for `max(read.len(), write.len())` words. If `read` is shorter,
    /// incoming words after `read` has been filled will be discarded. If `write` is shorter,
    /// the value of words sent in MOSI after all `write` has been sent is implementation-defined,
    /// typically `0x00`, `0xFF`, or configurable.
    fn transfer(&mut self, read: &mut [Word], write: &[Word]) -> Result<(), Self::Error>;
}

#[cfg(conflicting)]
impl<T: Transfer<Word>, Word: Copy> Transfer<Word> for &mut T {
    fn transfer(&mut self, read: &mut [Word], write: &[Word]) -> Result<(), Self::Error> {
        T::transfer(self, read, write)
    }
}

/// Blocking transfer with single buffer (in-place)
pub trait TransferInplace<Word: Copy = u8>: ErrorType {
    /// Writes and reads simultaneously. The contents of `words` are
    /// written to the slave, and the received words are stored into the same
    /// `words` buffer, overwriting it.
    fn transfer_inplace(&mut self, words: &mut [Word]) -> Result<(), Self::Error>;
}

#[cfg(conflicting)]
impl<T: TransferInplace<Word>, Word: Copy> TransferInplace<Word> for &mut T {
    fn transfer_inplace(&mut self, words: &mut [Word]) -> Result<(), Self::Error> {
        T::transfer_inplace(self, words)
    }
}

/// Blocking read
pub trait Read<Word: Copy = u8>: ErrorType {
    /// Reads `words` from the slave.
    ///
    /// The word value sent on MOSI during reading is implementation-defined,
    /// typically `0x00`, `0xFF`, or configurable.
    fn read(&mut self, words: &mut [Word]) -> Result<(), Self::Error>;
}

#[cfg(conflicting)]
impl<T: Read<Word>, Word: Copy> Read<Word> for &mut T {
    fn read(&mut self, words: &mut [Word]) -> Result<(), Self::Error> {
        T::read(self, words)
    }
}

/// Blocking write
pub trait Write<Word: Copy = u8>: ErrorType {
    /// Writes `words` to the slave, ignoring all the incoming words
    fn write(&mut self, words: &[Word]) -> Result<(), Self::Error>;
}

#[cfg(conflicting)]
impl<T: Write<Word>, Word: Copy> Write<Word> for &mut T {
    fn write(&mut self, words: &[Word]) -> Result<(), Self::Error> {
        T::write(self, words)
    }
}

/// Blocking write (iterator version)
pub trait WriteIter<Word: Copy = u8>: ErrorType {
    /// Writes `words` to the slave, ignoring all the incoming words
    fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
    where
        WI: IntoIterator<Item = Word>;
}

impl<T: WriteIter<Word>, Word: Copy> WriteIter<Word> for &mut T {
    fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
    where
        WI: IntoIterator<Item = Word>,
    {
        T::write_iter(self, words)
    }
}

/// Operation for transactional SPI trait
///
/// This allows composition of SPI operations into a single bus transaction
#[derive(Debug, PartialEq)]
pub enum Operation<'a, Word: 'static + Copy = u8> {
    /// Read data into the provided buffer.
    Read(&'a mut [Word]),
    /// Write data from the provided buffer, discarding read data
    Write(&'a [Word]),
    /// Write data out while reading data into the provided buffer
    Transfer(&'a mut [Word], &'a [Word]),
    /// Write data out while reading data into the provided buffer
    TransferInplace(&'a mut [Word]),
}

/// Transactional trait allows multiple actions to be executed
/// as part of a single SPI transaction
pub trait Transactional<Word: 'static + Copy = u8>: ErrorType {
    /// Execute the provided transactions
    fn exec<'a>(&mut self, operations: &mut [Operation<'a, Word>]) -> Result<(), Self::Error>;
}

#[cfg(conflicting)]
impl<T: Transactional<Word>, Word: 'static + Copy> Transactional<Word> for &mut T {
    fn exec<'a>(&mut self, operations: &mut [Operation<'a, Word>]) -> Result<(), Self::Error> {
        T::exec(self, operations)
    }
}

/// SPI Managed CS trait
///
/// This uses a bunch of magic to manage CS for all SPI methods, and conflicts with the `&mut T` impls for each trait.
///
/// ```
/// use embedded_hal::spi::blocking::{ManagedCs, Write};
///
/// // Automatic CS assertion
/// fn spi_write_auto_cs<SPI: ManagedCs + Write>(spi: &mut SPI) {
///   let _ = spi.write(&[0xaa, 0xbb, 0xcc]);
/// }
/// // Manual CS assertion
/// fn spi_write_manual_cs<SPI: ManagedCs<Inner=P>, P: Write>(spi: &mut SPI) {
///   let _ = spi.with_cs(|d|{
///     let _ = d.write(&[0xaa, 0xbb, 0xcc]);
///     Ok(())
///   });
/// }
/// ```
pub trait ManagedCs: ErrorType {
    /// Inner SPI type
    type Inner: ErrorType;

    /// Execute the provided closure within a CS assertion
    fn with_cs<F: FnMut(&mut Self::Inner) -> Result<(), Self::Error>>(
        &mut self,
        f: F,
    ) -> Result<(), Self::Error>;

    /// Reference the inner SPI object without manipulating CS
    fn inner(&mut self) -> &mut Self::Inner;
}

/// Alternate managed CS trait
///
/// This one doesn't require any defaults / provide any magic, if you want
/// to do things within a CS assertion you call `with_cs`.
///
/// ```
/// use embedded_hal::spi::blocking::{ManagedCsAlt, Write};
///
/// // Manual CS assertion
/// fn spi_write_manual_cs<SPI: ManagedCsAlt + Write>(spi: &mut SPI) {
///   let _ = spi.with_cs(|d|{
///     let _ = d.write(&[0xaa, 0xbb, 0xcc]);
///     Ok(())
///   });
/// }
/// ```
pub trait ManagedCsAlt: ErrorType {
    /// Execute the provided closure within a CS assertion
    fn with_cs<F: FnMut(&mut Self) -> Result<(), Self::Error>>(
        &mut self,
        f: F,
    ) -> Result<(), Self::Error>;
}

/// These default conflict with the &mut impls, we could just not have them / require folks to always call [`ManagedCs::with_cs`]?
mod defaults {
    use super::*;

    /// Default blocking [`Transfer`] with CS management
    impl<T, I, E, Word> Transfer<Word> for T
    where
        T: ManagedCs<Inner = I> + ErrorType<Error = E>,
        I: Transfer<Word> + ErrorType<Error = E>,
        Word: Copy + 'static,
    {
        fn transfer<'a>(&mut self, read: &mut [Word], write: &[Word]) -> Result<(), Self::Error> {
            self.with_cs(|i: &mut I| i.transfer(read, write))
        }
    }

    /// Default blocking [`TransferInplace`] with CS management
    impl<T, I, E, Word> TransferInplace<Word> for T
    where
        T: ManagedCs<Inner = I> + ErrorType<Error = E>,
        I: TransferInplace<Word> + ErrorType<Error = E>,
        Word: Copy + 'static,
    {
        fn transfer_inplace<'a>(&mut self, words: &mut [Word]) -> Result<(), Self::Error> {
            self.with_cs(|i: &mut I| i.transfer_inplace(words))
        }
    }

    /// Default blocking [`Read`] with CS management
    impl<T, I, E, Word> Read<Word> for T
    where
        T: ManagedCs<Inner = I> + ErrorType<Error = E>,
        I: Read<Word> + ErrorType<Error = E>,
        Word: Copy + 'static,
    {
        fn read<'a>(&mut self, words: &mut [Word]) -> Result<(), Self::Error> {
            self.with_cs(|i: &mut I| i.read(words))
        }
    }

    /// Default blocking [`Write`] with CS management
    impl<T, I, E, Word> Write<Word> for T
    where
        T: ManagedCs<Inner = I> + ErrorType<Error = E>,
        I: Write<Word> + ErrorType<Error = E>,
        Word: Copy + 'static,
    {
        fn write<'a>(&mut self, words: &[Word]) -> Result<(), Self::Error> {
            self.with_cs(|i: &mut I| i.write(words))
        }
    }

    /// Default blocking [`Transactional`] with CS management
    impl<T, I, E, Word> Transactional<Word> for T
    where
        T: ManagedCs<Inner = I> + ErrorType<Error = E>,
        I: Transactional<Word> + ErrorType<Error = E>,
        Word: Copy + 'static,
    {
        fn exec<'a>(&mut self, operations: &mut [Operation<'a, Word>]) -> Result<(), Self::Error> {
            self.with_cs(|i: &mut I| i.exec(operations))
        }
    }
}

/// [`SpiWithCs`] wraps an SPI implementation with Chip Select (CS)
/// pin management for exclusive (non-shared) use.
/// For sharing SPI between peripherals, see [shared-bus](https://crates.io/crates/shared-bus)
pub struct SpiWithCs<Spi, Pin> {
    spi: Spi,
    cs: Pin,
}

/// Wrapper for errors returned by [`SpiWithCs`]
#[derive(Clone, Debug, PartialEq)]
pub enum SpiWithCsError<SpiError, PinError> {
    /// Underlying SPI communication error
    Spi(SpiError),
    /// Underlying chip-select pin state setting error
    Pin(PinError),
}

/// [`ErrorType`] implementation for [`SpiWithCs`] wrapper
impl<Spi, Pin> ErrorType for SpiWithCs<Spi, Pin>
where
    Spi: ErrorType,
    Pin: crate::digital::blocking::OutputPin + crate::digital::ErrorType,
{
    type Error =
        SpiWithCsError<<Spi as ErrorType>::Error, <Pin as crate::digital::ErrorType>::Error>;
}

/// [`ManagedCs`] implementation for [`SpiWithCs`] wrapper.
/// Provides `with_cs` function that asserts and deasserts CS
impl<Spi, Pin> ManagedCs for SpiWithCs<Spi, Pin>
where
    Spi: ErrorType,
    Pin: crate::digital::blocking::OutputPin + crate::digital::ErrorType,
{
    type Inner = Spi;

    /// Executes the provided closure within a CS assertion
    fn with_cs<F: FnMut(&mut Self::Inner) -> Result<(), Self::Error>>(
        &mut self,
        mut f: F,
    ) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(SpiWithCsError::Pin)?;

        let r = f(&mut self.spi);

        self.cs.set_high().map_err(SpiWithCsError::Pin)?;

        r
    }

    /// Reference the inner SPI object without manipulating CS
    fn inner(&mut self) -> &mut Self::Inner {
        &mut self.spi
    }
}

/// [`ManagedCs`] implementation for [`SpiWithCs`] wrapper.
/// Provides `with_cs` function that asserts and deasserts CS
impl<Spi, Pin> ManagedCsAlt for SpiWithCs<Spi, Pin>
where
    Spi: ErrorType,
    Pin: crate::digital::blocking::OutputPin + crate::digital::ErrorType,
{
    /// Executes the provided closure within a CS assertion
    fn with_cs<F: FnMut(&mut Self) -> Result<(), Self::Error>>(
        &mut self,
        mut f: F,
    ) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(SpiWithCsError::Pin)?;

        let r = f(self);

        self.cs.set_high().map_err(SpiWithCsError::Pin)?;

        r
    }
}

/// [`super::Error`] implementation for [`SpiWithCsError`]
impl<SpiError: super::Error + core::fmt::Debug, PinError: core::fmt::Debug> super::Error
    for SpiWithCsError<SpiError, PinError>
{
    fn kind(&self) -> super::ErrorKind {
        match self {
            SpiWithCsError::Spi(spi) => spi.kind(),
            SpiWithCsError::Pin(_pin) => super::ErrorKind::Other,
        }
    }
}
