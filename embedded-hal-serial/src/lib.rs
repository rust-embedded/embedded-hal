//! Serial interface

#![deny(missing_docs)]
#![no_std]

/// Read half of a serial interface
///
/// Some serial interfaces support different data sizes (8 bits, 9 bits, etc.);
/// This can be encoded in this trait via the `Word` type parameter.
pub trait Read<Word> {
    /// Read error
    type Error;

    /// Reads a single word from the serial interface
    fn read(&mut self) -> nb::Result<Word, Self::Error>;
}

/// Write half of a serial interface
pub trait Write<Word> {
    /// Write error
    type Error;

    /// Writes a single word to the serial interface
    fn write(&mut self, word: Word) -> nb::Result<(), Self::Error>;

    /// Ensures that none of the previously written words are still buffered
    fn flush(&mut self) -> nb::Result<(), Self::Error>;
}

/// Blocking serial API
pub mod blocking {

    /// Write half of a serial interface (blocking variant)
    pub trait Write<Word> {
        /// The type of error that can occur when writing
        type Error;

        /// Writes a slice, blocking until everything has been written
        ///
        /// An implementation can choose to buffer the write, returning `Ok(())`
        /// after the complete slice has been written to a buffer, but before all
        /// words have been sent via the serial interface. To make sure that
        /// everything has been sent, call [`bflush`] after this function returns.
        ///
        /// [`bflush`]: #tymethod.bflush
        fn bwrite_all(&mut self, buffer: &[Word]) -> Result<(), Self::Error>;

        /// Block until the serial interface has sent all buffered words
        fn bflush(&mut self) -> Result<(), Self::Error>;
    }

    /// Blocking serial write
    pub mod write {
        /// Marker trait to opt into default blocking write implementation
        ///
        /// Implementers of [`crate::Write`] can implement this marker trait
        /// for their type. Doing so will automatically provide the default
        /// implementation of [`blocking::Write`] for the type.
        ///
        /// [`crate::Write`]: ../../serial/trait.Write.html
        /// [`blocking::Write`]: ../trait.Write.html
        pub trait Default<Word>: crate::Write<Word> {}

        impl<S, Word> crate::blocking::Write<Word> for S
        where
            S: Default<Word>,
            Word: Clone,
        {
            type Error = S::Error;

            fn bwrite_all(&mut self, buffer: &[Word]) -> Result<(), Self::Error> {
                use nb::block;
                for word in buffer {
                    block!(self.write(word.clone()))?;
                }

                Ok(())
            }

            fn bflush(&mut self) -> Result<(), Self::Error> {
                use nb::block;
                block!(self.flush())?;
                Ok(())
            }
        }
    }
}
