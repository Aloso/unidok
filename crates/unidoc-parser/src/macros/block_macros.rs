use std::iter;

use crate::macros::utils::ParseMacroName;
use crate::parsing_mode::ParsingMode;
use crate::utils::{Indents, ParseLineBreak, ParseSpacesU8};
use crate::{Block, Context, Input, Parse, StrSlice};

use super::utils::{get_parsing_mode, ParseClosingBrace, ParseOpeningBrace};
use super::MacroArgs;

/// A block macro
///
/// ### Example
///
/// ````md
/// @SOME_MACRO(args)
/// The macro applies to this paragraph
/// ````
#[derive(Debug, Clone, PartialEq)]
pub struct BlockMacro {
    pub name: StrSlice,
    pub args: Option<MacroArgs>,
    pub content: BlockMacroContent,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockMacroContent {
    Prefixed(Box<Block>),
    Braces(Vec<Block>),
}

impl BlockMacro {
    pub fn parser(
        context: Context,
        ind: Indents<'_>,
        mode: Option<ParsingMode>,
        is_loose: bool,
        list_style: Option<String>,
    ) -> ParseBlockMacro<'_> {
        ParseBlockMacro { context, ind, mode, is_loose, list_style }
    }
}

pub struct ParseBlockMacro<'a> {
    context: Context,
    ind: Indents<'a>,
    mode: Option<ParsingMode>,
    is_loose: bool,
    list_style: Option<String>,
}

impl Parse for ParseBlockMacro<'_> {
    type Output = BlockMacro;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        let ind = self.ind.push_indent(input.parse(ParseSpacesU8)?);

        input.parse('@')?;
        let name = input.parse(ParseMacroName)?;
        let name_str = name.to_str(input.text()).to_string();
        let args = input.parse(MacroArgs::parser(&name_str, ind))?;

        let mode = get_parsing_mode(&name_str, &args, &input)?.or(self.mode);

        if name.is_empty() && args.is_none() {
            return None;
        }

        let mac = if input.parse(ParseLineBreak(ind)).is_some() {
            let is_loose = self.is_loose || name_str == "LOOSE";

            let list_style = self.list_style.take();
            let list_style = list_style.or_else(|| {
                if name_str == "BULLET" {
                    args.as_ref().and_then(|args| {
                        let tts = args.as_token_trees()?;
                        let mut list_style = String::new();

                        for tt in tts {
                            let atom = tt.as_atom()?;
                            if let Some(word) = atom.as_word() {
                                list_style.push_str(word.to_str(input.text()));
                                list_style.push(' ');
                            } else {
                                let word = atom.as_quoted_word()?;
                                list_style.push('"');
                                list_style.extend(word.chars().flat_map(|c| {
                                    iter::once('\\')
                                        .filter(move |_| matches!(c, '"' | '\'' | '\\'))
                                        .chain(iter::once(c))
                                }));
                                list_style.push('"');
                            }
                        }

                        Some(list_style)
                    })
                } else {
                    None
                }
            });

            let parser = Block::parser(self.context, ind, mode, is_loose, list_style);
            let block = Box::new(input.parse(parser)?);

            BlockMacro { name, args, content: BlockMacroContent::Prefixed(block) }
        } else if input.parse(ParseOpeningBrace(self.ind)).is_some() {
            let blocks = input.parse(Block::multi_parser(Context::BlockBraces, ind))?;
            input.try_parse(ParseClosingBrace(self.ind));

            BlockMacro { name, args, content: BlockMacroContent::Braces(blocks) }
        } else {
            return None;
        };

        input.apply();
        Some(mac)
    }
}
