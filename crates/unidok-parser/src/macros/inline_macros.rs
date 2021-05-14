use unidok_repr::ast::html::HtmlNode;
use unidok_repr::ast::macros::InlineMacro;
use unidok_repr::ast::segments::Segment;

use crate::html::elem::ParseHtmlElem;
use crate::inlines::braces::ParseBraces;
use crate::inlines::code::ParseCode;
use crate::inlines::images::ParseImage;
use crate::inlines::links::ParseLink;
use crate::inlines::math::ParseMath;
use crate::parsing_mode::ParsingMode;
use crate::{Indents, Input, Parse};

use super::args::ParseMacroArgs;
use super::utils::{get_parsing_mode, ParseMacroName};

pub(crate) struct ParseInlineMacro<'a> {
    pub ind: Indents<'a>,
    pub mode: Option<ParsingMode>,
}

impl Parse for ParseInlineMacro<'_> {
    type Output = InlineMacro;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('@')?;
        let name = input.parse(ParseMacroName)?;
        let name_str = name.to_str(input.text()).to_string();
        let args = input.parse(ParseMacroArgs { name: &name_str, ind: self.ind })?;

        if name.is_empty() && args.is_none() {
            return None;
        }

        let mode = get_parsing_mode(&name_str, &args, &input)?.or(self.mode);

        let segment = if let Some(braces) = input.parse(ParseBraces { ind: self.ind }) {
            Segment::Braces(braces)
        } else if let Some(code) = input.parse(ParseCode { ind: self.ind, mode }) {
            Segment::Code(code)
        } else if let Some(mac) = input.parse(ParseInlineMacro { ind: self.ind, mode }) {
            Segment::InlineMacro(mac)
        } else if let Some(img) = input.parse(ParseImage { ind: self.ind }) {
            Segment::Image(img)
        } else if let Some(link) = input.parse(ParseLink { ind: self.ind }) {
            Segment::Link(link)
        } else if let Some(math) = input.parse(ParseMath { ind: self.ind }) {
            Segment::Math(math)
        } else if let Some(elem) = input.parse(ParseHtmlElem { ind: self.ind }) {
            Segment::InlineHtml(HtmlNode::Element(elem))
        } else {
            return None;
        };
        let segment = Box::new(segment);

        input.apply();
        Some(InlineMacro { name, args, segment })
    }
}
