use unidok_repr::ast::macros::{BlockMacro, BlockMacroContent};

use crate::blocks::ParseBlock;
use crate::macros::utils::ParseMacroName;
use crate::parsing_mode::ParsingMode;
use crate::state::ParsingState;
use crate::utils::{ParseLineBreak, ParseLineEnd, ParseSpaces, ParseSpacesU8};
use crate::{Context, Input, Parse};

use super::args::ParseMacroArgs;
use super::utils::{get_parsing_mode, ParseClosingBrace, ParseOpeningBrace};

pub(crate) struct ParseBlockMacro<'a> {
    mode: Option<ParsingMode>,
    state: ParsingState<'a>,
}

impl<'a> ParseBlockMacro<'a> {
    pub fn new(mode: Option<ParsingMode>, state: ParsingState<'a>) -> Self {
        Self { mode, state }
    }
}

impl Parse for ParseBlockMacro<'_> {
    type Output = BlockMacro;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        let ind = self.state.ind();
        let ind = ind.push_indent(input.parse(ParseSpacesU8)?);
        let ac = self.state.special_chars();

        input.parse('@')?;
        let name = input.parse(ParseMacroName)?;
        let name_str = name.to_str(&input.text).to_string();
        let args = input.parse(ParseMacroArgs { ind, name: &name_str, ac })?;

        if name.is_empty() && args.is_none() {
            return None;
        }

        input.parse_i(ParseSpaces);

        let mode = get_parsing_mode(&name_str, &args, &input)?.or(self.mode);

        let content = if input.parse(ParseLineEnd).is_some() {
            let mut line_breaks = 0u8;
            if input.parse(ParseLineBreak(ind)).is_some() {
                line_breaks += 1;
            }
            if input.parse(ParseLineBreak(ind)).is_some() {
                line_breaks += 1;
            }

            if line_breaks == 1 {
                let parser =
                    ParseBlock::new(mode, ParsingState::new(ind, self.state.context(), ac));
                let block = Box::new(input.parse(parser)?);

                BlockMacroContent::Prefixed(block)
            } else {
                BlockMacroContent::None
            }
        } else if input.parse(ParseOpeningBrace(self.state.ind())).is_some() {
            let blocks = input.parse(ParseBlock::new_multi(
                self.mode,
                ParsingState::new(ind, Context::BlockBraces, ac),
            ))?;
            input.try_parse(ParseClosingBrace(ind));

            BlockMacroContent::Braces(blocks)
        } else {
            return None;
        };

        input.apply();
        Some(BlockMacro { name, args, content })
    }
}
