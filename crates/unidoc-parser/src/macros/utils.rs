use crate::parsing_mode::ParsingMode;
use crate::utils::{ParseLineBreak, ParseLineEnd, ParseSpaces};
use crate::{Indents, Input, Parse, StrSlice};

use super::MacroArgs;

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
                    match &input[tt.as_atom()?.as_word()?] {
                        "inline" | "i" => pm = pm.set(ParsingMode::INLINE),
                        "codeblock" | "c" => pm = pm.set(ParsingMode::CODE_BLOCKS),
                        "heading" | "h" => pm = pm.set(ParsingMode::HEADINGS),
                        "tbreak" | "b" => pm = pm.set(ParsingMode::THEMATIC_BREAKS),
                        "subst" | "s" => pm = pm.set(ParsingMode::SUBSTITUTIONS),
                        "list" | "l" => pm = pm.set(ParsingMode::LISTS),
                        "limiter" | "$" => pm = pm.set(ParsingMode::LIMITER),
                        "macro" | "@" => pm = pm.set(ParsingMode::MACROS),
                        "math" | "%" => pm = pm.set(ParsingMode::MATH),
                        "table" | "|" => pm = pm.set(ParsingMode::TABLES),
                        "quote" | ">" => pm = pm.set(ParsingMode::QUOTES),
                        "html" | "<" => pm = pm.set(ParsingMode::HTML),
                        "link_img" | "li" => pm = pm.set(ParsingMode::LINKS_IMAGES),
                        _ => return None,
                    }
                }
                Some(pm)
            }
            _ => return None,
        },
        "NOPASS" => Some(ParsingMode::new_nothing()),
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
