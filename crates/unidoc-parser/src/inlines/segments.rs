use super::*;
use crate::utils::Indents;
use crate::Parse;

#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    LineBreak(LineBreak),
    Text(Text),
    Escaped(Escaped),
    Limiter(Limiter),
    Braces(Braces),
    Math(Math),
    Link(Link),
    Image(Image),
    Macro(Macro),
    InlineFormat(InlineFormat),
}

// TODO: Think about braces *WITHOUT* macro, i.e.
//
// {some {text} and %{math} and @{macro content}!}

impl Segment {
    pub(crate) fn parser(context: SegmentCtx, ind: Indents<'_>) -> ParseSegment<'_> {
        ParseSegment { ind, context }
    }

    pub(crate) fn multi_parser(context: SegmentCtx, ind: Indents<'_>) -> ParseSegments<'_> {
        ParseSegments { ind, context }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ParseSegment<'a> {
    ind: Indents<'a>,
    context: SegmentCtx,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ParseSegments<'a> {
    ind: Indents<'a>,
    context: SegmentCtx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentCtx {
    Table,
    Braces,
    LinkOrImg,
    Other,
}

impl Parse for ParseSegment<'_> {
    type Output = Segment;

    fn parse(&self, input: &mut crate::Input) -> Option<Self::Output> {
        if let Some(esc) = input.parse(Escaped::parser()) {
            Some(Segment::Escaped(esc))
        } else if let Some(limiter) = input.parse(Limiter::parser()) {
            Some(Segment::Limiter(limiter))
        } else if let Some(block) = input.parse(Braces::parser(self.ind)) {
            Some(Segment::Braces(block))
        } else if let Some(math) = input.parse(Math::parser(self.ind)) {
            Some(Segment::Math(math))
        } else if let Some(text) = input.parse(Text::parser()) {
            Some(Segment::Text(text))
        } else if let Some(mac) = input.parse(Macro::parser(self.ind)) {
            Some(Segment::Macro(mac))
        } else if !input.is_empty() {
            match input.peek_char().unwrap() {
                ']' if self.context == SegmentCtx::LinkOrImg => None,
                '|' if self.context == SegmentCtx::Table => None,
                '}' if self.context == SegmentCtx::Braces => None,
                '\n' => None,
                c => Some(Segment::Text(Text(input.bump(c.len_utf8() as usize)))),
            }
        } else {
            None
        }
    }
}

impl Parse for ParseSegments<'_> {
    type Output = Vec<Segment>;

    fn parse(&self, input: &mut crate::Input) -> Option<Self::Output> {
        let parser = Segment::parser(self.context, self.ind);

        let mut v = Vec::new();
        while let Some(node) = input.parse(parser) {
            v.push(node);
        }
        Some(v)
    }
}
