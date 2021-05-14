use unidok_repr::ast::segments::Escaped;

use crate::utils::While;
use crate::{Input, Parse};

pub(crate) struct ParseEscaped;

impl Parse for ParseEscaped {
    type Output = Escaped;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();
        input.parse('\\')?;

        let c = input.peek_char()?;
        if !is_escapable_char(c) {
            return None;
        }
        input.bump(1);
        if matches!(c, '*' | '_' | '`' | '~' | '^' | '#') {
            input.parse_i(While(c));
        }

        let text = input.apply().get(1..);
        Some(Escaped { text })
    }
}

fn is_escapable_char(c: char) -> bool {
    matches!(c, '!'..='/' | ':'..='@' | '['..='`' | '{'..='~')
}
