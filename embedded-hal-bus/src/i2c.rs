//! I2C bus sharing mechanisms.

/// I2C bus sharing with blocking traits
pub mod blocking {
    use embedded_hal::i2c::{
        blocking::{I2cBus, I2cDevice},
        ErrorType,
    };

    /// [`I2cDevice`] implementation with exclusive access to the bus (not shared).
    ///
    /// This is the most straightforward way of obtaining an [`I2cDevice`] from an [`I2cBus`],
    /// ideal for when no sharing is required (only one I2C device is present on the bus).
    #[derive(Debug)]
    pub struct ExclusiveDevice<BUS> {
        bus: BUS,
    }

    impl<BUS> ExclusiveDevice<BUS> {
        /// Create a new `ExclusiveDevice`
        pub fn new(bus: BUS) -> Self {
            Self { bus }
        }
    }

    impl<BUS> ErrorType for ExclusiveDevice<BUS>
    where
        BUS: ErrorType,
    {
        type Error = BUS::Error;
    }

    impl<BUS> I2cDevice for ExclusiveDevice<BUS>
    where
        BUS: I2cBus,
    {
        type Bus = BUS;

        fn transaction<R>(
            &mut self,
            f: impl FnOnce(&mut Self::Bus) -> Result<R, <Self::Bus as ErrorType>::Error>,
        ) -> Result<R, Self::Error> {
            let f_res = f(&mut self.bus);

            // On failure, it's important to still stop and flush.
            let stop_res = self.bus.stop();
            let flush_res = self.bus.flush();

            // Note that in the case of multiple failures, only one error
            // will be returned, in the following order.
            let f_ok = f_res?;
            stop_res?;
            flush_res?;
            Ok(f_ok)
        }
    }
}
