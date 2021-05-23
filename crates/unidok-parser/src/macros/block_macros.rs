use aho_corasick::AhoCorasick;
use unidok_repr::ast::macros::{BlockMacro, BlockMacroContent};

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
    no_toc: bool,
    ac: &'a AhoCorasick,
}

impl<'a> ParseBlockMacro<'a> {
    pub fn new(
        context: Context,
        ind: Indents<'a>,
        mode: Option<ParsingMode>,
        no_toc: bool,
        ac: &'a AhoCorasick,
    ) -> Self {
        Self { context, ind, mode, no_toc, ac }
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
        let args = input.parse(ParseMacroArgs { ind, name: &name_str, ac: self.ac })?;

        let mode = get_parsing_mode(&name_str, &args, &input)?.or(self.mode);

        if name.is_empty() && args.is_none() {
            return None;
        }

        let mac = if input.parse(ParseLineBreak(ind)).is_some() {
            let no_toc = self.no_toc || name_str == "NOTOC";

            let parser = ParseBlock::new(self.context, ind, mode, no_toc, self.ac);
            let block = Box::new(input.parse(parser)?);

            BlockMacro { name, args, content: BlockMacroContent::Prefixed(block) }
        } else if input.parse(ParseOpeningBrace(self.ind)).is_some() {
            let blocks = input.parse(ParseBlock::new_multi(
                Context::BlockBraces,
                ind,
                self.mode,
                self.ac,
            ))?;
            input.try_parse(ParseClosingBrace(self.ind));

            BlockMacro { name, args, content: BlockMacroContent::Braces(blocks) }
        } else {
            return None;
        };

        input.apply();
        Some(mac)
    }
}
