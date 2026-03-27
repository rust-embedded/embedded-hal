//! # embedded-storage-async - An async Storage Abstraction Layer for Embedded Systems
//!
//! Storage traits to allow on and off board storage devices to read and write
//! data asynchronously.

#![no_std]
#![allow(async_fn_in_trait)]

pub mod nor_flash;

/// Transparent read only storage trait
pub trait ReadStorage {
    /// An enumeration of storage errors
    type Error;

    /// Read a slice of data from the storage peripheral, starting the read
    /// operation at the given address offset, and reading `bytes.len()` bytes.
    ///
    /// This should throw an error in case `bytes.len()` will be larger than
    /// `self.capacity() - offset`.
    async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error>;

    /// The capacity of the storage peripheral in bytes.
    fn capacity(&self) -> usize;
}

/// Transparent read/write storage trait
pub trait Storage: ReadStorage {
    /// Write a slice of data to the storage peripheral, starting the write
    /// operation at the given address offset (between 0 and `self.capacity()`).
    ///
    /// **NOTE:**
    /// This function will automatically erase any pages necessary to write the given data,
    /// and might as such do RMW operations at an undesirable performance impact.
    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error>;
}
