use std::num::NonZeroU8;

use super::{Or, ParseAtMostNSpaces, ParseLineEnd};
use crate::{Input, Parse};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Indentation {
    Spaces(NonZeroU8),
    QuoteMarker,
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

    pub fn push_quote(&'a self) -> Self {
        Indents { root: INode::Node { ind: Indentation::QuoteMarker, next: &self.root } }
    }

    pub fn push_indent(&'a self, spaces: u8) -> Self {
        match NonZeroU8::new(spaces) {
            Some(ind) => {
                Indents { root: INode::Node { ind: Indentation::Spaces(ind), next: &self.root } }
            }
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

            input.parse(Or('\n', Or("\r\n", '\r')))?;

            if let State::Error = parse_indentation_rec(self.0.root, &mut input) {
                return None;
            }

            input.apply();
        }

        Some(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum INode<'a> {
    Node { ind: Indentation, next: &'a INode<'a> },
    Tail,
}

impl Default for INode<'_> {
    fn default() -> Self {
        INode::Tail
    }
}

enum State {
    Continue,
    Done,
    Error,
}

fn parse_indentation_rec(node: INode<'_>, input: &mut Input) -> State {
    match node {
        INode::Node { ind, next } => {
            let prev_state = parse_indentation_rec(*next, input);
            if let State::Continue = prev_state {
                match ind {
                    Indentation::Spaces(s) => {
                        let s: u8 = s.into();
                        match input.parse(ParseAtMostNSpaces(s)) {
                            Some(n) if n == s => State::Continue,
                            Some(_) if input.can_parse(ParseLineEnd) => State::Done,
                            _ => State::Error,
                        }
                    }
                    Indentation::QuoteMarker => match input.parse(ParseQuoteMarker) {
                        None if input.can_parse(ParseLineEnd) => State::Done,
                        Some(_) => State::Continue,
                        _ => State::Error,
                    },
                }
            } else {
                prev_state
            }
        }
        INode::Tail => State::Continue,
    }
}

#[test]
fn test_no_indentation() {
    let no_ind = Indents::new();

    parse!("\n", ParseLineBreak(no_ind), ());
    parse!("", ParseLineBreak(no_ind), ());
    parse!("x", ParseLineBreak(no_ind), None);
    parse!(" ", ParseLineBreak(no_ind), None);
}

#[test]
fn test_spaces_indentation() {
    let no_ind = Indents::new();
    let two_ind = no_ind.push_indent(2);

    parse!("", ParseLineBreak(two_ind), ());
    parse!("  ", ParseLineBreak(two_ind), None);
    parse!("\n", ParseLineBreak(two_ind), ());
    parse!("\n ", ParseLineBreak(two_ind), ());
    parse!("\n  ", ParseLineBreak(two_ind), ());
    parse!("\n   ", ParseLineBreak(two_ind), ());
    parse!("\n\n", ParseLineBreak(two_ind), ());
    parse!("\n \n", ParseLineBreak(two_ind), ());
    parse!("\n  \n", ParseLineBreak(two_ind), ());
    parse!("\n >", ParseLineBreak(two_ind), None);
    parse!("\n  >", ParseLineBreak(two_ind), ());
    parse!("\n   >", ParseLineBreak(two_ind), ());
    parse!("\r\n  >", ParseLineBreak(two_ind), ());
    parse!("\r  >", ParseLineBreak(two_ind), ());
}

#[test]
fn test_space_and_quote_indentation() {
    let no_ind = Indents::new();
    let two_ind = no_ind.push_indent(2);
    let three_ind = two_ind.push_quote();

    parse!(">", ParseLineBreak(three_ind), None);
    parse!("\n", ParseLineBreak(three_ind), ());
    parse!("\n  ", ParseLineBreak(three_ind), ());
    parse!("\n  > X\n", ParseLineBreak(three_ind), ());
    parse!("\n  > \n", ParseLineBreak(three_ind), ());
    parse!("\n  >\n", ParseLineBreak(three_ind), ());
    parse!("\n  \n", ParseLineBreak(three_ind), ());
    parse!("\n \n", ParseLineBreak(three_ind), ());
    parse!("\n\n", ParseLineBreak(three_ind), ());
    parse!("\n  X\n", ParseLineBreak(three_ind), None);
}

#[test]
fn test_quote_indentation() {
    let no_ind = Indents::new();
    let one_ind = no_ind.push_quote();

    parse!("", ParseLineBreak(one_ind), ());
    parse!(">", ParseLineBreak(one_ind), None);
    parse!("\n", ParseLineBreak(one_ind), ());
    parse!("\n>", ParseLineBreak(one_ind), ());
    parse!("\n> ", ParseLineBreak(one_ind), ());
    parse!("\n\n", ParseLineBreak(one_ind), ());
    parse!("\n>\n", ParseLineBreak(one_ind), ());
    parse!("\n> \n", ParseLineBreak(one_ind), ());
}

#[test]
fn test_quote_and_space_indentation() {
    let no_ind = Indents::new();
    let one_ind = no_ind.push_quote();
    let two_ind = one_ind.push_indent(1);

    parse!("", ParseLineBreak(two_ind), ());
    parse!("> ", ParseLineBreak(two_ind), None);
    parse!("\n", ParseLineBreak(two_ind), ());
    parse!("\n>", ParseLineBreak(two_ind), ());
    parse!("\n> ", ParseLineBreak(two_ind), ());
    parse!("\n>  ", ParseLineBreak(two_ind), ());
    parse!("\n\n", ParseLineBreak(two_ind), ());
    parse!("\n>\n", ParseLineBreak(two_ind), ());
    parse!("\n> \n", ParseLineBreak(two_ind), ());
    parse!("\n>>", ParseLineBreak(two_ind), None);
    parse!("\n> >", ParseLineBreak(two_ind), ());
    parse!("\n>  >", ParseLineBreak(two_ind), ());
}
