use crate::indent::Indents;
use crate::items::{Node, ParentKind};
use crate::str::StrSlice;
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
/// a separate line above the element. It must be directly after the line break
/// and must be the only content in the line.
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
    pub is_line_start: bool,
    pub content: StrSlice,
}

impl Attribute {
    pub fn parser(ind: Indents<'_>) -> ParseAttribute<'_> {
        ParseAttribute { ind }
    }
}

pub struct ParseAttribute<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseAttribute<'_> {
    type Output = Attribute;

    fn parse(&self, input: &mut crate::Input) -> Option<Self::Output> {
        let is_line_start = input.is_line_start();
        let mut input = input.start();

        input.parse('[')?;
        let content = {
            let mut input2 = input.start();
            input2.parse(Node::multi_parser(ParentKind::Attribute, self.ind, false))?;
            input2.apply()
        };
        input.parse(']')?;

        input.apply();
        Some(Attribute { is_line_start, content })
    }
}
