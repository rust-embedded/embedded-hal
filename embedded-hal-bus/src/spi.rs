//! SPI bus sharing mechanisms.

/// SPI bus sharing with blocking traits
pub mod blocking {
    use core::fmt::Debug;
    use embedded_hal::{
        digital::blocking::OutputPin,
        spi::{
            blocking::{SpiBusFlush, SpiDevice},
            Error, ErrorKind, ErrorType,
        },
    };

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
    /// This is the most straightforward way of obtaining an [`SpiDevice`] from an [`SpiBus`](embedded_hal::spi::blocking::SpiBus),
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
}
