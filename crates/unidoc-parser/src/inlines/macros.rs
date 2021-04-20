use crate::inlines::Braces;
use crate::utils::{Indents, UntilChar};
use crate::{Input, Parse, StrSlice};

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct InlineMacro {
    pub name: StrSlice,
    pub args: Option<StrSlice>,
    pub segments: Box<Segment>,
}

impl InlineMacro {
    pub(crate) fn parser(ind: Indents<'_>) -> ParseInlineMacro<'_> {
        ParseInlineMacro { ind }
    }
}

pub struct ParseInlineMacro<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseInlineMacro<'_> {
    type Output = InlineMacro;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('@')?;
        let name = input.parse(ParseMacroName)?;
        let args = input.parse(ParseMacroArgs);

        let segments = if let Some(braces) = input.parse(Braces::parser(self.ind)) {
            Segment::Braces(braces)
        } else if let Some(code) = {
            let pass = name.to_str(input.text()) == "PASS";
            input.parse(Code::parser(self.ind, pass))
        } {
            Segment::Code(code)
        } else if let Some(mac) = input.parse(InlineMacro::parser(self.ind)) {
            Segment::InlineMacro(mac)
        } else if let Some(img) = input.parse(Image::parser(self.ind)) {
            Segment::Image(img)
        } else if let Some(link) = input.parse(Link::parser(self.ind)) {
            Segment::Link(link)
        } else if let Some(math) = input.parse(Math::parser(self.ind)) {
            Segment::Math(math)
        } else {
            return None;
        };
        let segments = Box::new(segments);

        input.apply();
        Some(InlineMacro { name, args, segments })
    }
}

pub(crate) struct ParseMacroName;

impl Parse for ParseMacroName {
    type Output = StrSlice;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
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

pub(crate) struct ParseMacroArgs;

impl Parse for ParseMacroArgs {
    type Output = StrSlice;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();
        input.parse('(')?;
        let content = input.parse_i(UntilChar(|c| c == ')'));
        input.parse(')')?;
        input.apply();
        Some(content)
    }
}
