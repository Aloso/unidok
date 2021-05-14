use unidok_repr::ast::html::ElemName;

use crate::utils::While;
use crate::Parse;

pub(crate) struct ParseElemName;

impl Parse for ParseElemName {
    type Output = ElemName;

    fn parse(&mut self, input: &mut crate::input::Input) -> Option<Self::Output> {
        let mut input = input.start();

        let name =
            input.parse_i(While(|c: char| c.is_ascii_alphanumeric() || matches!(c, '-' | '_')));
        if name.is_empty() {
            return None;
        }
        let next = input.peek_char();
        if !matches!(next, Some(' ' | '\t' | '\r' | '\n' | '/' | '>')) {
            return None;
        }

        let elem_name = ElemName::try_from(name.to_str(input.text()))?;

        input.apply();
        Some(elem_name)
    }
}
