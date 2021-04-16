//! A crate for borrowing strings without a lifetime.
//!
//! ## Example
//!
//! ```
//! use detached_str::{Str, StrSlice};
//!
//! let string: Str = "Hello, world!".into();
//! let slice: StrSlice = string.get(7..);
//! assert_eq!(slice.to_str(&string), "world!");
//! ```
//!
//! A `StrSlice` is "detached", i.e. the string content can only be accessed
//! when you have a reference to the owned string. The owned string is immutable
//! to ensure that string slices remain valid.

use std::borrow::Cow;
use std::fmt;
use std::iter::FromIterator;
use std::ops::Deref;
use std::path::PathBuf;

mod slice;
#[cfg(test)]
mod tests;

pub use slice::{StrSlice, StrSliceIndex};

/// An immutable string. It dereferences to a `&str` and can also be borrowed as
/// a [`StrSlice`].
#[derive(Clone, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
pub struct Str(Box<str>);

impl Str {
    pub fn get<T>(&self, index: T) -> StrSlice
    where
        StrSlice: StrSliceIndex<T>,
    {
        StrSlice::new(0..self.len()).index(index)
    }
}

impl From<String> for Str {
    fn from(s: String) -> Self {
        Str(s.into_boxed_str())
    }
}

impl<'a> From<&'a str> for Str {
    fn from(s: &'a str) -> Self {
        Str(s.to_string().into_boxed_str())
    }
}

impl<'a> From<Cow<'a, str>> for Str {
    fn from(s: Cow<'a, str>) -> Self {
        Str(s.to_string().into_boxed_str())
    }
}

impl FromIterator<char> for Str {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let s: String = iter.into_iter().collect();
        s.into()
    }
}

impl<'a> FromIterator<&'a str> for Str {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let s: String = iter.into_iter().collect();
        s.into()
    }
}

impl FromIterator<String> for Str {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let s: String = iter.into_iter().collect();
        s.into()
    }
}

impl From<Str> for Box<str> {
    fn from(s: Str) -> Self {
        s.0
    }
}

impl From<Str> for String {
    fn from(s: Str) -> Self {
        s.0.into()
    }
}

impl From<Str> for Cow<'_, str> {
    fn from(s: Str) -> Self {
        Cow::Owned(s.0.into())
    }
}

impl From<Str> for PathBuf {
    fn from(s: Str) -> Self {
        s.0.to_string().into()
    }
}

impl AsRef<str> for Str {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Debug for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl Deref for Str {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
