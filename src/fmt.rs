//! Implementation of `core::fmt::Write` for the HAL's `serial::Write`.
//!
//! TODO write example of usage
use core::fmt::{Result, Write};

impl<Word, Error: crate::serial::Error> Write
    for dyn crate::serial::nb::Write<Word, Error = Error> + '_
where
    Word: Copy + From<u8>,
{
    fn write_str(&mut self, s: &str) -> Result {
        let _ = s
            .bytes()
            .map(|c| nb::block!(self.write(Word::from(c))))
            .last();
        Ok(())
    }
}
