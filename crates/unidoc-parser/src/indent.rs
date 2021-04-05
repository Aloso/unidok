use std::num::NonZeroU8;

use crate::line_breaks::INode;
use crate::{Input, Parse};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Indentation {
    Spaces(NonZeroU8),
    Quote,
}

impl Indentation {
    pub fn spaces(n: u8) -> Self {
        Indentation::Spaces(NonZeroU8::new(n).unwrap())
    }
}

pub(super) struct ParseSpacesIndent(pub(super) NonZeroU8);

pub(super) struct ParseQuoteIndent;

impl Parse for ParseSpacesIndent {
    type Output = ();

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        const SPACES: &str = "                                                                                                                                                                                                                                                                ";
        let spaces = &SPACES[..u8::from(self.0) as usize];
        input.parse(spaces)?;
        Some(())
    }
}

impl Parse for ParseQuoteIndent {
    type Output = ();

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        input.parse("> ")?;
        Some(())
    }
}

/// This type contains an immutable linked list of indentation nodes.
///
/// Each indentation node ([`INode`]) represents one level of indentation. The
/// nodes are stored on the stack, and are linked together with immutable
/// references, so you can push items at the front, but can't mutate existing
/// items, and you can't return an [Indents] object from a function that owns
/// one of its items. However, this is never required in parsers.
///
/// This type is perfect for this purpose because it is quite memory efficient
/// and performant, it is immutable, and it implements `Copy`. The downside is
/// that it can't be iterated with the [Iterator] trait, instead we use
/// recursive algorithms like we would do in functional programming languages.
///
/// A level of indentation can either be
///
/// - Indentation of _n_ spaces
/// - Quote indentation (The `>` character must be repeated in every line at the
///   correct indentation level)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Indents<'a> {
    pub(super) root: INode<'a>,
}

impl Indents<'_> {
    pub fn new() -> Self {
        Indents { root: INode::Tail }
    }
}
