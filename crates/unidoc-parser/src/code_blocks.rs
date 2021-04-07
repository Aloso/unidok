use crate::cond::If;
use crate::indent::Indents;
use crate::items::LineBreak;
use crate::str::StrSlice;
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
#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub info: StrSlice,
    pub backticks: usize,
    pub lines: Vec<StrSlice>,
    pub indent: u8,
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

        input.parse(Self::LINE_START)?;
        let indent = input.parse(Self::WS)?;

        input.parse("```")?;
        let backticks = 3 + input.parse(WhileChar('`'))?.len();
        let info = input.parse(UntilChar('\n'))?;

        let mut lines = Vec::new();
        loop {
            input.parse(LineBreak::parser(self.ind))?;
            input.parse(Self::spaces(indent))?;

            if input.rest().starts_with("```") {
                let mut input2 = input.start();
                let backticks_end = input2.parse(WhileChar('`'))?.len();
                input2.parse(Self::LINE_END)?;
                input2.parse(If(backticks == backticks_end))?;
                input2.apply();
                break;
            }

            let line = input.parse(UntilChar('\n'))?;
            lines.push(line);
        }

        input.apply();
        Some(CodeBlock { info, backticks, lines, indent })
    }
}
