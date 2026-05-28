use crate::Region;

/// Iterator producing block-region pairs, where each memory block maps to each
/// region.
pub struct OverlapIterator<'a, R, I>
where
	R: Region,
	I: Iterator<Item = R>,
{
	memory: &'a [u8],
	regions: I,
	base_address: u32,
}

/// Trait allowing us to automatically add an `overlaps` function to all iterators over [`Region`]
pub trait IterableByOverlaps<'a, R, I>
where
	R: Region,
	I: Iterator<Item = R>,
{
	/// Obtain an [`OverlapIterator`] over a subslice of `memory` that overlaps with the region in `self`
	fn overlaps(self, memory: &'a [u8], base_address: u32) -> OverlapIterator<'a, R, I>;
}

impl<'a, R, I> Iterator for OverlapIterator<'a, R, I>
where
	R: Region,
	I: Iterator<Item = R>,
{
	type Item = (&'a [u8], R, u32);

	fn next(&mut self) -> Option<Self::Item> {
		let mem_start = self.base_address;
		let mem_end = self.base_address + self.memory.len() as u32;
		for region in self.regions.by_ref() {
			if mem_start < region.end() && mem_end >= region.start() {
				let addr_start = core::cmp::max(mem_start, region.start());
				let addr_end = core::cmp::min(mem_end, region.end());
				let start = (addr_start - self.base_address) as usize;
				let end = (addr_end - self.base_address) as usize;
				return Some((&self.memory[start..end], region, addr_start));
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
	fn overlaps(self, memory: &'a [u8], base_address: u32) -> OverlapIterator<'a, R, I> {
		OverlapIterator {
			memory,
			regions: self,
			base_address,
		}
	}
}
