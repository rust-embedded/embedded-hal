//! Blocking serial API

/// Write half of a serial interface (blocking variant)
pub trait Write {
    /// Word type
    type Word;

    /// The type of error that can occur when writing
    type Error;

    /// Writes a slice, blocking until everything has been written
    ///
    /// An implementation can choose to buffer the write, returning `Ok(())`
    /// after the complete slice has been written to a buffer, but before all
    /// words have been sent via the serial interface. To make sure that
    /// everything has been sent, call [`flush`] after this function returns.
    ///
    /// [`flush`]: #tymethod.flush
    fn write(&mut self, buffer: &[Self::Word]) -> Result<(), Self::Error>;

    /// Block until the serial interface has sent all buffered words
    fn flush(&mut self) -> Result<(), Self::Error>;
}

/// Blocking serial write
pub mod write {
    /// Marker trait to opt into default blocking write implementation
    ///
    /// Implementers of [`nonblocking::serial::Write`] can implement this marker trait
    /// for their type. Doing so will automatically provide the default
    /// implementation of [`blocking::serial::Write`] for the type.
    ///
    /// [`nonblocking::serial::Write`]: ../../nonblocking/serial/trait.Write.html
    /// [`blocking::serial::Write`]: ../trait.Write.html
    pub trait Default: crate::nb::serial::Write {}

    impl<S> crate::blocking::serial::Write for S
    where
        S: Default,
        S::Word: Clone,
    {
        type Word = S::Word;
        type Error = S::Error;

        fn write(&mut self, buffer: &[Self::Word]) -> Result<(), Self::Error> {
            for word in buffer {
                nb::block!(self.write(word.clone()))?;
            }

            Ok(())
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            nb::block!(self.flush())?;
            Ok(())
        }
    }
}
