//! I2S API

/// Blocking I2S traits
pub mod blocking {

    /// Blocking I2S trait
    pub trait I2s<W> {
        /// Error type
        type Error: core::fmt::Debug;

        /// Reads enough bytes to fill `left_words` and `right_words`.
        fn read<'w>(
            &mut self,
            left_words: &'w mut [W],
            right_words: &'w mut [W],
        ) -> Result<(), Self::Error>;

        /// Sends `left_words` and `right_words`.
        fn write<'w>(
            &mut self,
            left_words: &'w [W],
            right_words: &'w [W],
        ) -> Result<(), Self::Error>;

        /// Sends `left_words` and `right_words`.
        fn write_iter<LW, RW>(
            &mut self,
            left_words: LW,
            right_words: RW,
        ) -> Result<(), Self::Error>
        where
            LW: IntoIterator<Item = W>,
            RW: IntoIterator<Item = W>;
    }
}
