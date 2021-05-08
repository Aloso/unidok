use crate::utils::Until;
use crate::{Parse, StrSlice};

#[derive(Debug, Clone, PartialEq)]
pub struct CDataSection {
    pub text: StrSlice,
}

impl CDataSection {
    pub(crate) fn parser() -> ParseCDataSection {
        ParseCDataSection
    }
}

pub(crate) struct ParseCDataSection;

impl Parse for ParseCDataSection {
    type Output = CDataSection;

    fn parse(&mut self, input: &mut crate::input::Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse("<![CDATA[")?;
        let text = input.parse_i(Until("]]>"));
        input.try_parse("]]>");

        input.apply();
        Some(CDataSection { text })
    }
}
