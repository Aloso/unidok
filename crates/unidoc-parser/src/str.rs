use std::ops::{Deref, Range, RangeFrom, RangeInclusive, RangeTo};

/// An immutable string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Str(Box<str>);

impl Str {
    pub fn slice(&self) -> StrSlice {
        StrSlice::new(0..self.len())
    }

    pub fn get<T>(&self, index: T) -> <StrSlice as OwnedIndex<T>>::Output
    where
        StrSlice: OwnedIndex<T>,
    {
        self.slice().index(index)
    }
}

impl From<String> for Str {
    fn from(s: String) -> Self {
        Str(s.into_boxed_str())
    }
}

impl Deref for Str {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A memory efficient string slice without a lifetime.
///
/// To get the content of the string slice, the original string
/// must be still around.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct StrSlice {
    start: usize,
    end: usize,
}

impl StrSlice {
    fn new(range: Range<usize>) -> Self {
        StrSlice { start: range.start, end: range.end }
    }

    /// Get a reference to the str slice's start.
    pub fn start(&self) -> usize {
        self.start
    }

    /// Get a reference to the str slice's end.
    pub fn end(&self) -> usize {
        self.end
    }

    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.end == self.start
    }

    pub fn get<T>(&self, index: T) -> <Self as OwnedIndex<T>>::Output
    where
        Self: OwnedIndex<T>,
    {
        self.index(index)
    }
}

pub trait OwnedIndex<T> {
    type Output;

    fn index(&self, index: T) -> Self::Output;
}

impl OwnedIndex<Range<usize>> for StrSlice {
    type Output = StrSlice;

    fn index(&self, index: Range<usize>) -> Self::Output {
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

impl OwnedIndex<RangeInclusive<usize>> for StrSlice {
    type Output = StrSlice;

    fn index(&self, index: RangeInclusive<usize>) -> Self::Output {
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

impl OwnedIndex<RangeFrom<usize>> for StrSlice {
    type Output = StrSlice;

    fn index(&self, index: RangeFrom<usize>) -> Self::Output {
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

impl OwnedIndex<RangeTo<usize>> for StrSlice {
    type Output = StrSlice;

    fn index(&self, index: RangeTo<usize>) -> Self::Output {
        let end = self.start + index.end;
        if end > self.end {
            panic!(
                "Range ..{} too big to index a StrSlice with length {}",
                index.end,
                self.len(),
            );
        }
        StrSlice::new(self.start..end)
    }
}
