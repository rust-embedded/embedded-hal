//! Serial interface.

/// Serial error.
pub trait Error: core::fmt::Debug {
    /// Convert error to a generic serial error kind
    ///
    /// By using this method, serial errors freely defined by HAL implementations
    /// can be converted to a set of generic serial errors upon which generic
    /// code can act.
    fn kind(&self) -> ErrorKind;
}

impl Error for core::convert::Infallible {
    #[inline]
    fn kind(&self) -> ErrorKind {
        match *self {}
    }
}

/// Serial error kind.
///
/// This represents a common set of serial operation errors. HAL implementations are
/// free to define more specific or additional error types. However, by providing
/// a mapping to these common serial errors, generic code can still react to them.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    /// The peripheral receive buffer was overrun.
    Overrun,
    /// Received data does not conform to the peripheral configuration.
    /// Can be caused by a misconfigured device on either end of the serial line.
    FrameFormat,
    /// Parity check failed.
    Parity,
    /// Serial line is too noisy to read valid data.
    Noise,
    /// A different error occurred. The original error may contain more information.
    Other,
}

impl Error for ErrorKind {
    #[inline]
    fn kind(&self) -> ErrorKind {
        *self
    }
}

impl core::error::Error for ErrorKind {}

impl core::fmt::Display for ErrorKind {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Overrun => write!(f, "The peripheral receive buffer was overrun"),
            Self::Parity => write!(f, "Parity check failed"),
            Self::Noise => write!(f, "Serial line is too noisy to read valid data"),
            Self::FrameFormat => write!(
                f,
                "Received data does not conform to the peripheral configuration"
            ),
            Self::Other => write!(
                f,
                "A different error occurred. The original error may contain more information"
            ),
        }
    }
}

/// Serial error type trait.
///
/// This just defines the error type, to be used by the other traits.
pub trait ErrorType {
    /// Error type
    type Error: Error;
}

impl<T: ErrorType + ?Sized> ErrorType for &mut T {
    type Error = T::Error;
}

/// Read half of a serial interface.
///
/// Some serial interfaces support different data sizes (8 bits, 9 bits, etc.);
/// This can be encoded in this trait via the `Word` type parameter.
pub trait Read<Word: Copy = u8>: ErrorType {
    /// Reads a single word from the serial interface
    fn read(&mut self) -> nb::Result<Word, Self::Error>;
}

impl<T: Read<Word> + ?Sized, Word: Copy> Read<Word> for &mut T {
    #[inline]
    fn read(&mut self) -> nb::Result<Word, Self::Error> {
        T::read(self)
    }
}

/// Write half of a serial interface.
pub trait Write<Word: Copy = u8>: ErrorType {
    /// Writes a single word to the serial interface.
    fn write(&mut self, word: Word) -> nb::Result<(), Self::Error>;

    /// Ensures that none of the previously written words are still buffered.
    fn flush(&mut self) -> nb::Result<(), Self::Error>;
}

impl<T: Write<Word> + ?Sized, Word: Copy> Write<Word> for &mut T {
    #[inline]
    fn write(&mut self, word: Word) -> nb::Result<(), Self::Error> {
        T::write(self, word)
    }

    #[inline]
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        T::flush(self)
    }
}

/// Implementation of `core::fmt::Write` for the HAL's `serial::Write`.
///
/// TODO write example of usage
impl<Word, Error: self::Error> core::fmt::Write for dyn Write<Word, Error = Error> + '_
where
    Word: Copy + From<u8>,
{
    #[inline]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let _ = s
            .bytes()
            .map(|c| nb::block!(self.write(Word::from(c))))
            .last();
        Ok(())
    }
}
