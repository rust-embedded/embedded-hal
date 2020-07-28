//! Storage
/// Traits to allow on and off board storage deivces to read and write data
/// Allows for Read Only Memory as well as Read and Write Memory


use nb;

/// Address represents an unsigned int. This allows for devices that have bigger or smaller address spaces than the host.
pub struct Address<U>(U);
/// Address Offset represents an unsigned int that is used as an optional offset from the base address.
pub struct AddressOffset<U>(U);

use core::ops::Add;

/// Implement add for the Address and AddressOffset Types.
impl<'a, 'b, U> Add<&'b AddressOffset<U>> for &'a Address<U> where U: Add<U, Output=U> + Copy {
    type Output = Address<U>;

    fn add(self, other: &'b AddressOffset<U>) -> Address<U> {
        Address(self.0 + other.0)
    }

}

/// Page represents an unsigned int that is a Page ID in the device memory space.
pub struct Page<U>(U);

/// Read a single word from the device.
///
/// `Word` type allows any word size to be used.
pub trait SingleRead<Word,U> {
    /// An enumeration of Storage errors
    type Error;

    /// Reads the word stored at the address
    /// ```
    /// pub fn try_read(&mut self, address: Address) -> nb::Result<u8, Self::Error>
    ///     let address = address as *const _;
    ///     unsafe {
    ///         Ok(core::slice::from_raw_parts::<'static, u8>(address,length)) 
    ///     }
    /// ```
    fn try_read(&mut self, address: Address<U>) -> nb::Result<Word, Self::Error>;

}

/// Write a single word to the device.
///
/// `Word` type allows any word size to be used.
pub trait SingleWrite<Word,U> {
    /// An enumeration of Storage errors
    type Error;

    /// Writes the word to the address
    fn try_write(&mut self, address: Address<U>, word: Word) -> nb::Result<(), Self::Error>;

}

/// Read multiple bytes from the device.
///
/// Intended to be used for when there is a optimized method of reading multiple bytes.
/// 
/// Iterating over the slice is a valid method to ```impl``` this trait.
pub trait MultiRead<Word,U> {
    /// An enumeration of Storage errors
    type Error;

    /// Reads the words stored at the address to fill the buffer
    /// ```
    /// pub fn try_read_slice(&mut self, address: Address,  buf: &mut [Word]) -> nb::Result<Word, Self::Error>
    ///     let address = address as *const _;
    ///     unsafe {
    ///         buf = core::slice::from_raw_parts::<'static, Word>(address,buf.len())
    ///     }
    ///     
    ///     Ok() 
    /// }
    /// ```
    fn try_read_slice(&mut self, address: Address<U>, buf: &mut [Word]) -> nb::Result<(), Self::Error>;
}

/// Write multiple bytes to the device.
///
/// Intended to be used for when there is a optimized method of reading multiple bytes.
/// 
/// Iterating over the slice is a valid method to ```impl``` this trait.
pub trait MultiWrite<Word,U> {
    /// An enumeration of Storage errors
    type Error;

    /// Writes the buffer to the address
    fn try_write_slice(&mut self, address: Address<U>, buf: &[Word]) -> nb::Result<(), Self::Error>;
}

/// A common interface to erase pages or memory locations.
///
/// For Flash storage, the write functions can't set a bit to 1.
///
/// For non flash devices, this trait is not required, but it can be used to erase data as recommended by the device (EG set all fields to 0).
pub trait ErasePage<U> {
    /// An enumeration of Storage errors
    type Error;

    /// Erase the page of memory
    /// For flash devices, this sets the whole page to 0xFF
    /// Implementations should mask the address as required to get the page to erase
    fn try_erase_page(&mut self, page: Page<U>) -> nb::Result<(), Self::Error>;

    /// Erase the page of memory at the address. Note: The only valid address is the start of the page
    /// For flash devices, this sets the whole page to 0xFF
    /// Implementations should mask the address as required to get the page to erase
    fn try_erase_address(&mut self, address: Address<U>) -> nb::Result<(), Self::Error>;
}

/// Allow for checking that data can fit before writing to the device.
///
/// As some devices have limits on writing accross pages, the page size is also included.
 pub trait StorageSize<Word,U> {
    /// An enumeration of Storage errors
    type Error;

    /// Returns the start address and the maximum size that can be stored by the device
    fn try_total_size(&mut self) -> nb::Result<(Address<U>,AddressOffset<U>), Self::Error>;
    /// For devices that are paged, this should return the size of the page
    ///
    /// For non paged devices, this should return the AddressOffset in ```try_total_size```
    fn try_page_size(&mut self) -> nb::Result<AddressOffset<U>, Self::Error>;
}