use crate::html::{HtmlElem, HtmlNode};
use crate::inlines::*;
use crate::parsing_mode::ParsingMode;
use crate::{Indents, Input, Parse, StrSlice};

use super::utils::{get_parsing_mode, ParseMacroName};
use super::MacroArgs;

#[derive(Debug, Clone, PartialEq)]
pub struct InlineMacro {
    pub name: StrSlice,
    pub args: Option<MacroArgs>,
    pub segment: Box<Segment>,
}

impl InlineMacro {
    pub(crate) fn parser(ind: Indents<'_>, mode: Option<ParsingMode>) -> ParseInlineMacro<'_> {
        ParseInlineMacro { ind, mode }
    }
}

pub struct ParseInlineMacro<'a> {
    ind: Indents<'a>,
    mode: Option<ParsingMode>,
}

impl Parse for ParseInlineMacro<'_> {
    type Output = InlineMacro;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('@')?;
        let name = input.parse(ParseMacroName)?;
        let name_str = name.to_str(input.text()).to_string();
        let args = input.parse(MacroArgs::parser(&name_str, self.ind))?;

        if name.is_empty() && args.is_none() {
            return None;
        }

        let mode = get_parsing_mode(&name_str, &args, &input)?.or(self.mode);

        let segment = if let Some(braces) = input.parse(Braces::parser(self.ind)) {
            Segment::Braces(braces)
        } else if let Some(code) = input.parse(Code::parser(self.ind, mode)) {
            Segment::Code(code)
        } else if let Some(mac) = input.parse(InlineMacro::parser(self.ind, mode)) {
            Segment::InlineMacro(mac)
        } else if let Some(img) = input.parse(Image::parser(self.ind)) {
            Segment::Image(img)
        } else if let Some(link) = input.parse(Link::parser(self.ind)) {
            Segment::Link(link)
        } else if let Some(math) = input.parse(Math::parser(self.ind)) {
            Segment::Math(math)
        } else if let Some(elem) = input.parse(HtmlElem::parser(self.ind)) {
            Segment::InlineHtml(HtmlNode::Element(elem))
        } else {
            return None;
        };
        let segment = Box::new(segment);

        input.apply();
        Some(InlineMacro { name, args, segment })
    }
}
