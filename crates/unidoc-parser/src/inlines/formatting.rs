use crate::{Input, Parse};

use super::Segment;

/// Inline formatting (bold, italic, etc.)
///
/// - `**bold**`, `__bold__`
/// - `*italic*`, `_italic_`
/// - `~~strikethrough~~`
/// - `^superscript^`
/// - `~subscript~`
/// - `` `code` ``
///
/// Inline formatting generally can't span multiple lines. To achieve this, you
/// need to add braces within the formatting, e.g.
///
/// ```markdown
/// **{this is
///
/// bold}**.
/// ```
///
/// which generates code like this:
///
/// ```html
/// <p><b>this is</b></p>
/// <p><b>bold</b>.</p>
/// ```
///
/// #### TODO:
/// Inline formatting should be able to span multiple lines, but not two
/// consecutive line breaks (which introduce a new paragraph). Consider how this
/// can be implemented and how it affects other parsers, e.g. headings or table
/// cells which contain inline formatting.
#[derive(Debug, Clone)]
pub struct InlineFormat {
    pub formatting: Formatting,
    pub content: Vec<Segment>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Formatting {
    Bold,
    Italic,
    StrikeThrough,
    Code,
    Superscript,
    Subscript,
}

pub struct ParseInlineFormat;

impl Parse for ParseInlineFormat {
    type Output = InlineFormat;

    fn parse(&self, _input: &mut Input) -> Option<Self::Output> {
        todo!()
    }
}
