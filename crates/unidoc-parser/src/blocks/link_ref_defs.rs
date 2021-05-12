use unidoc_repr::ast::blocks::LinkRefDef;

use crate::utils::{ParseLineBreak, ParseLineEnd, ParseSpacesU8, QuotedStringWithEscapes, Until};
use crate::{Indents, Input, Parse};

pub(crate) struct ParseLinkRefDef<'a> {
    pub ind: Indents<'a>,
}

impl Parse for ParseLinkRefDef<'_> {
    type Output = LinkRefDef;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.try_parse(ParseSpacesU8);
        input.parse('[')?;
        let name = input.parse_i(Until(|c| matches!(c, '\n' | '\r' | ']')));
        input.parse("]:")?;

        input.try_parse(ParseSpacesU8);
        let url = input.parse_i(Until(|c| matches!(c, '\n' | '\r' | '"' | '\'')));
        let url_trimmed = url.trim_end_matches(|c| matches!(c, ' ' | '\t'), input.text());

        if url_trimmed.is_empty() {
            return None;
        }

        let title = if url == url_trimmed {
            None
        } else {
            let title = input.parse(QuotedStringWithEscapes(self.ind));
            input.try_parse(ParseSpacesU8);
            title
        };

        input.parse(ParseLineEnd)?;
        input.try_parse(ParseLineBreak(self.ind));

        let lrd = LinkRefDef { name, url: url_trimmed, title };
        let name = input[name].to_string();
        input.state_mut().link_ref_defs.insert(name, lrd.clone());
        input.apply();
        Some(lrd)
    }
}
