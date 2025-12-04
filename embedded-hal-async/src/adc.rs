//! Asynchronous analog-digital conversion traits.

pub use embedded_hal::adc::{Error, ErrorKind, ErrorType};

/// Asynchronous voltmeter for measuring voltage.
///
/// # Examples
///
/// In the first naive example, [`Voltmeter`] is implemented using a spin loop.
///
/// ```
/// use embedded_hal_async::adc::{ErrorKind, ErrorType, Error, Voltmeter};
///
/// struct MySpinningVoltmeter;
///
/// impl MySpinningVoltmeter {
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
/// impl ErrorType for MySpinningVoltmeter {
///     type Error = ErrorKind;
/// }
///
/// impl Voltmeter for MySpinningVoltmeter {
///     async fn measure_nv(&mut self) -> Result<i64, Self::Error> {
///         Ok(self.measure_mv().await? as i64 * 1_000_000)
///     }
///
///     async fn measure_mv(&mut self) -> Result<i16, Self::Error> {
///         while !self.is_ready() {
///             core::hint::spin_loop();
///         }
///
///         Ok(self.data() as i16)
///     }
/// }
/// ```
///
/// The second example assumes an ADC that supports a “ready pin” which implements [`Wait`](crate::digital::Wait).
/// When the “ready pin” goes high, data is ready.
///
/// ```
/// use embedded_hal_async::{
///     adc::{self, ErrorKind, ErrorType, Error, Voltmeter},
///     digital::{self, Wait, Error as _, ErrorType as _},
/// };
///
/// struct MyWaitingVoltmeter<T> {
///     ready_pin: T,
/// };
///
/// impl<T> MyWaitingVoltmeter<T> {
///     pub fn data(&mut self) -> u16 {
///         3300
///     }
/// }
///
/// impl<T> adc::ErrorType for MyWaitingVoltmeter<T> {
///     type Error = adc::ErrorKind;
/// }
///
/// impl<T: Wait> Voltmeter for MyWaitingVoltmeter<T> {
///     async fn measure_nv(&mut self) -> Result<i64, Self::Error> {
///         Ok(self.measure_mv().await? as i64 * 1_000_000)
///     }
///
///     async fn measure_mv(&mut self) -> Result<i16, Self::Error> {
///         match self.ready_pin.wait_for_high().await {
///             Ok(()) => (),
///             Err(err) => return Err(match err.kind() {
///                 digital::ErrorKind::Other => adc::ErrorKind::Other,
///                 _ => adc::ErrorKind::Other,
///             })
///         }
///
///         Ok(self.data() as i16)
///     }
/// }
/// ```
pub trait Voltmeter: ErrorType {
    /// Measures voltage in nV (nanovolts).
    ///
    /// This can measure between -9223372036.854775808V and 9223372036.854775807V.
    async fn measure_nv(&mut self) -> Result<i64, Self::Error>;

    /// Measures voltage in mV (microvolts).
    ///
    /// This can measure between -2147.483648V and 2147.483647V.
    /// If you need to measure a larger range, use [`measure_nv`](Voltmeter::measure_nv) instead.
    ///
    /// When overriding the default implementation, ensure that the measured voltage is clamped
    /// between [`i32::MIN`] and [`i32::MAX`].
    async fn measure_uv(&mut self) -> Result<i32, Self::Error> {
        Ok((self.measure_nv().await? / 1_000).clamp(i32::MIN.into(), i32::MAX.into()) as i32)
    }

    /// Measures voltage in mV (millivolts).
    ///
    /// This can measure between between -32.768V and 32.767V.
    /// If you need to measure a larger range,
    /// use [`measure_uv`](Voltmeter::measure_uv) or [`measure_nv`](Voltmeter::measure_nv) instead.
    ///
    /// When overriding the default implementation, ensure that the measured voltage is clamped
    /// between [`i16::MIN`] and [`i16::MAX`].
    async fn measure_mv(&mut self) -> Result<i16, Self::Error> {
        Ok((self.measure_uv().await? / 1_000).clamp(i16::MIN.into(), i16::MAX.into()) as i16)
    }
}

impl<T> Voltmeter for &mut T
where
    T: Voltmeter + ?Sized,
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
    async fn measure_mv(&mut self) -> Result<i16, Self::Error> {
        (*self).measure_mv().await
    }
}

/// Asynchronous ammeter (ampere meter) for measuring current.
pub trait Ammeter: ErrorType {
    /// Measures current in nA (nanoampere).
    ///
    /// This can measure between -9223372036.854775808A and 9223372036.854775807A.
    async fn measure_na(&mut self) -> Result<i64, Self::Error>;

    /// Measures current in uA (microampere).
    ///
    /// This can measure between -2147.483648A and 2147.483647A.
    /// If you need to measure a larger range, use [`measure_na`](Ammeter::measure_na) instead.
    ///
    /// When overriding the default implementation, ensure that the measured current is clamped
    /// between [`i32::MIN`] and [`i32::MAX`].
    async fn measure_ua(&mut self) -> Result<i32, Self::Error> {
        Ok((self.measure_na().await? / 1_000).clamp(i32::MIN.into(), i32::MAX.into()) as i32)
    }

    /// Measures current in mA (milliampere).
    ///
    /// This can measure between between -32.768A and 32.767A.
    /// If you need to measure a larger range,
    /// use [`measure_ua`](Ammeter::measure_ua) or [`measure_na`](Ammeter::measure_na) instead.
    ///
    /// When overriding the default implementation, ensure that the measured voltage is clamped
    /// between [`i16::MIN`] and [`i16::MAX`].
    async fn measure_ma(&mut self) -> Result<i16, Self::Error> {
        Ok((self.measure_ua().await? / 1_000).clamp(i16::MIN.into(), i16::MAX.into()) as i16)
    }
}

impl<T> Ammeter for &mut T
where
    T: Ammeter + ?Sized,
{
    #[inline]
    async fn measure_na(&mut self) -> Result<i64, Self::Error> {
        (*self).measure_na().await
    }

    #[inline]
    async fn measure_ua(&mut self) -> Result<i32, Self::Error> {
        (*self).measure_ua().await
    }

    #[inline]
    async fn measure_ma(&mut self) -> Result<i16, Self::Error> {
        (*self).measure_ma().await
    }
}
