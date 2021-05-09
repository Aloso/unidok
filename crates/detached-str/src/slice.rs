use std::fmt;
use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

/// A memory efficient string slice without a lifetime.
///
/// To get the content of the string slice, the original string
/// must be still around.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct StrSlice {
    pub(super) start: usize,
    pub(super) end: usize,
}

impl StrSlice {
    #[inline]
    pub(crate) fn new(range: Range<usize>) -> Self {
        StrSlice { start: range.start, end: range.end }
    }

    /// Get a reference to the str slice's start.
    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

    /// Get a reference to the str slice's end.
    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }

    /// Returns the [`Range`] of this slice.
    #[inline]
    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    /// Returns the length of this slice.
    #[inline]
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns whether this slice is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.end == self.start
    }

    /// Joins this slice with another slice. The slices must be adjacent, i.e.
    /// `self.end == other.start`.
    ///
    /// ## Panics
    ///
    /// Panics if the slices aren't adjacent.
    #[inline]
    pub fn join(self, other: Self) -> Self {
        assert!(self.end == other.start);
        StrSlice { start: self.start, end: other.end }
    }

    /// Joins this slice with another slice. It succeeds if the slices are
    /// adjacent, i.e. `self.end == other.start`.
    #[inline]
    pub fn try_join(self, other: Self) -> Option<Self> {
        if self.end == other.start {
            Some(StrSlice { start: self.start, end: other.end })
        } else {
            None
        }
    }

    /// Returns a subslice of this slice. This function accepts all kinds of
    /// ranges.
    pub fn get<T>(&self, index: T) -> StrSlice
    where
        Self: StrSliceIndex<T>,
    {
        self.index(index)
    }

    /// Returns the `&str` corresponding to this slice.
    #[inline]
    pub fn to_str(self, text: &str) -> &str {
        &text[self.range()]
    }

    pub fn trim_end_matches<P>(self, pattern: P, text: &str) -> StrSlice
    where
        P: FnMut(char) -> bool,
    {
        let before = &text[self.range()];
        let after = before.trim_end_matches(pattern);
        let diff = before.len() - after.len();
        StrSlice { start: self.start, end: self.end - diff }
    }
}

impl fmt::Debug for StrSlice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StrSlice @ {}..{}", self.start, self.end)
    }
}

/// Trait for indexing a [`StrSlice`] (using the [`StrSlice::get`] method)
pub trait StrSliceIndex<T> {
    fn index(&self, index: T) -> StrSlice;
}

impl StrSliceIndex<Range<usize>> for StrSlice {
    #[inline]
    fn index(&self, index: Range<usize>) -> StrSlice {
        let (start, end) = (self.start + index.start, self.start + index.end);
        if start > end || end > self.end {
            panic!(
                "Range {}..{} too big to index a StrSlice with length {}",
                index.start,
                index.end,
                self.len(),
            );
        }
        StrSlice::new(start..end)
    }
}

impl StrSliceIndex<RangeInclusive<usize>> for StrSlice {
    #[inline]
    fn index(&self, index: RangeInclusive<usize>) -> StrSlice {
        let (start, end) = (self.start + index.start(), self.start + index.end() + 1);
        if start > end || end > self.end {
            panic!(
                "Range {}..={} too big to index a StrSlice with length {}",
                index.start(),
                index.end(),
                self.len(),
            );
        }
        StrSlice::new(start..end)
    }
}

impl StrSliceIndex<RangeFrom<usize>> for StrSlice {
    #[inline]
    fn index(&self, index: RangeFrom<usize>) -> StrSlice {
        let start = self.start + index.start;
        if start > self.end {
            panic!(
                "Range {}.. too big to index a StrSlice with length {}",
                index.start,
                self.len(),
            );
        }
        StrSlice::new(start..self.end)
    }
}

impl StrSliceIndex<RangeTo<usize>> for StrSlice {
    #[inline]
    fn index(&self, index: RangeTo<usize>) -> StrSlice {
        let end = self.start + index.end;
        if end > self.end {
            panic!("Range ..{} too big to index a StrSlice with length {}", index.end, self.len(),);
        }
        StrSlice::new(self.start..end)
    }
}

impl StrSliceIndex<RangeToInclusive<usize>> for StrSlice {
    #[inline]
    fn index(&self, index: RangeToInclusive<usize>) -> StrSlice {
        let end = self.start + index.end + 1;
        if end > self.end {
            panic!("Range ..{} too big to index a StrSlice with length {}", index.end, self.len(),);
        }
        StrSlice::new(self.start..end)
    }
}

impl StrSliceIndex<RangeFull> for StrSlice {
    #[inline]
    fn index(&self, _: RangeFull) -> StrSlice {
        *self
    }
}
