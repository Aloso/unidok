use crate::inlines::Braces;
use crate::parsing_mode::ParsingMode;
use crate::utils::{Indents, Until};
use crate::{Input, Parse, StrSlice};

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct InlineMacro {
    pub name: StrSlice,
    pub args: Option<StrSlice>,
    pub segment: Box<Segment>,
}

impl InlineMacro {
    pub(crate) fn parser(ind: Indents<'_>, mode: ParsingMode) -> ParseInlineMacro<'_> {
        ParseInlineMacro { ind, mode }
    }
}

pub struct ParseInlineMacro<'a> {
    ind: Indents<'a>,
    mode: ParsingMode,
}

impl Parse for ParseInlineMacro<'_> {
    type Output = InlineMacro;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('@')?;
        let name = input.parse(ParseMacroName)?;
        let args = input.parse(ParseMacroArgs);

        let parsing_mode = ParsingMode::try_from_macro(
            name.to_str(input.text()),
            args.map(|s| s.to_str(input.text())),
        )
        .unwrap_or(self.mode);

        let segment = if let Some(braces) = input.parse(Braces::parser(self.ind)) {
            Segment::Braces(braces)
        } else if let Some(code) = input.parse(Code::parser(self.ind, parsing_mode)) {
            Segment::Code(code)
        } else if let Some(mac) = input.parse(InlineMacro::parser(self.ind, parsing_mode)) {
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
        let segment = Box::new(segment);

        input.apply();
        Some(InlineMacro { name, args, segment })
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
        // TODO: Parse quotes
        let content = input.parse_i(Until(')'));
        input.parse(')')?;
        input.apply();
        Some(content)
    }
}
