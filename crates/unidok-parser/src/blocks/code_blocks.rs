use std::convert::TryInto;

use unidok_repr::ast::blocks::{CodeBlock, Fence};

use crate::parsing_mode::ParsingMode;
use crate::utils::{ParseLineBreak, ParseSpacesU8, ParseWsAndLineEnd, Until, While};
use crate::{Indents, Input, Parse, StrSlice};

use super::{Context, ParseBlock};

pub(crate) struct ParseCodeBlock<'a> {
    pub ind: Indents<'a>,
    pub mode: Option<ParsingMode>,
}

impl Parse for ParseCodeBlock<'_> {
    type Output = CodeBlock;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        let indent = input.parse(ParseSpacesU8)?;
        let ind = self.ind.push_indent(indent);

        let fence = input.parse(ParseFence)?;
        let info = input.parse(ParseInfo(fence))?;

        let mode = self.mode.unwrap_or_else(ParsingMode::new_nothing);
        let context = Context::CodeBlock;

        let mut lines = Vec::new();
        while !input.is_empty() {
            if input.parse(ParseLineBreak(ind)).is_none() {
                break;
            }

            let mut input2 = input.start();
            if let Some(closing_fence) = input2.parse(ParseFence) {
                if closing_fence.can_close(fence) && input2.parse(ParseWsAndLineEnd).is_some() {
                    input2.apply();
                    break;
                }
            }
            drop(input2);

            let line = input.parse(ParseBlock::new(context, ind, Some(mode), true))?;
            lines.push(line);
        }

        input.apply();
        Some(CodeBlock { info, fence, lines, indent })
    }
}

struct ParseFence;

impl Parse for ParseFence {
    type Output = Fence;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        if input.can_parse("```") {
            let count = input.parse_i(While('`')).len();
            let count = count.try_into().ok()?;
            Some(Fence::Backticks(count))
        } else if input.can_parse("~~~") {
            let count = input.parse_i(While('~')).len();
            let count = count.try_into().ok()?;
            Some(Fence::Tildes(count))
        } else {
            None
        }
    }
}

struct ParseInfo(Fence);

impl Parse for ParseInfo {
    type Output = StrSlice;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let s = input.parse_i(Until(|c| matches!(c, '\n' | '\r')));

        let c = match self.0 {
            Fence::Backticks(_) => '`',
            Fence::Tildes(_) => '~',
        };
        if s.to_str(input.text()).contains(c) {
            return None;
        }

        Some(s)
    }
}
