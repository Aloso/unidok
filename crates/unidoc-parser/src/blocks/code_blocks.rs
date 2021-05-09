use std::convert::TryInto;

use crate::parsing_mode::ParsingMode;
use crate::utils::{ParseLineBreak, ParseSpacesU8, ParseWsAndLineEnd, Until, While};
use crate::{Indents, Input, Parse, StrSlice};

use super::{Block, Context};

#[rustfmt::skip]
/// A code block
///
/// ### Example
///
/// ````md
/// ```rust
/// pub struct Foo;
/// ```
/// ````
#[derive(Debug, Clone, PartialEq)]
pub struct CodeBlock {
    pub info: StrSlice,
    pub fence: Fence,
    pub lines: Vec<Block>,
    pub indent: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fence {
    Backticks(u32),
    Tildes(u32),
}

impl Fence {
    fn can_close(self, opening_fence: Fence) -> bool {
        match (opening_fence, self) {
            (Fence::Backticks(a), Fence::Backticks(b)) => a <= b,
            (Fence::Tildes(a), Fence::Tildes(b)) => a <= b,
            _ => false,
        }
    }
}

pub(crate) struct ParseCodeBlock<'a> {
    ind: Indents<'a>,
    mode: Option<ParsingMode>,
}

impl CodeBlock {
    pub(crate) fn parser(ind: Indents<'_>, mode: Option<ParsingMode>) -> ParseCodeBlock<'_> {
        ParseCodeBlock { ind, mode }
    }
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

            let line = input.parse(Block::parser(context, ind, Some(mode), false, None))?;
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
