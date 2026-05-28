use crate::{iter::IterableByOverlaps, ReadStorage, Region, Storage};

/// NOR flash errors.
///
/// NOR flash implementations must use an error type implementing this trait. This permits generic
/// code to extract a generic error kind.
pub trait NorFlashError: core::fmt::Debug {
	/// Convert a specific NOR flash error into a generic error kind.
	fn kind(&self) -> NorFlashErrorKind;
}

impl NorFlashError for core::convert::Infallible {
	fn kind(&self) -> NorFlashErrorKind {
		match *self {}
	}
}

/// A trait that NorFlash implementations can use to share an error type.
pub trait ErrorType {
	/// Errors returned by this NOR flash.
	type Error: NorFlashError;
}

/// NOR flash error kinds.
///
/// NOR flash implementations must map their error to those generic error kinds through the
/// [`NorFlashError`] trait.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum NorFlashErrorKind {
	/// The arguments are not properly aligned.
	NotAligned,

	/// The arguments are out of bounds.
	OutOfBounds,

	/// Error specific to the implementation.
	Other,
}

impl NorFlashError for NorFlashErrorKind {
	fn kind(&self) -> NorFlashErrorKind {
		*self
	}
}

impl core::fmt::Display for NorFlashErrorKind {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::NotAligned => write!(f, "Arguments are not properly aligned"),
			Self::OutOfBounds => write!(f, "Arguments are out of bounds"),
			Self::Other => write!(f, "An implementation specific error occurred"),
		}
	}
}

/// Read only NOR flash trait.
pub trait ReadNorFlash: ErrorType {
	/// The minumum number of bytes the storage peripheral can read
	const READ_SIZE: usize;

	/// Read a slice of data from the storage peripheral, starting the read
	/// operation at the given address offset, and reading `bytes.len()` bytes.
	///
	/// # Errors
	///
	/// Returns an error if the arguments are not aligned or out of bounds. The implementation
	/// can use the [`check_read`] helper function.
	fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error>;

	/// The capacity of the peripheral in bytes.
	fn capacity(&self) -> usize;
}

/// Return whether a read operation is within bounds.
pub fn check_read<T: ReadNorFlash>(
	flash: &T,
	offset: u32,
	length: usize,
) -> Result<(), NorFlashErrorKind> {
	check_slice(flash, T::READ_SIZE, offset, length)
}

/// NOR flash trait.
pub trait NorFlash: ReadNorFlash {
	/// The minumum number of bytes the storage peripheral can write
	const WRITE_SIZE: usize;

	/// The minumum number of bytes the storage peripheral can erase
	const ERASE_SIZE: usize;

	/// Erase the given storage range, clearing all data within `[from..to]`.
	/// The given range will contain all 1s afterwards.
	///
	/// If power is lost during erase, contents of the page are undefined.
	///
	/// # Errors
	///
	/// Returns an error if the arguments are not aligned or out of bounds (the case where `to >
	/// from` is considered out of bounds). The implementation can use the [`check_erase`]
	/// helper function.
	fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error>;

	/// If power is lost during write, the contents of the written words are undefined,
	/// but the rest of the page is guaranteed to be unchanged.
	/// It is not allowed to write to the same word twice.
	///
	/// # Errors
	///
	/// Returns an error if the arguments are not aligned or out of bounds. The implementation
	/// can use the [`check_write`] helper function.
	fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error>;
}

/// Return whether an erase operation is aligned and within bounds.
pub fn check_erase<T: NorFlash>(flash: &T, from: u32, to: u32) -> Result<(), NorFlashErrorKind> {
	let (from, to) = (from as usize, to as usize);
	if from > to || to > flash.capacity() {
		return Err(NorFlashErrorKind::OutOfBounds);
	}
	if from % T::ERASE_SIZE != 0 || to % T::ERASE_SIZE != 0 {
		return Err(NorFlashErrorKind::NotAligned);
	}
	Ok(())
}

/// Return whether a write operation is aligned and within bounds.
pub fn check_write<T: NorFlash>(
	flash: &T,
	offset: u32,
	length: usize,
) -> Result<(), NorFlashErrorKind> {
	check_slice(flash, T::WRITE_SIZE, offset, length)
}

fn check_slice<T: ReadNorFlash>(
	flash: &T,
	align: usize,
	offset: u32,
	length: usize,
) -> Result<(), NorFlashErrorKind> {
	let offset = offset as usize;
	if length > flash.capacity() || offset > flash.capacity() - length {
		return Err(NorFlashErrorKind::OutOfBounds);
	}
	if !offset.is_multiple_of(align) || !length.is_multiple_of(align) {
		return Err(NorFlashErrorKind::NotAligned);
	}
	Ok(())
}

impl<T: ErrorType> ErrorType for &mut T {
	type Error = T::Error;
}

impl<T: ReadNorFlash> ReadNorFlash for &mut T {
	const READ_SIZE: usize = T::READ_SIZE;

	fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
		T::read(self, offset, bytes)
	}

	fn capacity(&self) -> usize {
		T::capacity(self)
	}
}

impl<T: NorFlash> NorFlash for &mut T {
	const WRITE_SIZE: usize = T::WRITE_SIZE;
	const ERASE_SIZE: usize = T::ERASE_SIZE;

	fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
		T::erase(self, from, to)
	}

	fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
		T::write(self, offset, bytes)
	}
}

/// Marker trait for NorFlash relaxing the restrictions on `write`.
///
/// Writes to the same word twice are now allowed. The result is the logical AND of the
/// previous data and the written data. That is, it is only possible to change 1 bits to 0 bits.
///
/// If power is lost during write:
/// - Bits that were 1 on flash and are written to 1 are guaranteed to stay as 1
/// - Bits that were 1 on flash and are written to 0 are undefined
/// - Bits that were 0 on flash are guaranteed to stay as 0
/// - Rest of the bits in the page are guaranteed to be unchanged
pub trait MultiwriteNorFlash: NorFlash {}
impl<T: MultiwriteNorFlash> MultiwriteNorFlash for &mut T {}

struct Page {
	pub start: u32,
	pub size: usize,
}

impl Page {
	fn new(index: u32, size: usize) -> Self {
		Self {
			start: index * size as u32,
			size,
		}
	}
}

impl Region for Page {
	fn start(&self) -> u32 {
		self.start
	}

	fn end(&self) -> u32 {
		self.start + self.size as u32
	}
}

/// Read-Modify-Write (RMW) Nor Flash storage structure.
pub struct RmwNorFlashStorage<'a, S> {
	storage: S,
	merge_buffer: &'a mut [u8],
}

impl<'a, S> RmwNorFlashStorage<'a, S>
where
	S: NorFlash,
{
	/// Instantiate a new generic `Storage` from a `NorFlash` peripheral
	///
	/// **NOTE** This will panic if the provided merge buffer,
	/// is smaller than the erase size of the flash peripheral
	pub fn new(nor_flash: S, merge_buffer: &'a mut [u8]) -> Self {
		if merge_buffer.len() < S::ERASE_SIZE {
			panic!("Merge buffer is too small");
		}

		Self {
			storage: nor_flash,
			merge_buffer,
		}
	}
}

impl<'a, S> ReadStorage for RmwNorFlashStorage<'a, S>
where
	S: ReadNorFlash,
{
	type Error = S::Error;

	fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
		// Nothing special to be done for reads
		self.storage.read(offset, bytes)
	}

	fn capacity(&self) -> usize {
		self.storage.capacity()
	}
}

impl<'a, S> Storage for RmwNorFlashStorage<'a, S>
where
	S: NorFlash,
{
	fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
		// Perform read/modify/write operations on the byte slice.
		let last_page = self.storage.capacity() / S::ERASE_SIZE;

		// `data` is the part of `bytes` contained within `page`,
		// and `addr` in the address offset of `page` + any offset into the page as requested by `address`
		for (data, page, addr) in (0..last_page as u32)
			.map(move |i| Page::new(i, S::ERASE_SIZE))
			.overlaps(bytes, offset)
		{
			let offset_into_page = addr.saturating_sub(page.start) as usize;

			self.storage
				.read(page.start, &mut self.merge_buffer[..S::ERASE_SIZE])?;

			// If we cannot write multiple times to the same page, we will have to erase it
			self.storage.erase(page.start, page.end())?;
			self.merge_buffer[..S::ERASE_SIZE]
				.iter_mut()
				.skip(offset_into_page)
				.zip(data)
				.for_each(|(byte, input)| *byte = *input);
			self.storage
				.write(page.start, &self.merge_buffer[..S::ERASE_SIZE])?;
		}
		Ok(())
	}
}

/// Read-Modify-Write (RMW) Multi-Write Nor Flash storage structure.
pub struct RmwMultiwriteNorFlashStorage<'a, S> {
	storage: S,
	merge_buffer: &'a mut [u8],
}

impl<'a, S> RmwMultiwriteNorFlashStorage<'a, S>
where
	S: MultiwriteNorFlash,
{
	/// Instantiate a new generic `Storage` from a `NorFlash` peripheral
	///
	/// **NOTE** This will panic if the provided merge buffer,
	/// is smaller than the erase size of the flash peripheral
	pub fn new(nor_flash: S, merge_buffer: &'a mut [u8]) -> Self {
		if merge_buffer.len() < S::ERASE_SIZE {
			panic!("Merge buffer is too small");
		}

		Self {
			storage: nor_flash,
			merge_buffer,
		}
	}
}

impl<'a, S> ReadStorage for RmwMultiwriteNorFlashStorage<'a, S>
where
	S: ReadNorFlash,
{
	type Error = S::Error;

	fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
		// Nothing special to be done for reads
		self.storage.read(offset, bytes)
	}

	fn capacity(&self) -> usize {
		self.storage.capacity()
	}
}

impl<'a, S> Storage for RmwMultiwriteNorFlashStorage<'a, S>
where
	S: MultiwriteNorFlash,
{
	fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
		// Perform read/modify/write operations on the byte slice.
		let last_page = self.storage.capacity() / S::ERASE_SIZE;

		// `data` is the part of `bytes` contained within `page`,
		// and `addr` in the address offset of `page` + any offset into the page as requested by `address`
		for (data, page, addr) in (0..last_page as u32)
			.map(move |i| Page::new(i, S::ERASE_SIZE))
			.overlaps(bytes, offset)
		{
			let offset_into_page = addr.saturating_sub(page.start) as usize;

			self.storage
				.read(page.start, &mut self.merge_buffer[..S::ERASE_SIZE])?;

			let rhs = &self.merge_buffer[offset_into_page..S::ERASE_SIZE];
			let is_subset = data.iter().zip(rhs.iter()).all(|(a, b)| *a & *b == *a);

			// Check if we can write the data block directly, under the limitations imposed by NorFlash:
			// - We can only change 1's to 0's
			if is_subset {
				// Use `merge_buffer` as allocation for padding `data` to `WRITE_SIZE`
				let offset = addr as usize % S::WRITE_SIZE;
				let aligned_end = data.len() % S::WRITE_SIZE + offset + data.len();
				self.merge_buffer[..aligned_end].fill(0xff);
				self.merge_buffer[offset..offset + data.len()].copy_from_slice(data);
				self.storage
					.write(addr - offset as u32, &self.merge_buffer[..aligned_end])?;
			} else {
				self.storage.erase(page.start, page.end())?;
				self.merge_buffer[..S::ERASE_SIZE]
					.iter_mut()
					.skip(offset_into_page)
					.zip(data)
					.for_each(|(byte, input)| *byte = *input);
				self.storage
					.write(page.start, &self.merge_buffer[..S::ERASE_SIZE])?;
			}
		}
		Ok(())
	}
}
