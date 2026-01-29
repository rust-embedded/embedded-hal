#![no_std]
pub use embedded_hal::i2c::Operation;

/// I3C error.
pub trait Error: core::fmt::Debug {
    /// Convert error to a generic I3C error kind.
    ///
    /// By using this method, I3C errors freely defined by HAL implementations
    /// can be converted to a set of generic I3C errors upon which generic
    /// code can act.
    fn kind(&self) -> ErrorKind;
}

impl Error for core::convert::Infallible {
    #[inline]
    fn kind(&self) -> ErrorKind {
        match *self {}
    }
}

#[non_exhaustive]
pub enum ErrorKind {
    Bus,
    ArbitrationLoss,
    Overrun,
    Other,
}

/// I3C error type trait.
///
/// This just defines the error type, to be used by the other traits.
pub trait ErrorType {
    type Error: Error;
}

impl<T: ErrorType + ?Sized> ErrorType for &mut T {
    type Error = T::Error;
}

pub trait I3c<S: SpeedMode = Sdr>: ErrorType {
    // Here we also add read, write read_write like I2C removed for brevity

    /// Execute the provided operations on the I3C bus.
    ///
    /// Transaction contract:
    /// - Before executing the first operation an ST is sent automatically. This is followed by SAD+R/W as appropriate.
    /// - After the first time the address is send the bus switches to the specified speed
    /// - Data from adjacent operations of the same type are sent after each other without an SP or SR.
    /// - Between adjacent operations of a different type an SR and SAD+R/W is sent.
    /// - After executing the last operation an SP is sent automatically.
    /// - If the last operation is a `Read` the master does not send an acknowledge for the last byte.
    ///
    /// - `ST` = start condition
    /// - `SAD+R/W` = slave address followed by bit 1 to indicate reading or 0 to indicate writing
    /// - `SR` = repeated start condition
    /// - `SP` = stop condition
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error>;
}

trait Sealed {}

#[allow(private_bounds)]
pub trait SpeedMode: Sealed + 'static {}

/// Single data rate
pub struct Sdr {}

impl Sealed for Sdr {}

impl SpeedMode for Sdr {}

/// Double data rate
pub struct Ddr {}

impl Sealed for Ddr {}

impl SpeedMode for Ddr {}
