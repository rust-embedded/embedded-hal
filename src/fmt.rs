//! Implementation of `core::fmt::Write` for the HAL's `serial::Write`.
//!
//! TODO write example of usage
use core::fmt::{Result, Write};

impl<Word, Error> Write for ::serial::Write<Word, Error=Error>
where
    Word: From<u8>,
{
    fn write_str(&mut self, s: &str) -> Result {
        let _ = s.as_bytes()
            .into_iter()
            .map(|c| block!(self.write(Word::from(*c))))
            .last();
        Ok(())
    }
}
