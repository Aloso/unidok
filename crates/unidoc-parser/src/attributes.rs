use crate::indent::Indents;
use crate::inlines::Segment;
use crate::items::LineBreak;
use crate::str::StrSlice;
use crate::utils::cond::If;
use crate::utils::ParseSpaces;
use crate::Parse;

/// An attribute.
///
/// ### Syntax
///
/// ````markdown
/// [attr]{content} and [attr]**fat content**
///
/// [attr]
/// # Heading
///
/// [attr1]
/// [attr2]
/// . This is
/// . a list
/// ````
///
/// When it applies to a block element (list, table, quote, etc.), it must be in
/// a separate line above the element. It must be after a line break, and in the
/// same can be nothing except whitespace.
///
/// Inline attributes can apply to formatting (`[attr]**bold**`,
/// `[attr]_italic_`, etc.), to braces (`[attr]{content}`), to math, links,
/// images and macros.
///
/// There can be arbitrarily many attributes for an item. Attributes can't
/// contain line breaks directly, but if an attribute contains braces, these
/// braces can have line breaks.
///
/// ### Attribute content
///
/// Attributes contain HTML attributes and other information about the element.
/// There are some built-in attributes for controlling parsing or document
/// generation.
///
/// Attributes may be comma-separated words, or comma-separated key-value pairs,
/// e.g.
///
/// ````markdown
/// [.foo, #bar]{}
/// [title={Hello world}, role=button]</link click here!>
/// ````
#[derive(Debug, Clone)]
pub struct Attribute {
    pub is_separate_line: bool,
    pub content: StrSlice,
}

impl Attribute {
    pub(crate) fn parser(ind: Indents<'_>) -> ParseAttribute<'_> {
        ParseAttribute { ind }
    }
}

pub(crate) struct ParseAttribute<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseAttribute<'_> {
    type Output = Attribute;

    fn parse(&self, input: &mut crate::Input) -> Option<Self::Output> {
        let is_line_start = input.is_line_start();
        let mut input = input.start();

        let indent = if is_line_start { input.parse(ParseSpaces)? } else { 0 };

        input.parse('[')?;
        let content = {
            let mut input2 = input.start();
            input2.parse(Segment::multi_parser(
                crate::inlines::SegmentCtx::Attribute,
                self.ind,
            ))?;
            input2.apply()
        };
        input.parse(']')?;

        let outdent = if is_line_start { input.parse(ParseSpaces)? } else { 0 };

        let is_separate_line = if input.parse(LineBreak::parser(self.ind)).is_some() {
            true
        } else {
            input.parse(If(indent == 0 && outdent == 0))?;
            false
        };

        input.apply();
        Some(Attribute { is_separate_line, content })
    }
}
