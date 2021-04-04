use crate::{Input, Parse, StrSlice};

use super::indent::Indents;
use super::marker::{ParseLineEnd, ParseLineStart};
use super::{UntilChar, UntilStr};

/// A code block, e.g.
///
/// ````md
/// ```rust
/// pub struct Foo;
/// ```
/// ````
#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub meta: StrSlice,
    pub backticks: u8,
    pub content: StrSlice,
}

pub struct ParseCodeBlock<'a> {
    pub ind: Indents<'a>,
}

impl Parse for ParseCodeBlock<'_> {
    type Output = CodeBlock;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse(ParseLineStart)?;
        input.parse("```")?;
        let mut backticks = 3;
        while input.parse('`').is_some() {
            if backticks == u8::MAX {
                return None;
            }
            backticks += 1;
        }
        let meta = input.parse(UntilChar(|c| c == '\n'))?;
        input.parse('\n')?;

        const LIMIT: &str = "\n\
            ````````````````````````````````````````````````````````````````\
            ````````````````````````````````````````````````````````````````\
            ````````````````````````````````````````````````````````````````\
            ````````````````````````````````````````````````````````````````";
        let limit = &LIMIT[..backticks as usize + 1];
        let content = input.parse(UntilStr(limit))?;
        input.parse(limit)?;
        input.parse(ParseLineEnd)?;

        input.apply();
        Some(CodeBlock { meta, backticks, content })
    }
}
