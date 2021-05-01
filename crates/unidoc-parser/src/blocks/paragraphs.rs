use crate::inlines::segments::Segments;
use crate::inlines::Segment;
use crate::parsing_mode::ParsingMode;
use crate::utils::Indents;
use crate::{Context, Input, Parse};

use super::*;

/// A paragraph
///
/// Paragraphs can be interrupted by ATX-style headings, lists, quotes, tables,
/// code blocks, thematic breaks that don't consist of dashes and line comments.
#[derive(Debug, Clone, PartialEq)]
pub struct Paragraph {
    pub segments: Vec<Segment>,
    pub underline: Option<Underline>,
}

pub(crate) struct ParseParagraph<'a> {
    pub(crate) ind: Indents<'a>,
    pub(crate) context: Context,
}

impl Paragraph {
    pub(crate) fn parser(ind: Indents<'_>, context: Context) -> ParseParagraph<'_> {
        ParseParagraph { ind, context }
    }
}

impl Parse for ParseParagraph<'_> {
    type Output = Paragraph;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let parser = Segments::parser(self.ind, self.context, ParsingMode::Everything);

        match input.parse(parser)? {
            Segments::Empty => None,
            Segments::Some { segments, underline } => Some(Paragraph { segments, underline }),
        }
    }
}
