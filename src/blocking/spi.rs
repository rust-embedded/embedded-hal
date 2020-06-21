//! Blocking SPI API

/// Blocking transfer
pub trait Transfer<W> {
    /// Error type
    type Error;

    /// Sends `words` to the slave. Returns the `words` received from the slave
    fn try_transfer<'w>(&mut self, words: &'w mut [W]) -> Result<&'w [W], Self::Error>;
}

/// Blocking write
pub trait Write<W> {
    /// Error type
    type Error;

    /// Sends `words` to the slave, ignoring all the incoming words
    fn try_write(&mut self, words: &[W]) -> Result<(), Self::Error>;
}

/// Blocking write (iterator version)
pub trait WriteIter<W> {
    /// Error type
    type Error;

    /// Sends `words` to the slave, ignoring all the incoming words
    fn try_write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
    where
        WI: IntoIterator<Item = W>;
}

/// Blocking transfer
pub mod transfer {
    /// Default implementation of `blocking::spi::Transfer<W>` for implementers of
    /// `spi::FullDuplex<W>`
    pub trait Default<W>: crate::spi::FullDuplex<W> {}

    impl<W, S> crate::blocking::spi::Transfer<W> for S
    where
        S: Default<W>,
        W: Clone,
    {
        type Error = S::Error;

        fn try_transfer<'w>(&mut self, words: &'w mut [W]) -> Result<&'w [W], S::Error> {
            for word in words.iter_mut() {
                block!(self.try_send(word.clone()))?;
                *word = block!(self.try_read())?;
            }

            Ok(words)
        }
    }
}

/// Blocking write
pub mod write {
    /// Default implementation of `blocking::spi::Write<W>` for implementers of `spi::FullDuplex<W>`
    pub trait Default<W>: crate::spi::FullDuplex<W> {}

    impl<W, S> crate::blocking::spi::Write<W> for S
    where
        S: Default<W>,
        W: Clone,
    {
        type Error = S::Error;

        fn try_write(&mut self, words: &[W]) -> Result<(), S::Error> {
            for word in words {
                block!(self.try_send(word.clone()))?;
                block!(self.try_read())?;
            }

            Ok(())
        }
    }
}

/// Blocking write (iterator version)
pub mod write_iter {
    /// Default implementation of `blocking::spi::WriteIter<W>` for implementers of
    /// `spi::FullDuplex<W>`
    pub trait Default<W>: crate::spi::FullDuplex<W> {}

    impl<W, S> crate::blocking::spi::WriteIter<W> for S
    where
        S: Default<W>,
        W: Clone,
    {
        type Error = S::Error;

        fn try_write_iter<WI>(&mut self, words: WI) -> Result<(), S::Error>
        where
            WI: IntoIterator<Item = W>,
        {
            for word in words.into_iter() {
                block!(self.try_send(word.clone()))?;
                block!(self.try_read())?;
            }

            Ok(())
        }
    }
}
