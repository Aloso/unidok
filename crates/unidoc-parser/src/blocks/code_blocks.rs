use std::convert::TryInto;

use crate::utils::{Indents, ParseLineBreak, ParseLineEnd, ParseSpaces, UntilChar, WhileChar};
use crate::{Input, Parse, StrSlice};

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
    pub lines: Vec<StrSlice>,
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
}

impl CodeBlock {
    pub(crate) fn parser(ind: Indents<'_>) -> ParseCodeBlock<'_> {
        ParseCodeBlock { ind }
    }
}

impl Parse for ParseCodeBlock<'_> {
    type Output = CodeBlock;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        let indent = input.parse_i(ParseSpaces);
        let ind = self.ind.push_indent(indent);

        let fence = input.parse(ParseFence)?;
        let info = input.parse(ParseInfo(fence))?;

        let mut lines = Vec::new();
        while !input.is_empty() {
            if input.parse(ParseLineBreak(ind)).is_none() {
                break;
            }

            let mut input2 = input.start();
            if let Some(closing_fence) = input2.parse(ParseFence) {
                if input2.can_parse(ParseLineEnd) && closing_fence.can_close(fence) {
                    input2.apply();
                    break;
                }
            }
            drop(input2);

            let line = input.parse_i(UntilChar(|c| matches!(c, '\n' | '\r')));
            lines.push(line);
        }

        input.apply();
        Some(CodeBlock { info, fence, lines, indent })
    }
}

struct ParseFence;

impl Parse for ParseFence {
    type Output = Fence;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        if input.can_parse("```") {
            let count = input.parse_i(WhileChar('`')).len();
            let count = count.try_into().ok()?;
            Some(Fence::Backticks(count))
        } else if input.can_parse("~~~") {
            let count = input.parse_i(WhileChar('~')).len();
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

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let s = input.parse_i(UntilChar(|c| matches!(c, '\n' | '\r')));

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
