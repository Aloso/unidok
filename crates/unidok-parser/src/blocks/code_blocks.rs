use std::convert::TryInto;

use aho_corasick::AhoCorasick;
use detached_str::StrSlice;
use unidok_repr::ast::blocks::{CodeBlockAst, FenceType};
use unidok_repr::Span;

use crate::parsing_mode::ParsingMode;
use crate::state::ParsingState;
use crate::utils::{ParseLineBreak, ParseSpacesU8, ParseWsAndLineEnd, Until, While};
use crate::{Context, Indents, Input, Parse};

use super::ParseBlock;

pub(crate) struct ParseCodeBlock<'a> {
    pub mode: Option<ParsingMode>,
    pub ind: Indents<'a>,
    pub ac: &'a AhoCorasick,
}

impl Parse for ParseCodeBlock<'_> {
    type Output = CodeBlockAst;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        let indent = input.parse(ParseSpacesU8)?;
        let ind = self.ind.push_indent(indent);

        let mut closing_fence = None;
        let (fence_type, opening_fence) = input.parse(ParseFence)?;
        let info = input.parse(ParseInfo(fence_type))?;

        let mode = self.mode.unwrap_or_else(ParsingMode::new_nothing);
        let context = Context::CodeBlock;

        let mut lines = Vec::new();
        while !input.is_empty() {
            if input.parse(ParseLineBreak(ind)).is_none() {
                break;
            }

            let mut input2 = input.start();
            if let Some((cf_type, span)) = input2.parse(ParseFence) {
                if cf_type.can_close(fence_type) && input2.parse(ParseWsAndLineEnd).is_some() {
                    closing_fence = Some(span);
                    input2.apply();
                    break;
                }
            }
            drop(input2);

            let line = input
                .parse(ParseBlock::new(Some(mode), ParsingState::new(ind, context, self.ac)))?;
            lines.push(line);
        }

        let closing_fence = closing_fence.unwrap_or_else(|| input.prev_slice_bytes(0).into());

        input.apply();
        Some(CodeBlockAst { info, fence_type, lines, indent, opening_fence, closing_fence })
    }
}

struct ParseFence;

impl Parse for ParseFence {
    type Output = (FenceType, Span);

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        if input.can_parse("```") {
            let slice = input.parse_i(While('`'));
            let count = slice.len();
            let count = count.try_into().ok()?;

            input.apply();
            Some((FenceType::Backticks(count), slice.into()))
        } else if input.can_parse("~~~") {
            let slice = input.parse_i(While('~'));
            let count = slice.len();
            let count = count.try_into().ok()?;

            input.apply();
            Some((FenceType::Tildes(count), slice.into()))
        } else {
            None
        }
    }
}

struct ParseInfo(FenceType);

impl Parse for ParseInfo {
    type Output = StrSlice;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let s = input.parse_i(Until(|c| matches!(c, '\n' | '\r')));

        let c = match self.0 {
            FenceType::Backticks(_) => '`',
            FenceType::Tildes(_) => '~',
        };
        if s.to_str(&input.text).contains(c) {
            return None;
        }

        Some(s)
    }
}
