use crate::indent::Indents;
use crate::inlines::{Segment, SegmentCtx};
use crate::items::NodeCtx;
use crate::Parse;

#[derive(Debug, Clone)]
pub struct Line {
    pub segments: Vec<Segment>,
}

pub(crate) struct ParseLine<'a> {
    ind: Indents<'a>,
    context: NodeCtx,
}

impl Line {
    pub(crate) fn parser(ind: Indents<'_>, context: NodeCtx) -> ParseLine<'_> {
        ParseLine { ind, context }
    }
}

impl Parse for ParseLine<'_> {
    type Output = Line;

    fn parse(&self, input: &mut crate::Input) -> Option<Self::Output> {
        let context = match self.context {
            NodeCtx::Braces => SegmentCtx::Braces,
            NodeCtx::ContainerOrGlobal => SegmentCtx::Other,
        };
        let segments = input.parse(Segment::multi_parser(context, self.ind))?;

        Some(Line { segments })
    }
}
