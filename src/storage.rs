//! Storage traits to allow on and off board storage deivces to read and write
//! data.
//!
//! Implementation based on `Cuervo`s great work in
//! https://www.ecorax.net/as-above-so-below-1/ and
//! https://www.ecorax.net/as-above-so-below-2/

use core::ops::{Add, BitOr, Sub};
use nb;

/// Trait to check if two entities are bitwise subset of another.
pub trait BitSubset {
    /// Check that every '1' bit is a '1' on the right hand side.
    fn is_subset_of(&self, rhs: &Self) -> bool;
}

/// Blanket implementation of [`BitSubset`] for all arrays of a type implementing [`BitOr`]
impl<T: Copy + Eq + BitOr<Output = T>> BitSubset for [T] {
    fn is_subset_of(&self, rhs: &Self) -> bool {
        if self.len() > rhs.len() {
            false
        } else {
            self.iter().zip(rhs.iter()).all(|(a, b)| (*a | *b) == *b)
        }
    }
}

/// An address denotes the read/write address of a single word.
#[derive(Default, Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Ord)]
pub struct Address(pub u32);

impl Add<usize> for Address {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Address(self.0 + rhs as u32)
    }
}

impl Add<Address> for Address {
    type Output = Self;

    fn add(self, rhs: Address) -> Self::Output {
        Address(self.0 + rhs.0)
    }
}

impl Sub<Address> for Address {
    type Output = Self;

    fn sub(self, rhs: Address) -> Self::Output {
        Address(self.0 - rhs.0)
    }
}

/// A region denotes a contiguous piece of memory between two addresses.
pub trait Region {
    /// Check if `address` is contained in the region of `Self`
    fn contains(&self, address: Address) -> bool;
}

/// Iterator producing block-region pairs, where each memory block maps to each
/// region.
pub struct OverlapIterator<'a, R, I>
where
    R: Region,
    I: Iterator<Item = R>,
{
    memory: &'a [u8],
    regions: I,
    base_address: Address,
}

/// Trait allowing us to automatically add an `overlaps` function to all iterators over [`Region`]
pub trait IterableByOverlaps<'a, R, I>
where
    R: Region,
    I: Iterator<Item = R>,
{
    /// Obtain an [`OverlapIterator`] over a subslice of `memory` that overlaps with the region in `self`
    fn overlaps(self, memory: &'a [u8], base_address: Address) -> OverlapIterator<R, I>;
}

impl<'a, R, I> Iterator for OverlapIterator<'a, R, I>
where
    R: Region,
    I: Iterator<Item = R>,
{
    type Item = (&'a [u8], R, Address);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(region) = self.regions.next() {
            //  TODO: This might be possible to do in a smarter way?
            let mut block_range = (0..self.memory.len())
                .skip_while(|index| !region.contains(self.base_address + Address(*index as u32)))
                .take_while(|index| region.contains(self.base_address + Address(*index as u32)));
            if let Some(start) = block_range.next() {
                let end = block_range.last().unwrap_or(start) + 1;
                return Some((
                    &self.memory[start..end],
                    region,
                    self.base_address + Address(start as u32),
                ));
            }
        }
        None
    }
}

/// Blanket implementation for all types implementing [`Iterator`] over [`Regions`]
impl<'a, R, I> IterableByOverlaps<'a, R, I> for I
where
    R: Region,
    I: Iterator<Item = R>,
{
    fn overlaps(self, memory: &'a [u8], base_address: Address) -> OverlapIterator<R, I> {
        OverlapIterator {
            memory,
            regions: self,
            base_address,
        }
    }
}

/// Storage trait
pub trait ReadWrite {
    /// An enumeration of storage errors
    type Error;

    /// Read a slice of data from the storage peripheral, starting the read
    /// operation at the given address, and reading until end address
    /// (`self.range().1`) or buffer length, whichever comes first.
    fn try_read(&mut self, address: Address, bytes: &mut [u8]) -> nb::Result<(), Self::Error>;

    /// Write a slice of data to the storage peripheral, starting the write
    /// operation at the given address.
    fn try_write(&mut self, address: Address, bytes: &[u8]) -> nb::Result<(), Self::Error>;

    /// The range of possible addresses within the peripheral.
    ///
    /// (start_addr, end_addr)
    fn range(&self) -> (Address, Address);

    /// Erase the given storage range, clearing all data within `[from..to]`.
    ///
    /// This should return an error if the range is not aligned to a proper
    /// erase resolution
    fn try_erase(&mut self, from: Address, to: Address) -> nb::Result<(), Self::Error>;
}
