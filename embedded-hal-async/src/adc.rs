//! Asynchronous analog-digital conversion traits.

pub use embedded_hal::adc::{Error, ErrorKind, ErrorType};

/// Read data from an ADC.
///
/// # Note for Implementers
///
/// This should wait until data is ready and then read it.
///
/// # Examples
///
/// In the first naive example, [`AdcChannel`] is implemented
/// using a spin loop and only returns once data is ready.
///
/// ```
/// # use embedded_hal_async::adc::{AdcChannel, ErrorKind, ErrorType, Error};
/// #
/// struct MySpinningAdc;
///
/// impl MySpinningAdc {
///     pub fn is_ready(&mut self) -> bool {
///         // Just pretend this returns `false` the first few times.
///         true
///     }
///
///     pub fn data(&mut self) -> u16 {
///         3300
///     }
/// }
///
/// impl ErrorType for MySpinningAdc {
///     type Error = ErrorKind;
/// }
///
/// impl AdcChannel for MySpinningAdc {
///     async fn measure_nv(&mut self) -> Result<i64, Self::Error> {
///         Ok(self.measure_mv().await? as i64 * 1_000_000)
///     }
///
///     async fn measure_mv(&mut self) -> Result<i32, Self::Error> {
///         while !self.is_ready() {
///             core::hint::spin_loop();
///         }
///
///         Ok(self.data() as i32)
///     }
/// }
/// ```
///
/// The second example assumes an ADC that supports a “ready pin” which implements [`Wait`](crate::digital::Wait).
/// When the “ready pin” goes high, data is ready.
///
/// ```
/// # use embedded_hal_async::{adc::{self, ErrorKind, ErrorType, Error, AdcChannel}, digital::{self, Wait, Error as _, ErrorType as _}};
/// #
/// struct MyWaitingAdc<T> {
///     ready_pin: T,
/// };
///
/// impl<T> MyWaitingAdc<T> {
///     pub fn data(&mut self) -> u16 {
///         3300
///     }
/// }
///
/// impl<T> adc::ErrorType for MyWaitingAdc<T> {
///     type Error = adc::ErrorKind;
/// }
///
/// impl<T: Wait> AdcChannel for MyWaitingAdc<T> {
///     async fn measure_nv(&mut self) -> Result<i64, Self::Error> {
///         Ok(self.measure_mv().await? as i64 * 1_000_000)
///     }
///
///     async fn measure_mv(&mut self) -> Result<i32, Self::Error> {
///         match self.ready_pin.wait_for_high().await {
///             Ok(()) => (),
///             Err(err) => return Err(match err.kind() {
///                 digital::ErrorKind::Other => adc::ErrorKind::Other,
///                 _ => adc::ErrorKind::Other,
///             })
///         }
///
///         Ok(self.data() as i32)
///     }
/// }
/// ```
pub trait AdcChannel: ErrorType {
    /// Take a measurement in nV (nanovolts).
    async fn measure_nv(&mut self) -> Result<i64, Self::Error>;

    /// Take a measurement in mV (microvolts).
    async fn measure_uv(&mut self) -> Result<i32, Self::Error> {
        Ok((self.measure_nv().await? / 1_000) as i32)
    }

    /// Take a measurement in mV (millivolts).
    async fn measure_mv(&mut self) -> Result<i32, Self::Error> {
        Ok(self.measure_uv().await? / 1_000)
    }
}

impl<T> AdcChannel for &mut T
where
    T: AdcChannel + ?Sized,
{
    #[inline]
    async fn measure_nv(&mut self) -> Result<i64, Self::Error> {
        (*self).measure_nv().await
    }

    #[inline]
    async fn measure_uv(&mut self) -> Result<i32, Self::Error> {
        (*self).measure_uv().await
    }

    #[inline]
    async fn measure_mv(&mut self) -> Result<i32, Self::Error> {
        (*self).measure_mv().await
    }
}
