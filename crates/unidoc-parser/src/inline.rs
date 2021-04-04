use crate::{Input, Parse};

use crate::Node;

/// Inline formatting:
///
/// - `**bold**`, `__bold__`
/// - `*italic*`, `_italic_`
/// - `~~strikethrough~~`
/// - `^superscript^`
/// - `~subscript~`
/// - ```markdown `code` ```
///
/// A b *c *d* e*
#[derive(Debug, Clone)]
pub struct InlineFormat {
    pub formatting: Formatting,
    pub content: Vec<Node>,
}

#[allow(unused)]
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
