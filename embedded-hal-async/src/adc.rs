//! Asynchronous analog-digital conversion traits.

pub use embedded_hal::adc::{Error, ErrorKind, ErrorType};

/// Read data from an ADC.
///
/// # Examples
///
/// In the first naive example, [`read`](crate::adc::AdcChannel::read) is implemented
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
///     pub fn data(&mut self) -> u32 {
///         42
///     }
/// }
///
/// impl ErrorType for MySpinningAdc {
///     type Error = ErrorKind;
/// }
///
/// impl AdcChannel for MySpinningAdc {
///     async fn read(&mut self) -> Result<u32, Self::Error> {
///         while !self.is_ready() {
///             core::hint::spin_loop();
///         }
///
///         Ok(self.data())
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
///     pub fn data(&mut self) -> u32 {
///         42
///     }
/// }
///
/// impl<T> adc::ErrorType for MyWaitingAdc<T> {
///     type Error = adc::ErrorKind;
/// }
///
/// impl<T: Wait> AdcChannel for MyWaitingAdc<T> {
///     async fn read(&mut self) -> Result<u32, Self::Error> {
///         match self.ready_pin.wait_for_high().await {
///             Ok(()) => (),
///             Err(err) => return Err(match err.kind() {
///                 digital::ErrorKind::Other => adc::ErrorKind::Other,
///                 _ => adc::ErrorKind::Other,
///             })
///         }
///
///         Ok(self.data())
///     }
/// }
/// ```
pub trait AdcChannel: ErrorType {
    /// Reads data from the ADC.
    ///
    /// # Note for Implementers
    ///
    /// This should wait until data is ready and then read it.
    /// If the ADC's precision is less than 32 bits, the value must be scaled accordingly.
    async fn read(&mut self) -> Result<u32, Self::Error>;
}

impl<T> AdcChannel for &mut T
where
    T: AdcChannel + ?Sized,
{
    #[inline]
    async fn read(&mut self) -> Result<u32, Self::Error> {
        (*self).read().await
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Scale an integer containing `bits` bits to 32 bits.
    fn scale_bits(raw_data: u32, bits: u32) -> u32 {
        let mut scaled_data: u32 = 0;

        let mut remaining_bits = u32::BITS;
        while remaining_bits > 0 {
            let shl = bits.min(remaining_bits);
            scaled_data = (scaled_data.wrapping_shl(shl)) | (raw_data.wrapping_shr(bits - shl));
            remaining_bits -= shl;
        }

        scaled_data
    }

    #[test]
    fn scale_bits_i8_to_i32() {
        let raw_data = u32::from(i8::MIN as u8);
        let scaled_data = scale_bits(raw_data, 8);
        assert!(i32::MIN <= (scaled_data as i32) && (scaled_data as i32) <= (i32::MIN + 1 << 8));
    }

    macro_rules! impl_adc {
        ($Adc:ident, $bits:literal, $uint:ty) => {
            struct $Adc($uint);

            impl $Adc {
                const MAX: $uint = !(<$uint>::MAX.wrapping_shl($bits - 1).wrapping_shl(1));

                pub fn data(&mut self) -> $uint {
                    self.0
                }
            }

            impl ErrorType for $Adc {
                type Error = core::convert::Infallible;
            }

            impl AdcChannel for $Adc {
                async fn read(&mut self) -> Result<u32, Self::Error> {
                    Ok(scale_bits(u32::from(self.data()), $bits))
                }
            }
        };
    }

    macro_rules! test_adc {
        ($Adc:ident, $bits:literal, $uint:ty) => {{
            impl_adc!($Adc, $bits, $uint);

            // 0 should always be scaled to 0.
            let mut adc_0 = $Adc(0);
            assert_eq!(adc_0.read().await, Ok(0));

            // `$Adc::MAX` should always be scaled to `u32::MAX`.
            let mut adc_max = $Adc($Adc::MAX);
            assert_eq!(adc_max.read().await, Ok(u32::MAX));
        }};
    }

    #[tokio::test]
    async fn test_8_bit() {
        test_adc!(Adc8, 8, u8);
    }

    #[tokio::test]
    async fn test_12_bit() {
        test_adc!(Adc12, 12, u16);
    }

    #[tokio::test]
    async fn test_16_bit() {
        test_adc!(Adc16, 16, u16);
    }

    #[tokio::test]
    async fn test_24_bit() {
        test_adc!(Adc24, 24, u32);
    }

    #[tokio::test]
    async fn test_32_bit() {
        test_adc!(Adc32, 32, u32);
    }
}
