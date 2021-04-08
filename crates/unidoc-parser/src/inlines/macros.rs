use crate::inlines::Braces;
use crate::str::StrSlice;
use crate::utils::Indents;
use crate::{Input, Parse, UntilChar};

#[derive(Debug, Clone, PartialEq)]
pub struct Macro {
    pub name: StrSlice,
    pub args: Option<StrSlice>,
    pub content: Option<Braces>,
}

impl Macro {
    pub fn parser(ind: Indents<'_>) -> ParseMacro<'_> {
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
        let content = input.parse(Braces::parser(self.ind));

        input.apply();
        Some(Macro { name, args, content })
    }
}

struct ParseMacroName;

impl Parse for ParseMacroName {
    type Output = StrSlice;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        fn is_macro_char(c: char) -> bool {
            c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'
        }

        match input.rest().find(|c| !is_macro_char(c)) {
            Some(0) => None,
            Some(no_match) => {
                let rest = &input.rest()[no_match..];
                if rest.starts_with(|c: char| c.is_alphanumeric()) {
                    None
                } else {
                    let len = input.len() - rest.len();
                    Some(input.bump(len))
                }
            }
            None if input.is_empty() => None,
            None => Some(input.bump(input.len())),
        }
    }
}

struct ParseMacroArgs;

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
