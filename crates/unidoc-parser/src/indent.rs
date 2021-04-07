use std::num::NonZeroU8;

use crate::line_breaks::INode;
use crate::{Input, Parse};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Indentation {
    Spaces(NonZeroU8),
    QuoteMarker,
}

impl Indentation {
    pub fn spaces(n: u8) -> Self {
        Indentation::Spaces(NonZeroU8::new(n).unwrap())
    }
}

pub struct ParseSpaces;

pub struct ParseNSpaces(pub u8);

pub struct ParseQuoteMarker;

impl Parse for ParseSpaces {
    type Output = u8;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut res = 0;
        let mut len = 0;
        let rest = input.rest();
        for c in rest.chars() {
            match c {
                ' ' => {
                    res += 1;
                    len += 1;
                }
                '\t' => {
                    res += 4;
                    len += 1;
                }
                _ => break,
            }
        }
        input.bump(len);
        Some(res)
    }
}

impl Parse for ParseNSpaces {
    type Output = ();

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        const SPACES: &str = "                                                                                                                                                                                                                                                                ";
        let spaces = &SPACES[..self.0 as usize];
        input.parse(spaces)?;
        Some(())
    }
}

impl Parse for ParseQuoteMarker {
    type Output = ();

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        input.parse('>')?;
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

impl<'a> Indents<'a> {
    pub fn new() -> Self {
        Indents { root: INode::Tail }
    }

    pub fn push(&'a self, ind: Indentation) -> Self {
        Indents { root: INode::Node { ind, next: &self.root } }
    }

    pub fn indent(&'a self, spaces: u8) -> Self {
        match NonZeroU8::new(spaces) {
            Some(ind) => Indents {
                root: INode::Node { ind: Indentation::Spaces(ind), next: &self.root },
            },
            None => *self,
        }
    }
}
