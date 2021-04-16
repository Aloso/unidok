use crate::str::StrSlice;
use crate::{Input, Parse, WhileChar};

/// Text escaped with the escape character, `\`. All ASCII characters that are
/// visible and not whitespace or alphanumeric can be escaped:
///
/// ```text
/// ! " # $ % & ' ( ) * + , - . / : ; < = > ? @ [ \ ] ^ _ ` { | } ~
/// ```
///
/// A single backslash applies to the next ASCII character that fulfills the
/// above criteria. Exceptions are inline formatting characters
/// (`` * _ ` ~ ^ # ``): When a backslash is followed by one of these
/// characters, it can escape an arbitrary number of the same consecutive
/// character.
///
/// Example: `\***text\***` results in all 6 stars to be displayed as-is. You
/// can prevent this by inserting a limiter (`$`): `\*$**text**` means that only
/// one star is displayed, followed by bold text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Escaped {
    pub text: StrSlice,
}

impl Escaped {
    pub(crate) fn parser() -> ParseEscape {
        ParseEscape
    }
}

pub(crate) struct ParseEscape;

impl Parse for ParseEscape {
    type Output = Escaped;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();
        input.parse('\\')?;

        let c = input.peek_char()?;
        if !is_escapable_char(c) {
            return None;
        }
        input.bump(1);
        if matches!(c, '*' | '_' | '`' | '~' | '^') {
            input.parse(WhileChar(c))?;
        }

        let text = input.apply().get(1..);
        Some(Escaped { text })
    }
}

fn is_escapable_char(c: char) -> bool {
    matches!(c, '!'..='/' | ':'..='@' | '['..='`' | '{'..='~')
}
