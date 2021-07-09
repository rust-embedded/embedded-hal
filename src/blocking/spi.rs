//! Blocking SPI API

/// Blocking transfer
pub trait Transfer<W> {
    /// Error type
    type Error;

    /// Writes `words` to the slave. Returns the `words` received from the slave
    fn transfer<'w>(&mut self, words: &'w mut [W]) -> Result<&'w [W], Self::Error>;
}

/// Blocking write
pub trait Write<W> {
    /// Error type
    type Error;

    /// Writes `words` to the slave, ignoring all the incoming words
    fn write(&mut self, words: &[W]) -> Result<(), Self::Error>;
}

/// Blocking write (iterator version)
pub trait WriteIter<W> {
    /// Error type
    type Error;

    /// Writes `words` to the slave, ignoring all the incoming words
    fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
    where
        WI: IntoIterator<Item = W>;
}

/// ManagedCS marker trait specifies that all `spi` operations will be preceded by
/// asserting the CS pin, and followed by de-asserting the CS pin.
///
/// TODO: document wrappers that can be used where this is required
pub trait ManagedCs {}

/// Blocking transfer
pub mod transfer {
    /// Default implementation of `blocking::spi::Transfer<W>` for implementers of
    /// `nonblocking::spi::FullDuplex<W>`
    pub trait Default<W>: crate::nb::spi::FullDuplex<W> {}

    impl<W, S> crate::blocking::spi::Transfer<W> for S
    where
        S: Default<W>,
        W: Clone,
    {
        type Error = S::Error;

        fn transfer<'w>(&mut self, words: &'w mut [W]) -> Result<&'w [W], S::Error> {
            for word in words.iter_mut() {
                nb::block!(self.write(word.clone()))?;
                *word = nb::block!(self.read())?;
            }

            Ok(words)
        }
    }
}

/// Blocking write
pub mod write {
    /// Default implementation of `blocking::spi::Write<W>` for implementers
    /// of `nonblocking::spi::FullDuplex<W>`
    pub trait Default<W>: crate::nb::spi::FullDuplex<W> {}

    impl<W, S> crate::blocking::spi::Write<W> for S
    where
        S: Default<W>,
        W: Clone,
    {
        type Error = S::Error;

        fn write(&mut self, words: &[W]) -> Result<(), S::Error> {
            for word in words {
                nb::block!(self.write(word.clone()))?;
                nb::block!(self.read())?;
            }

            Ok(())
        }
    }
}

/// Blocking write (iterator version)
pub mod write_iter {
    /// Default implementation of `blocking::spi::WriteIter<W>` for implementers of
    /// `nonblocking::spi::FullDuplex<W>`
    pub trait Default<W>: crate::nb::spi::FullDuplex<W> {}

    impl<W, S> crate::blocking::spi::WriteIter<W> for S
    where
        S: Default<W>,
        W: Clone,
    {
        type Error = S::Error;

        fn write_iter<WI>(&mut self, words: WI) -> Result<(), S::Error>
        where
            WI: IntoIterator<Item = W>,
        {
            for word in words.into_iter() {
                nb::block!(self.write(word.clone()))?;
                nb::block!(self.read())?;
            }

            Ok(())
        }
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
    type Error;

    /// Execute the provided transactions
    fn exec<'a>(&mut self, operations: &mut [Operation<'a, W>]) -> Result<(), Self::Error>;
}

/// Blocking transactional impl over spi::Write and spi::Transfer
pub mod transactional {
    use super::{Operation, Transfer, Write};

    /// Default implementation of `blocking::spi::Transactional<W>` for implementers of
    /// `spi::Write<W>` and `spi::Transfer<W>`
    pub trait Default<W>: Write<W> + Transfer<W> {}

    impl<W: 'static, E, S> super::Transactional<W> for S
    where
        S: self::Default<W> + Write<W, Error = E> + Transfer<W, Error = E>,
        W: Copy + Clone,
    {
        type Error = E;

        fn exec<'a>(&mut self, operations: &mut [super::Operation<'a, W>]) -> Result<(), E> {
            for op in operations {
                match op {
                    Operation::Write(w) => self.write(w)?,
                    Operation::Transfer(t) => self.transfer(t).map(|_| ())?,
                }
            }

            Ok(())
        }
    }
}

/// Provides SpiWithCS wrapper around an spi::* and OutputPin impl
pub mod spi_with_cs {

    use core::fmt::Debug;
    use core::marker::PhantomData;

    use super::{ManagedCs, Transfer, Write, WriteIter};
    use crate::digital::OutputPin;

    /// SpiWithCS wraps an blocking::spi* implementation with Chip Select (CS)
    /// pin management.
    /// For sharing SPI between peripherals, see [shared-bus](https://crates.io/crates/shared-bus)
    pub struct SpiWithCs<Spi, SpiError, Pin, PinError> {
        spi: Spi,
        cs: Pin,

        _spi_err: PhantomData<SpiError>,
        _pin_err: PhantomData<PinError>,
    }

    /// Underlying causes for errors. Either SPI communication or CS pin state setting error
    #[derive(Clone, Debug, PartialEq)]
    pub enum SpiWithCsError<SpiError, PinError> {
        /// Underlying SPI communication error
        Spi(SpiError),
        /// Underlying chip-select pin state setting error
        Pin(PinError),
    }

    /// ManagedCS marker trait indicates Chip Select management is automatic
    impl<Spi, SpiError, Pin, PinError> ManagedCs for SpiWithCs<Spi, SpiError, Pin, PinError> {}

    impl<Spi, SpiError, Pin, PinError> SpiWithCs<Spi, SpiError, Pin, PinError>
    where
        Pin: crate::digital::OutputPin<Error = PinError>,
        SpiError: Debug,
        PinError: Debug,
    {
        /// Create a new SpiWithCS wrapper with the provided Spi and Pin
        pub fn new(spi: Spi, cs: Pin) -> Self {
            Self {
                spi,
                cs,
                _spi_err: PhantomData,
                _pin_err: PhantomData,
            }
        }

        /// Fetch references to the inner Spi and Pin types.
        /// Note that using these directly will violate the `ManagedCs` constraint.
        pub fn inner(&mut self) -> (&mut Spi, &mut Pin) {
            (&mut self.spi, &mut self.cs)
        }

        /// Destroy the SpiWithCs wrapper, returning the bus and pin objects
        pub fn destroy(self) -> (Spi, Pin) {
            (self.spi, self.cs)
        }
    }

    impl<Spi, SpiError, Pin, PinError> Transfer<u8> for SpiWithCs<Spi, SpiError, Pin, PinError>
    where
        Spi: Transfer<u8, Error = SpiError>,
        Pin: OutputPin<Error = PinError>,
        SpiError: Debug,
        PinError: Debug,
    {
        type Error = SpiWithCsError<SpiError, PinError>;

        /// Attempt an SPI transfer with automated CS assert/deassert
        fn try_transfer<'w>(&mut self, data: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
            // First assert CS
            self.cs.try_set_low().map_err(SpiWithCsError::Pin)?;

            // Attempt the transfer, storing the result for later
            let spi_result = self.spi.try_transfer(data).map_err(SpiWithCsError::Spi);

            // Deassert CS
            self.cs.try_set_high().map_err(SpiWithCsError::Pin)?;

            // Return failures
            spi_result
        }
    }

    impl<Spi, SpiError, Pin, PinError> Write<u8> for SpiWithCs<Spi, SpiError, Pin, PinError>
    where
        Spi: Write<u8, Error = SpiError>,
        Pin: OutputPin<Error = PinError>,
        SpiError: Debug,
        PinError: Debug,
    {
        type Error = SpiWithCsError<SpiError, PinError>;

        /// Attempt an SPI write with automated CS assert/deassert
        fn try_write<'w>(&mut self, data: &'w [u8]) -> Result<(), Self::Error> {
            // First assert CS
            self.cs.try_set_low().map_err(SpiWithCsError::Pin)?;

            // Attempt the transfer, storing the result for later
            let spi_result = self.spi.try_write(data).map_err(SpiWithCsError::Spi);

            // Deassert CS
            self.cs.try_set_high().map_err(SpiWithCsError::Pin)?;

            // Return failures
            spi_result
        }
    }

    impl<Spi, SpiError, Pin, PinError> WriteIter<u8> for SpiWithCs<Spi, SpiError, Pin, PinError>
    where
        Spi: WriteIter<u8, Error = SpiError>,
        Pin: OutputPin<Error = PinError>,
        SpiError: Debug,
        PinError: Debug,
    {
        type Error = SpiWithCsError<SpiError, PinError>;

        /// Attempt an SPI write_iter with automated CS assert/deassert
        fn try_write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
        where
            WI: IntoIterator<Item = u8>,
        {
            // First assert CS
            self.cs.try_set_low().map_err(SpiWithCsError::Pin)?;

            // Attempt the transfer, storing the result for later
            let spi_result = self.spi.try_write_iter(words).map_err(SpiWithCsError::Spi);

            // Deassert CS
            self.cs.try_set_high().map_err(SpiWithCsError::Pin)?;

            // Return failures
            spi_result
        }
    }
}
