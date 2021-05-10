use crate::blocks::CellMeta;
use crate::input::Input;
use crate::parsing_mode::ParsingMode;
use crate::utils::{Indents, ParseLineBreak, ParseOneWS, Until};
use crate::{Parse, ParseInfallible, StrSlice};

use super::{TokenTree, TokenTreeAtom};

#[derive(Debug, Clone, PartialEq)]
pub enum MacroArgs {
    Raw(StrSlice),
    TokenTrees(Vec<TokenTree>),
    CellMeta(Vec<CellMeta>),
    ParsingMode(ParsingMode),
}

impl MacroArgs {
    pub(crate) fn parser<'a>(name: &'a str, ind: Indents<'a>) -> ParseMacroArgs<'a> {
        ParseMacroArgs { name, ind }
    }

    pub fn get_one_string(&self, input: &Input) -> Option<String> {
        match self {
            MacroArgs::Raw(s) => Some(input[*s].to_string()),
            MacroArgs::TokenTrees(t) if t.len() == 1 => match &t[0] {
                TokenTree::Atom(t) => match t {
                    TokenTreeAtom::Word(w) => Some(input[*w].to_string()),
                    TokenTreeAtom::QuotedWord(w) => Some(w.clone()),
                    _ => None,
                },
                TokenTree::KV(_, _) => None,
            },
            _ => None,
        }
    }

    pub fn as_token_trees(&self) -> Option<&[TokenTree]> {
        if let MacroArgs::TokenTrees(t) = self {
            Some(t)
        } else {
            None
        }
    }
}

pub struct ParseMacroArgs<'a> {
    name: &'a str,
    ind: Indents<'a>,
}

impl Parse for ParseMacroArgs<'_> {
    type Output = Option<MacroArgs>;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        if input.parse('(').is_none() {
            return Some(None);
        }
        let content = match self.name {
            "LOAD" => MacroArgs::Raw(input.parse_i(ParseRaw)),
            _ => MacroArgs::TokenTrees(
                input.parse(TokenTree::multi_parser(self.ind.push_indent(2)))?,
            ),
        };

        input.try_parse(ParseLineBreak(self.ind));
        input.try_parse(ParseOneWS);
        input.parse(')')?;

        input.apply();
        Some(Some(content))
    }
}

struct ParseRaw;

impl ParseInfallible for ParseRaw {
    type Output = StrSlice;

    fn parse_infallible(&self, input: &mut Input) -> Self::Output {
        input.parse_i(Until(|c| matches!(c, ')' | '\n' | '\r')))
    }
}
