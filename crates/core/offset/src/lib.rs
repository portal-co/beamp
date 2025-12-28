//! Generic offset management utilities.

/// A trait to construct types from an offset value.
///
/// Implementations may return the offset (e.g., usize) or `()` for writers that ignore offsets.
pub trait FromOffset<Off> {
    fn from_offset(offset: Off) -> Self;
}

impl FromOffset<usize> for usize {
    fn from_offset(offset: usize) -> Self {
        offset
    }
}

impl FromOffset<u64> for u64 {
    fn from_offset(offset: u64) -> Self {
        offset
    }
}

impl FromOffset<u32> for u32 {
    fn from_offset(offset: u32) -> Self {
        offset
    }
}

// Writers can ignore offsets by using `()` as the implementation.
impl<Off> FromOffset<Off> for () {
    fn from_offset(_offset: Off) -> Self {}
}
