use crate::inlines::Segment;

/// A paragraph
///
/// Paragraphs can be interrupted by ATX-style headings, lists, quotes, tables,
/// code blocks, thematic breaks that don't consist of dashes and line comments.
#[derive(Debug, Clone, PartialEq)]
pub struct Paragraph {
    pub segments: Vec<Segment>,
}
