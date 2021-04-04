use std::collections::VecDeque;
use std::num::NonZeroU8;

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

pub(super) struct ParseSpacesIndent(NonZeroU8);

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Indents<'a>(INode<'a>);

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

impl<'a> Indents<'a> {
    pub fn push(&'a self, ind: Indentation) -> Self {
        Indents(INode::Node { ind, next: &self.0 })
    }
}

/// Parses a line break, including left padding
pub struct LineBreak<'a>(pub Indents<'a>);

impl Parse for LineBreak<'_> {
    type Output = ();

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('\n')?;

        let mut stack = VecDeque::with_capacity(8);
        let mut acc = self.0 .0;
        while let INode::Node { next, ind } = acc {
            stack.push_front(ind);
            acc = *next;
        }
        dbg!(&stack);

        for ind in stack {
            match ind {
                Indentation::Spaces(s) => {
                    input.parse(ParseSpacesIndent(s))?;
                }
                Indentation::Quote => {
                    input.parse(ParseQuoteIndent)?;
                }
            }
        }

        input.set_line_start(true);
        input.apply();
        Some(())
    }
}
