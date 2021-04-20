use crate::inlines::text::{parse_paragraph_items, stack_to_segments};
use crate::inlines::Segment;
use crate::utils::Indents;
use crate::{Context, Input, Parse};

use super::*;

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
        let (items, underline) = self.lex_paragraph_items(input)?;
        if items.is_empty() {
            return None;
        }

        let stack = parse_paragraph_items(items);
        let segments = stack_to_segments(stack);
        Some(Paragraph { segments, underline })
    }
}

impl ParseParagraph<'_> {
    pub(crate) fn can_parse_block(&self, input: &mut Input) -> bool {
        let ind = self.ind;
        input.can_parse(CodeBlock::parser(ind))
            || input.can_parse(Comment::parser(ind))
            || input.can_parse(Heading::parser(ind))
            || input.can_parse(Table::parser(ind))
            || input.can_parse(List::parser(ind))
            || input.can_parse(ThematicBreak::parser(ind))
            || input.can_parse(Quote::parser(ind))
    }
}
