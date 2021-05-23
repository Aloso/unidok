use detached_str::StrSlice;
use unidok_repr::ast::macros::MacroArgs;

use crate::parsing_mode::ParsingMode;
use crate::utils::{ParseLineBreak, ParseLineEnd, ParseSpaces};
use crate::{Indents, Input, Parse};

pub(super) fn get_parsing_mode(
    name: &str,
    args: &Option<MacroArgs>,
    input: &Input,
) -> Option<Option<ParsingMode>> {
    Some(match name {
        "PASS" => match &args {
            None => Some(ParsingMode::new_all()),
            Some(MacroArgs::TokenTrees(tts)) => {
                let mut pm = ParsingMode::new_nothing();
                for tt in tts {
                    let word = &input[tt.as_atom()?.as_word()?];
                    match ParsingMode::parse_param(word) {
                        Some(param) => pm = pm.set(param),
                        _ => return None,
                    }
                }
                Some(pm)
            }
            _ => return None,
        },
        "NOPASS" => match &args {
            None => Some(ParsingMode::new_nothing()),
            Some(MacroArgs::TokenTrees(tts)) => {
                let mut pm = ParsingMode::new_all();
                for tt in tts {
                    let word = &input[tt.as_atom()?.as_word()?];
                    match ParsingMode::parse_param(word) {
                        Some(param) => pm = pm.unset(param),
                        _ => return None,
                    }
                }
                Some(pm)
            }
            _ => return None,
        },
        _ => None,
    })
}

pub(crate) struct ParseMacroName;

impl Parse for ParseMacroName {
    type Output = StrSlice;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        fn is_macro_char(c: char) -> bool {
            c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'
        }

        if let Some(mat) = input.rest().find(|c| !is_macro_char(c)) {
            let rest = &input.rest()[mat..];
            if rest.starts_with(char::is_alphanumeric) {
                None
            } else {
                let len = input.len() - rest.len();
                Some(input.bump(len))
            }
        } else {
            Some(input.bump(input.len()))
        }
    }
}

pub(crate) struct ParseOpeningBrace<'a>(pub(crate) Indents<'a>);

impl Parse for ParseOpeningBrace<'_> {
    type Output = ();

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('{')?;
        input.parse_i(ParseSpaces);
        input.parse(ParseLineEnd)?;
        input.try_parse(ParseLineBreak(self.0));

        input.apply();
        Some(())
    }
}

pub(crate) struct ParseClosingBrace<'a>(pub(crate) Indents<'a>);

impl Parse for ParseClosingBrace<'_> {
    type Output = ();

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse_i(ParseSpaces);
        input.parse('}')?;
        input.parse_i(ParseSpaces);
        input.parse(ParseLineEnd)?;
        input.try_parse(ParseLineBreak(self.0));

        input.apply();
        Some(())
    }
}
