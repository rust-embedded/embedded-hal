//! Asynchronous Analog to Digital conversion.
//!
//! See the [blocking](embedded_hal::adc) types documentation for more information.

use core::future::Future;

use embedded_hal::adc::{Channel, ErrorType};

/// Asynchronous ADC channel capable of a one shot measurement.
pub trait OneShotChannel: Channel + ErrorType {
    /// The maximum value that may be produced by the ADC.
    ///
    /// This returns [`Ready`](core::task::Poll::Ready) when the max value is obtained. Some implementations such
    /// as a channel connected over I2C may return [`Pending`](core::task::Poll::Pending) while the max value is
    /// being obtained.
    ///
    /// This is guaranteed to not change once a channel is constructed.
    ///
    /// For example, a 12-bit ADC would return 4095.
    async fn max_value(&self) -> Result<Self::Word, Self::Error>;

    /// Sample from the ADC.
    ///
    /// This returns [`Ready`](core::task::Poll::Ready) when the sample is complete.
    async fn sample(&mut self) -> Result<Self::Word, Self::Error>;
}

impl<T: OneShotChannel + ?Sized> OneShotChannel for &mut T {
    #[inline]
    fn max_value(&self) -> impl Future<Output = Result<Self::Word, Self::Error>> {
        T::max_value(self)
    }

    #[inline]
    fn sample(&mut self) -> impl Future<Output = Result<Self::Word, Self::Error>> {
        T::sample(self)
    }
}
