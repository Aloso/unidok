use crate::marker::{ParseLineEnd, ParseLineStart};
use crate::str::StrSlice;
use crate::{Input, Parse, WhileChar};

/// The escape character, `\`.
#[derive(Debug, Clone)]
pub struct Escaped {
    pub line_start: bool,
    pub text: StrSlice,
}

impl Escaped {
    pub fn parser() -> ParseEscape {
        ParseEscape
    }
}

pub struct ParseEscape;

impl Parse for ParseEscape {
    type Output = Escaped;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let line_start = input.parse(ParseLineStart).is_some();
        let mut input = input.start();
        input.parse('\\')?;
        let rest = input.rest();

        let text = if two_escapable_inline_chars(rest) {
            input.bump(2)
        } else if input.peek_char().map(escapable_inline_char) == Some(true) {
            input.bump(1)
        } else if line_start {
            if rest.starts_with('#') {
                input.parse(WhileChar('#'))?
            } else if rest.starts_with("//") {
                input.bump(2)
            } else if rest.starts_with("|===") {
                let mut input2 = input.start();
                input2.parse('|')?;
                input2.parse(WhileChar('='))?;
                input2.parse(ParseLineEnd)?;
                input2.apply()
            } else {
                return None;
            }
        } else {
            return None;
        };

        input.apply();
        Some(Escaped { line_start, text })
    }
}

fn two_escapable_inline_chars(rest: &str) -> bool {
    rest.starts_with("**") || rest.starts_with("__") || rest.starts_with("~~")
}

fn escapable_inline_char(c: char) -> bool {
    matches!(
        c,
        '[' | '{' | '}' | '<' | '\\' | '*' | '_' | '~' | '`' | '^' | '$' | '%' | '@'
    )
}
