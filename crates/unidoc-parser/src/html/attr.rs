use crate::input::Input;
use crate::parse::Parse;
use crate::utils::{Indents, ParseWs, QuotedString, Until};
use crate::StrSlice;

#[derive(Debug, Clone, PartialEq)]
pub struct HtmlAttr {
    pub key: StrSlice,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttrQuotes {
    Double,
    Single,
    None,
}

impl HtmlAttr {
    fn parser(ind: Indents<'_>) -> ParseAttribute<'_> {
        ParseAttribute { ind }
    }

    pub(crate) fn multi_parser(ind: Indents<'_>) -> ParseAttributes<'_> {
        ParseAttributes { ind }
    }
}

pub(crate) struct ParseAttribute<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseAttribute<'_> {
    type Output = HtmlAttr;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        let key = input.parse(ParseAttrName)?;
        input.parse_i(ParseWs(self.ind));

        let value = if input.parse('=').is_some() {
            input.parse_i(ParseWs(self.ind));

            let value = input.parse(QuotedString(self.ind)).or_else(|| {
                input.parse(ParseAttrName).map(|s| s.to_str(input.text()).to_string())
            })?;

            Some(value)
        } else {
            None
        };

        input.apply();
        Some(HtmlAttr { key, value })
    }
}

pub(crate) struct ParseAttributes<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseAttributes<'_> {
    type Output = Vec<HtmlAttr>;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut attrs = vec![];

        while !matches!(input.peek_char(), Some('>' | '/')) {
            attrs.push(input.parse(HtmlAttr::parser(self.ind))?);
        }

        Some(attrs)
    }
}

struct ParseAttrName;

impl Parse for ParseAttrName {
    type Output = StrSlice;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let s = input.parse_i(Until(|c| {
            matches!(c, ' ' | '\t' | '\r' | '\n' | '"' | '\'' | '>' | '<' | '=')
        }));
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }
}
