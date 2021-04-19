use std::convert::TryInto;

use crate::str::StrSlice;
use crate::utils::{If, Indents, ParseLineBreak, ParseLineEnd, ParseNSpaces, ParseSpaces};
use crate::{Input, Parse, UntilChar, WhileChar};

#[rustfmt::skip]
/// A code block.
///
/// ### Syntax
///
/// ````md
/// ```rust
/// pub struct Foo;
/// ```
/// ````
///
/// The code block is enclosed with lines that contain at least 3 backticks. The
/// number of backticks must be equal at the start and at the end.
///
/// At the end of the first line, it's possible to add metadata, which is a
/// comma-separated list of words. These words can be
///
/// - The programming language of the code, which should be syntax highlighted
/// - The `diff` keyword, which highlights added and removed lines (indicated by
///   a leading `+` or `-`); this can be combined with a programming language
/// - Parsing directives, such as `+macros`, which parses and expands macros
///   within the code block.
///
/// ### Syntax highlighting
///
/// Syntax highlighting happens _after_ expansion and formatting of the content.
/// It works like this:
///
/// - The _text content_, i.e. the visible text in the document is obtained by
///   removing all HTML elements.
/// - The text content is passed to a syntax highlighter. This returns a HTML
///   tree where parts that should be colored have a CSS class.
/// - The previously removed HTML elements are inserted into the HTML tree
///   again. If necessary, elements are split if they overlap with the previous
///   elements.
///
/// Example:
///
/// 1.  ```html
///     fn hello<i class="underscore">_</i>world();
///     ```
///
/// 2.  ```html
///     fn hello_world();
///     ```
///
/// 3.  ```html
///     <span class="kw">fn</span> <span class="ident">hello_world</span><span class="punct">();</span>
///     ```
///
/// 4.  ```html
///     <span class="kw">fn</span> <span class="ident">hello<i class="underscore">_</i>world</span><span class="punct">();</span>
///     ```
///
/// You can configure unidoc to use `<i>` instead of `<span>` elements for syntax highlighting.
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

        let indent = input.parse(ParseSpaces)?;

        let fence = input.parse(ParseFence)?;
        let info = input.parse(ParseInfo(fence))?;

        let mut lines = Vec::new();
        loop {
            input.parse(ParseLineBreak(self.ind))?;
            input.parse(ParseNSpaces(indent))?;

            let mut input2 = input.start();
            if let Some(closing_fence) = input2.parse(ParseFence) {
                if input2.can_parse(ParseLineEnd) && closing_fence.can_close(fence) {
                    input2.apply();
                    break;
                }
            }
            drop(input2);

            let line = input.parse(UntilChar('\n'))?;
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
            let count = input.parse(WhileChar('`'))?.len();
            let count = count.try_into().ok()?;
            Some(Fence::Backticks(count))
        } else if input.can_parse("~~~") {
            let count = input.parse(WhileChar('~'))?.len();
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
        let s = input.parse(UntilChar(|c| matches!(c, '\n' | '\r'))).unwrap();
        if let Fence::Backticks(_) = self.0 {
            input.parse(If(!s.to_str(input.text()).contains('`')))?;
        }
        Some(s)
    }
}
