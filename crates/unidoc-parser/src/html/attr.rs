use crate::parse::Parse;
use crate::StrSlice;

#[derive(Debug, Clone, PartialEq)]
pub struct HtmlAttr {
    pub key: StrSlice,
    pub value: Option<StrSlice>,
    pub quotes: AttrQuotes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttrQuotes {
    Double,
    Single,
    None,
}

impl HtmlAttr {
    fn parser() -> ParseAttribute {
        ParseAttribute
    }

    pub(crate) fn multi_parser() -> ParseAttributes {
        ParseAttributes
    }
}

pub(crate) struct ParseAttribute;

pub(crate) struct ParseAttributes;

impl Parse for ParseAttribute {
    type Output = HtmlAttr;

    fn parse(&self, _input: &mut crate::input::Input) -> Option<Self::Output> {
        todo!("HTML attributes are unimplemented")
    }
}

impl Parse for ParseAttributes {
    type Output = Vec<HtmlAttr>;

    fn parse(&self, input: &mut crate::input::Input) -> Option<Self::Output> {
        let mut attrs = vec![];

        while !matches!(input.peek_char(), Some('>' | '/')) {
            attrs.push(input.parse(HtmlAttr::parser())?);
        }

        Some(attrs)
    }
}
