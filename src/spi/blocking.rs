//! Blocking SPI API
/// Blocking transfer
pub trait Transfer<W> {
    /// Error type
    type Error: crate::spi::Error;

    /// Writes and reads simultaneously. The contents of `words` are
    /// written to the slave, and the received words are stored into the same
    /// `words` buffer, overwriting it.
    fn transfer(&mut self, words: &mut [W]) -> Result<(), Self::Error>;
}

impl<T: Transfer<W>, W> Transfer<W> for &mut T {
    type Error = T::Error;

    fn transfer(&mut self, words: &mut [W]) -> Result<(), Self::Error> {
        T::transfer(self, words)
    }
}

/// Blocking write
pub trait Write<W> {
    /// Error type
    type Error: crate::spi::Error;

    /// Writes `words` to the slave, ignoring all the incoming words
    fn write(&mut self, words: &[W]) -> Result<(), Self::Error>;
}

impl<T: Write<W>, W> Write<W> for &mut T {
    type Error = T::Error;

    fn write(&mut self, words: &[W]) -> Result<(), Self::Error> {
        T::write(self, words)
    }
}

/// Blocking write (iterator version)
pub trait WriteIter<W> {
    /// Error type
    type Error: crate::spi::Error;

    /// Writes `words` to the slave, ignoring all the incoming words
    fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
    where
        WI: IntoIterator<Item = W>;
}

impl<T: WriteIter<W>, W> WriteIter<W> for &mut T {
    type Error = T::Error;

    fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
    where
        WI: IntoIterator<Item = W>,
    {
        T::write_iter(self, words)
    }
}

/// Operation for transactional SPI trait
///
/// This allows composition of SPI operations into a single bus transaction
#[derive(Debug, PartialEq)]
pub enum Operation<'a, W: 'static> {
    /// Write data from the provided buffer, discarding read data
    Write(&'a [W]),
    /// Write data out while reading data into the provided buffer
    Transfer(&'a mut [W]),
}

/// Transactional trait allows multiple actions to be executed
/// as part of a single SPI transaction
pub trait Transactional<W: 'static> {
    /// Associated error type
    type Error: crate::spi::Error;

    /// Execute the provided transactions
    fn exec<'a>(&mut self, operations: &mut [Operation<'a, W>]) -> Result<(), Self::Error>;
}

impl<T: Transactional<W>, W: 'static> Transactional<W> for &mut T {
    type Error = T::Error;

    fn exec<'a>(&mut self, operations: &mut [Operation<'a, W>]) -> Result<(), Self::Error> {
        T::exec(self, operations)
    }
}

/// Provides SpiWithCS wrapper using for spi::* types combined with an OutputPin
pub use spi_with_cs::{SpiWithCs, SpiWithCsError};
mod spi_with_cs {

    use core::fmt::Debug;

    use super::{Transfer, Write, WriteIter};
    use crate::digital::blocking::OutputPin;
    use crate::spi::{ErrorKind, ManagedChipSelect};

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

    /// Implement [`crate::spi::Error`] for wrapped types
    impl<SpiError, PinError> crate::spi::Error for SpiWithCsError<SpiError, PinError>
    where
        SpiError: crate::spi::Error + Debug,
        PinError: Debug,
    {
        fn kind(&self) -> ErrorKind {
            match self {
                SpiWithCsError::Spi(e) => e.kind(),
                SpiWithCsError::Pin(_e) => ErrorKind::Other,
            }
        }
    }

    /// [`ManagedChipSelect`] marker trait indicates Chip Select (CS) pin management is automatic
    impl<Spi, Pin> ManagedChipSelect for SpiWithCs<Spi, Pin> {}

    impl<Spi, Pin> SpiWithCs<Spi, Pin>
    where
        Pin: OutputPin,
    {
        /// Create a new SpiWithCS wrapper with the provided Spi and Pin
        pub fn new(spi: Spi, cs: Pin) -> Self {
            Self { spi, cs }
        }

        /// Fetch references to the inner Spi and Pin types.
        /// Note that using these directly will violate the `ManagedChipSelect` constraint.
        pub fn inner(&mut self) -> (&mut Spi, &mut Pin) {
            (&mut self.spi, &mut self.cs)
        }

        /// Destroy the SpiWithCs wrapper, returning the bus and pin objects
        pub fn destroy(self) -> (Spi, Pin) {
            (self.spi, self.cs)
        }
    }

    impl<Spi, Pin, W> Transfer<W> for SpiWithCs<Spi, Pin>
    where
        Spi: Transfer<W>,
        Pin: OutputPin,
        <Spi as Transfer<W>>::Error: Debug,
        <Pin as OutputPin>::Error: Debug,
    {
        type Error = SpiWithCsError<<Spi as Transfer<W>>::Error, <Pin as OutputPin>::Error>;

        /// Attempt an SPI transfer with automated CS assert/deassert
        fn transfer<'w>(&mut self, data: &'w mut [W]) -> Result<(), Self::Error> {
            // First assert CS
            self.cs.set_low().map_err(SpiWithCsError::Pin)?;

            // Attempt the transfer, storing the result for later
            let spi_result = self.spi.transfer(data).map_err(SpiWithCsError::Spi);

            // Deassert CS
            self.cs.set_high().map_err(SpiWithCsError::Pin)?;

            // Return failures
            spi_result
        }
    }

    impl<Spi, Pin, W> Write<W> for SpiWithCs<Spi, Pin>
    where
        Spi: Write<W>,
        Pin: OutputPin,
        <Spi as Write<W>>::Error: Debug,
        <Pin as OutputPin>::Error: Debug,
    {
        type Error = SpiWithCsError<<Spi as Write<W>>::Error, <Pin as OutputPin>::Error>;

        /// Attempt an SPI write with automated CS assert/deassert
        fn write<'w>(&mut self, data: &'w [W]) -> Result<(), Self::Error> {
            // First assert CS
            self.cs.set_low().map_err(SpiWithCsError::Pin)?;

            // Attempt the transfer, storing the result for later
            let spi_result = self.spi.write(data).map_err(SpiWithCsError::Spi);

            // Deassert CS
            self.cs.set_high().map_err(SpiWithCsError::Pin)?;

            // Return failures
            spi_result
        }
    }

    impl<Spi, Pin, W> WriteIter<W> for SpiWithCs<Spi, Pin>
    where
        Spi: WriteIter<W>,
        Pin: OutputPin,
        <Spi as WriteIter<W>>::Error: Debug,
        <Pin as OutputPin>::Error: Debug,
    {
        type Error = SpiWithCsError<<Spi as WriteIter<W>>::Error, <Pin as OutputPin>::Error>;

        /// Attempt an SPI write_iter with automated CS assert/deassert
        fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
        where
            WI: IntoIterator<Item = W>,
        {
            // First assert CS
            self.cs.set_low().map_err(SpiWithCsError::Pin)?;

            // Attempt the transfer, storing the result for later
            let spi_result = self.spi.write_iter(words).map_err(SpiWithCsError::Spi);

            // Deassert CS
            self.cs.set_high().map_err(SpiWithCsError::Pin)?;

            // Return failures
            spi_result
        }
    }
}
