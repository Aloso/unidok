use crate::blocks::macros::get_parsing_mode;
use crate::html::{HtmlElem, HtmlNode};
use crate::inlines::Braces;
use crate::macros::MacroArgs;
use crate::parsing_mode::ParsingMode;
use crate::utils::Indents;
use crate::{Input, Parse, StrSlice};

use super::*;

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

pub(crate) struct ParseMacroName;

impl Parse for ParseMacroName {
    type Output = StrSlice;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        fn is_macro_char(c: char) -> bool {
            c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'
        }

        if let Some(mat) = input.rest().find(|c| !is_macro_char(c)) {
            let rest = &input.rest()[mat..];
            if rest.starts_with(char::is_alphanumeric) {
                None
            } else {
                let len = input.len() - rest.len();
                Some(input.bump(len))
            }
        } else {
            Some(input.bump(input.len()))
        }
    }
}
