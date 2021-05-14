use unidok_repr::ast::html::CDataSection;

use crate::utils::Until;
use crate::Parse;

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
