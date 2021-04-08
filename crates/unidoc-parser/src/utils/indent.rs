use std::num::NonZeroU8;

use super::ParseNSpaces;
use crate::{Input, Parse};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Indentation {
    Spaces(NonZeroU8),
    QuoteMarker,
}

impl Indentation {
    pub(crate) fn spaces(n: u8) -> Self {
        Indentation::Spaces(NonZeroU8::new(n).unwrap())
    }
}

pub(crate) struct ParseQuoteMarker;

impl Parse for ParseQuoteMarker {
    type Output = ();

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        input.parse('>')?;
        Some(())
    }

    fn can_parse(&self, input: &mut Input) -> bool {
        input.rest().starts_with('>')
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
    root: INode<'a>,
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

/// Parses a line break, including indentation (whitespace and quote markers) on
/// the next line, if present.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ParseLineBreak<'a>(pub Indents<'a>);

impl Parse for ParseLineBreak<'_> {
    type Output = ();

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        if !input.is_empty() {
            let mut input = input.start();

            input.parse('\n')?;

            if !matches!(input.peek_char(), Some('\n') | None) {
                parse_recursive(self.0.root, &mut input)?;
            }

            input.apply();
        }

        Some(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum INode<'a> {
    Node { ind: Indentation, next: &'a INode<'a> },
    Tail,
}

impl Default for INode<'_> {
    fn default() -> Self {
        INode::Tail
    }
}

fn parse_recursive(node: INode<'_>, input: &mut Input) -> Option<()> {
    match node {
        INode::Node { ind, next } => {
            parse_recursive(*next, input)?;
            match ind {
                Indentation::Spaces(s) => input.parse(ParseNSpaces(s.into())),
                Indentation::QuoteMarker => input.parse(ParseQuoteMarker),
            }
        }
        INode::Tail => Some(()),
    }
}
