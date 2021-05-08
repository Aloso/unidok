use crate::utils::{AsciiCI, ClosingQuoteMark, ParseOneWS, ParseSpaces, QuoteMark};
use crate::{Parse, StrSlice};

#[derive(Debug, Clone, PartialEq)]
pub struct Doctype {
    pub text: StrSlice,
}

impl Doctype {
    pub(crate) fn parser() -> ParseDoctype {
        ParseDoctype
    }
}

pub(crate) struct ParseDoctype;

impl Parse for ParseDoctype {
    type Output = Doctype;

    fn parse(&mut self, input: &mut crate::input::Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse("<!")?;
        input.parse(AsciiCI("doctype"))?;
        input.parse(ParseOneWS)?;
        input.parse_i(ParseSpaces);
        input.parse(AsciiCI("html"))?;
        input.try_parse(ParseLegacyString);
        input.parse_i(ParseSpaces);
        input.parse('>')?;

        Some(Doctype { text: input.apply() })
    }
}

struct ParseLegacyString;

impl Parse for ParseLegacyString {
    type Output = ();

    fn parse(&mut self, input: &mut crate::input::Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse(ParseOneWS)?;
        input.parse_i(ParseSpaces);
        input.parse(AsciiCI("system"))?;
        input.parse(ParseOneWS)?;
        input.parse_i(ParseSpaces);
        let q = input.parse(QuoteMark)?;
        input.parse("about:legacy-compat")?;
        input.parse(ClosingQuoteMark(q))?;

        input.apply();
        Some(())
    }
}
