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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Indents<'a> {
    pub(super) root: INode<'a>,
}
