use super::*;
use crate::containers::*;
use crate::inlines::{LineBreak, Segment, SegmentCtx};
use crate::utils::{Indents, ParseLineBreak, ParseSpaces};
use crate::{Input, NodeCtx, Parse};

#[derive(Debug, Clone, PartialEq)]
pub struct Paragraph {
    pub segments: Vec<Segment>,
}

pub(crate) struct ParseParagraph<'a> {
    ind: Indents<'a>,
    context: NodeCtx,
}

impl Paragraph {
    pub(crate) fn parser(ind: Indents<'_>, context: NodeCtx) -> ParseParagraph<'_> {
        ParseParagraph { ind, context }
    }
}

impl Parse for ParseParagraph<'_> {
    type Output = Paragraph;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let context = match self.context {
            NodeCtx::Braces => SegmentCtx::Braces,
            NodeCtx::ContainerOrGlobal => SegmentCtx::Other,
        };
        let parser = Segment::parser(context, self.ind);

        let mut segments = Vec::new();
        loop {
            if let Some(segment) = input.parse(parser) {
                dbg!(&segment);
                segments.push(segment);
            } else {
                if input.parse(ParseLineBreak(self.ind)).is_some() {
                    let mut input2 = input.start();
                    let offset = input2.parse(ParseSpaces)?;
                    let ind = self.ind.push_indent(offset);

                    if !can_parse_block(&mut input2, ind) {
                        input2.apply();
                        if segments.last() == Some(&Segment::LineBreak(LineBreak)) {
                            break;
                        }
                        segments.push(Segment::LineBreak(LineBreak));
                        continue;
                    }
                }
                break;
            }
        }

        if segments.is_empty() {
            None
        } else {
            Some(Paragraph { segments })
        }
    }
}

fn can_parse_block(input: &mut Input, ind: Indents) -> bool {
    input.can_parse(CodeBlock::parser(ind))
        || input.can_parse(Comment::parser(ind))
        || input.can_parse(Heading::parser(ind))
        || input.can_parse(ThematicBreak::parser(ind))
        || input.can_parse(Table::parser(ind))
        || input.can_parse(List::parser(ind))
        || input.can_parse(Quote::parser(ind))
}