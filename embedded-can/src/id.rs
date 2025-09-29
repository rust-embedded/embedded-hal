//! CAN Identifiers.

/// Standard 11-bit CAN Identifier (`0..=0x7FF`).
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct StandardId(u16);

impl StandardId {
    /// CAN ID `0`, the highest priority.
    pub const ZERO: Self = Self(0);

    /// CAN ID `0x7FF`, the lowest priority.
    pub const MAX: Self = Self(Self::MAX_RAW);

    /// Raw CAN ID `0x7FF`, the lowest priority.
    const MAX_RAW: u16 = 0x7FF;

    /// Tries to create a `StandardId` from a raw 16-bit integer.
    ///
    /// This will return `None` if `raw` is out of range of an 11-bit integer (`> 0x7FF`).
    #[inline]
    #[must_use]
    pub const fn new(raw: u16) -> Option<Self> {
        if raw <= Self::MAX_RAW {
            Some(Self(raw))
        } else {
            None
        }
    }

    /// Creates a new `StandardId` without checking if it is inside the valid range.
    ///
    /// # Safety
    /// Using this method can create an invalid ID and is thus marked as unsafe.
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(raw: u16) -> Self {
        Self(raw)
    }

    /// Returns this CAN Identifier as a raw 16-bit integer.
    #[inline]
    #[must_use]
    pub const fn as_raw(&self) -> u16 {
        self.0
    }
}

/// Extended 29-bit CAN Identifier (`0..=1FFF_FFFF`).
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct ExtendedId(u32);

impl ExtendedId {
    /// CAN ID `0`, the highest priority.
    pub const ZERO: Self = Self(0);

    /// CAN ID `0x1FFFFFFF`, the lowest priority.
    pub const MAX: Self = Self(Self::MAX_RAW);

    /// Raw CAN ID `0x1FFFFFFF`, the lowest priority.
    const MAX_RAW: u32 = 0x1FFF_FFFF;

    /// Tries to create a `ExtendedId` from a raw 32-bit integer.
    ///
    /// This will return `None` if `raw` is out of range of an 29-bit integer (`> 0x1FFF_FFFF`).
    #[inline]
    #[must_use]
    pub const fn new(raw: u32) -> Option<Self> {
        if raw <= Self::MAX_RAW {
            Some(Self(raw))
        } else {
            None
        }
    }

    /// Creates a new `ExtendedId` without checking if it is inside the valid range.
    ///
    /// # Safety
    /// Using this method can create an invalid ID and is thus marked as unsafe.
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns this CAN Identifier as a raw 32-bit integer.
    #[inline]
    #[must_use]
    pub const fn as_raw(&self) -> u32 {
        self.0
    }

    /// Returns the Base ID part of this extended identifier.
    #[must_use]
    pub fn standard_id(&self) -> StandardId {
        // ID-28 to ID-18
        StandardId((self.0 >> 18) as u16)
    }
}

/// A CAN Identifier (standard or extended).
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Id {
    /// Standard 11-bit Identifier (`0..=0x7FF`).
    Standard(StandardId),

    /// Extended 29-bit Identifier (`0..=0x1FFF_FFFF`).
    Extended(ExtendedId),
}

impl Id {
    /// Creates a CAN identifier as a standard ID.
    pub fn new_standard(raw: u16) -> Option<Self> {
        Some(Id::from(StandardId::new(raw)?))
    }

    /// Creates a CAN identifier as an extended ID.
    pub fn new_extended(raw: u32) -> Option<Self> {
        Some(Id::from(ExtendedId::new(raw)?))
    }

    /// Determines if the value is a standard, 11-bit, identifier.
    pub fn is_standard(&self) -> bool {
        matches!(self, Id::Standard(_))
    }
    /// Determines if the value is an extended, 29-bit, identifier.
    pub fn is_extended(&self) -> bool {
        matches!(self, Id::Extended(_))
    }
}

/// Implement `Ord` according to the CAN arbitration rules
///
/// When performing arbitration, frames are looked at bit for bit starting
/// from the beginning. A bit with the value 0 is dominant and a bit with
/// value of 1 is recessive.
///
/// When two devices are sending frames at the same time, as soon as the first
/// bit is found which differs, the frame with the corresponding dominant
/// 0 bit will win and get to send the rest of the frame.
///
/// This implementation of `Ord` for `Id` will take this into consideration
/// and when comparing two different instances of `Id` the "smallest" will
/// always be the ID which would form the most dominant frame, all other
/// things being equal.
impl Ord for Id {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let split_id = |id: &Id| {
            let (standard_id_part, ide_bit, extended_id_part) = match id {
                Id::Standard(StandardId(x)) => (*x, 0, 0),
                Id::Extended(x) => (
                    x.standard_id().0,
                    1,
                    x.0 & ((1 << 18) - 1), // Bit ID-17 to ID-0
                ),
            };
            (standard_id_part, ide_bit, extended_id_part)
        };

        split_id(self).cmp(&split_id(other))
    }
}

impl PartialOrd for Id {
    fn partial_cmp(&self, other: &Id) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<StandardId> for Id {
    #[inline]
    fn from(id: StandardId) -> Self {
        Id::Standard(id)
    }
}

impl From<ExtendedId> for Id {
    #[inline]
    fn from(id: ExtendedId) -> Self {
        Id::Extended(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_id_new() {
        assert_eq!(StandardId::new(StandardId::MAX_RAW), Some(StandardId::MAX));
    }

    #[test]
    fn standard_id_new_out_of_range() {
        assert_eq!(StandardId::new(StandardId::MAX_RAW + 1), None);
    }

    #[test]
    fn standard_id_new_unchecked_out_of_range() {
        let id = StandardId::MAX_RAW + 1;
        assert_eq!(unsafe { StandardId::new_unchecked(id) }, StandardId(id));
    }

    #[test]
    fn extended_id_new() {
        assert_eq!(
            ExtendedId::new(ExtendedId::MAX.as_raw()),
            Some(ExtendedId::MAX)
        );
    }

    #[test]
    fn extended_id_new_out_of_range() {
        assert_eq!(ExtendedId::new(ExtendedId::MAX_RAW + 1), None);
    }

    #[test]
    fn extended_id_new_unchecked_out_of_range() {
        let id = ExtendedId::MAX_RAW + 1;
        assert_eq!(unsafe { ExtendedId::new_unchecked(id) }, ExtendedId(id));
    }

    #[test]
    fn get_standard_id_from_extended_id() {
        assert_eq!(
            Some(ExtendedId::MAX.standard_id()),
            StandardId::new((ExtendedId::MAX.0 >> 18) as u16)
        );
    }

    #[test]
    fn cmp_id() {
        assert!(StandardId::ZERO < StandardId::MAX);
        assert!(ExtendedId::ZERO < ExtendedId::MAX);

        assert!(Id::Standard(StandardId::ZERO) < Id::Extended(ExtendedId::ZERO));
        assert!(Id::Extended(ExtendedId::ZERO) < Id::Extended(ExtendedId::MAX));
        assert!(Id::Extended(ExtendedId((1 << 11) - 1)) < Id::Standard(StandardId(1)));
        assert!(Id::Standard(StandardId(1)) < Id::Extended(ExtendedId::MAX));
    }

    #[test]
    fn id_new() {
        let id = Id::new_standard(StandardId::MAX_RAW).unwrap();
        assert!(id.is_standard());
        assert!(!id.is_extended());
        match id {
            Id::Standard(id) => assert_eq!(StandardId::MAX, id),
            _ => assert!(false),
        }

        let id = Id::new_extended(ExtendedId::MAX_RAW).unwrap();
        assert!(!id.is_standard());
        assert!(id.is_extended());
        match id {
            Id::Extended(id) => assert_eq!(ExtendedId::MAX, id),
            _ => assert!(false),
        }
    }

    #[test]
    fn id_raw() {
        let id = StandardId::new(StandardId::MAX_RAW).unwrap();
        assert_eq!(StandardId::MAX_RAW, id.as_raw());

        let id = ExtendedId::new(ExtendedId::MAX_RAW).unwrap();
        assert_eq!(ExtendedId::MAX_RAW, id.as_raw());
    }
}
