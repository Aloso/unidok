use crate::basic::Cond;
use crate::indent::Indents;
use crate::items::LineBreak;
use crate::marker::{ParseLineEnd, ParseLineStart};
use crate::str::StrSlice;
use crate::{Input, Parse, UntilChar, WhileChar};

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
    pub backticks: usize,
    pub lines: Vec<StrSlice>,
}

pub struct ParseCodeBlock<'a> {
    ind: Indents<'a>,
}

impl CodeBlock {
    pub fn parser(ind: Indents<'_>) -> ParseCodeBlock<'_> {
        ParseCodeBlock { ind }
    }
}

impl Parse for ParseCodeBlock<'_> {
    type Output = CodeBlock;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse(ParseLineStart)?;
        input.parse("```")?;
        let backticks = 3 + input.parse(WhileChar('`'))?.len();
        let meta = input.parse(UntilChar('\n'))?;

        let mut lines = Vec::new();
        loop {
            input.parse(LineBreak::parser(self.ind))?;

            if input.rest().starts_with("```") {
                let mut input2 = input.start();
                let backticks_end = input2.parse(UntilChar(|c| c != '`'))?.len();
                input2.parse(ParseLineEnd)?;
                input2.parse(Cond(|| backticks == backticks_end))?;
                input2.apply();
                break;
            }

            let line = input.parse(UntilChar('\n'))?;
            lines.push(line);
        }

        input.apply();
        Some(CodeBlock { meta, backticks, lines })
    }
}
