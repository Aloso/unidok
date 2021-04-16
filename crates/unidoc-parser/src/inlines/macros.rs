use crate::inlines::Braces;
use crate::str::StrSlice;
use crate::utils::Indents;
use crate::{Input, Parse, UntilChar};

#[derive(Debug, Clone, PartialEq)]
pub struct Macro {
    pub name: StrSlice,
    pub args: Option<StrSlice>,
    pub content: Braces,
}

impl Macro {
    pub(crate) fn parser(ind: Indents<'_>) -> ParseMacro<'_> {
        ParseMacro { ind }
    }
}

pub struct ParseMacro<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseMacro<'_> {
    type Output = Macro;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('@')?;
        let name = input.parse(ParseMacroName)?;
        let args = input.parse(ParseMacroArgs);
        let content = input.parse(Braces::parser(self.ind))?;

        input.apply();
        Some(Macro { name, args, content })
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
        let content = input.parse(UntilChar(|c| c == ')'))?;
        input.parse(')')?;
        input.apply();
        Some(content)
    }
}
