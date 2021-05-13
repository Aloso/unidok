use std::iter;

use unidoc_repr::ast::macros::{BlockMacro, BlockMacroContent};

use crate::blocks::ParseBlock;
use crate::macros::utils::ParseMacroName;
use crate::parsing_mode::ParsingMode;
use crate::utils::{Indents, ParseLineBreak, ParseSpacesU8};
use crate::{Context, Input, Parse};

use super::args::ParseMacroArgs;
use super::utils::{get_parsing_mode, ParseClosingBrace, ParseOpeningBrace};

pub(crate) struct ParseBlockMacro<'a> {
    context: Context,
    ind: Indents<'a>,
    mode: Option<ParsingMode>,
    is_loose: bool,
    list_style: Option<String>,
    no_toc: bool,
}

impl<'a> ParseBlockMacro<'a> {
    pub fn new(
        context: Context,
        ind: Indents<'a>,
        mode: Option<ParsingMode>,
        is_loose: bool,
        list_style: Option<String>,
        no_toc: bool,
    ) -> Self {
        Self { context, ind, mode, is_loose, list_style, no_toc }
    }
}

impl Parse for ParseBlockMacro<'_> {
    type Output = BlockMacro;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        let ind = self.ind.push_indent(input.parse(ParseSpacesU8)?);

        input.parse('@')?;
        let name = input.parse(ParseMacroName)?;
        let name_str = name.to_str(input.text()).to_string();
        let args = input.parse(ParseMacroArgs { ind, name: &name_str })?;

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

            let no_toc = self.no_toc || name_str == "NO_TOC";

            let parser = ParseBlock::new(self.context, ind, mode, is_loose, list_style, no_toc);
            let block = Box::new(input.parse(parser)?);

            BlockMacro { name, args, content: BlockMacroContent::Prefixed(block) }
        } else if input.parse(ParseOpeningBrace(self.ind)).is_some() {
            let blocks = input.parse(ParseBlock::new_multi(Context::BlockBraces, ind))?;
            input.try_parse(ParseClosingBrace(self.ind));

            BlockMacro { name, args, content: BlockMacroContent::Braces(blocks) }
        } else {
            return None;
        };

        input.apply();
        Some(mac)
    }
}
