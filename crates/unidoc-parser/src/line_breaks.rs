use std::collections::VecDeque;

use crate::indent::{Indentation, Indents, ParseQuoteIndent, ParseSpacesIndent};
use crate::{Input, Parse};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineBreak;

impl LineBreak {
    pub fn parser(ind: Indents<'_>) -> ParseLineBreak<'_> {
        ParseLineBreak { ind }
    }
}

/// Parses a line break, including left padding
#[derive(Debug, Clone, Copy)]
pub struct ParseLineBreak<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseLineBreak<'_> {
    type Output = LineBreak;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('\n')?;

        if !matches!(input.peek_char(), Some('\n') | None) {
            let mut stack = VecDeque::with_capacity(8);
            let mut acc = self.ind.root;
            while let INode::Node { next, ind } = acc {
                stack.push_front(ind);
                acc = *next;
            }

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
        }

        input.set_line_start(true);
        input.apply();
        Some(LineBreak)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(super) enum INode<'a> {
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
        Indents { root: INode::Node { ind, next: &self.root } }
    }
}
