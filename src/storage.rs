//! Storage
//! The Read and Write traits are seperate to allow for Read Only Memory as well as Read and Write
//! Example implementations include:

use nb;

/// Read a single word from the device
/// `Word` type allows any word size to be used
pub trait SingleRead<Word> {
    /// An enumeration of Storage errors
    type Error;

    /// Reads the word stored at the address
    /// ```
    /// pub fn try_read(&mut self, address: usize) -> nb::Result<u8, Self::Error>
    ///     let address = address as *const _;
    ///     unsafe {
    ///         Ok(core::slice::from_raw_parts::<'static, u8>(address,length)) 
    ///     }
    /// ```
    fn try_read(&mut self, address: usize) -> nb::Result<Word, Self::Error>;

}

/// Write a single word to the device
/// `Word` type allows any word size to be used
pub trait SingleWrite<Word> {
    /// An enumeration of Storage errors
    type Error;

    /// Writes the word to the address
    fn try_write(&mut self, address: usize, word: Word) -> nb::Result<(), Self::Error>;

}

/// Read multiple bytes from the device
/// Intended to be used for when there is a optimized method of reading multiple bytes
/// 
/// Iterating over the slice is a valid method to ```impl``` this trait
pub trait MultiRead<Word> {
    /// An enumeration of Storage errors
    type Error;

    /// Reads the words stored at the address to fill the buffer
    /// ```
    /// pub fn try_read_slice(&mut self, address: usize,  buf: &mut [Word]) -> nb::Result<Word, Self::Error>
    ///     let address = address as *const _;
    ///     unsafe {
    ///         buf = core::slice::from_raw_parts::<'static, Word>(address,buf.len())
    ///     }
    ///     
    ///     Ok() 
    /// }
    /// ```
    fn try_read_slice(&mut self, address: usize, buf: &mut [Word]) -> nb::Result<(), Self::Error>;
}

/// Write multiple bytes to the device
/// Intended to be used for when there is a optimized method of reading multiple bytes
/// 
/// Iterating over the slice is a valid method to ```impl``` this trait
pub trait MultiWrite<Word> {
    /// An enumeration of Storage errors
    type Error;

    /// Writes the buffer to the address
    fn try_write_slice(&mut self, address: usize, buf: &[Word]) -> nb::Result<(), Self::Error>;
}

/// For Flash storage, the write functions can't set a bit to 1. To enable a complete rewrite flash, it needs to be cleared beforehand
/// For non flash devices, this trait is not required, but it can be used to clear data as recommended by the device
pub trait ClearPage<Word> {
    /// An enumeration of Storage errors
    type Error;

    /// Clear the page of memory at the address
    /// For flash devices, this sets the whole page to 0xFF
    fn try_clear(&mut self, address: usize) -> nb::Result<(), Self::Error>;
}