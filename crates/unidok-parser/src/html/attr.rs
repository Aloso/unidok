use detached_str::StrSlice;
use unidok_repr::ast::html::AttrAst;

use crate::utils::{ParseWs, QuotedString, Until};
use crate::{Indents, Input, Parse};

pub(crate) struct ParseAttribute<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseAttribute<'_> {
    type Output = AttrAst;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
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
        Some(AttrAst { key, value })
    }
}

pub(crate) struct ParseAttributes<'a> {
    pub ind: Indents<'a>,
}

impl Parse for ParseAttributes<'_> {
    type Output = Vec<AttrAst>;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut attrs = vec![];

        while !matches!(input.peek_char(), Some('>' | '/')) {
            attrs.push(input.parse(ParseAttribute { ind: self.ind })?);
        }

        Some(attrs)
    }
}

struct ParseAttrName;

impl Parse for ParseAttrName {
    type Output = StrSlice;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
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
