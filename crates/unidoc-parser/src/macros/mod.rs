use crate::blocks::CellMeta;
use crate::input::Input;
use crate::parsing_mode::ParsingMode;
use crate::utils::{Indents, Until};
use crate::{Parse, ParseInfallible, StrSlice};

mod token_trees;

pub use token_trees::{TokenTree, TokenTreeAtom};

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
}

pub struct ParseMacroArgs<'a> {
    name: &'a str,
    ind: Indents<'a>,
}

impl Parse for ParseMacroArgs<'_> {
    type Output = Option<MacroArgs>;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();
        if input.parse('(').is_none() {
            return Some(match self.name {
                "PASS" => Some(MacroArgs::ParsingMode(ParsingMode::new_all())),
                "NOPASS" => Some(MacroArgs::ParsingMode(ParsingMode::new_nothing())),
                _ => None,
            });
        }
        let content = match self.name {
            "LOAD" => MacroArgs::Raw(input.parse_i(ParseRaw)),
            _ => MacroArgs::TokenTrees(input.parse_i(TokenTree::multi_parser(self.ind))),
        };
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
