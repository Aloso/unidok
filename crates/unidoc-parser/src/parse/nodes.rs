use crate::{Input, Parse, StrSlice};

use super::attributes::{Attribute, ParseAttribute};
use super::braces::{Braces, ParseBraces};
use super::code_blocks::{CodeBlock, ParseCodeBlock};
use super::comments::{Comment, ParseComment};
use super::escapes_limiters::{Escape, Limiter, ParseEscape, ParseLimiter};
use super::headings::{Heading, ParseHeading};
use super::hr::{HorizontalLine, ParseHorizontalLine};
use super::images::Image;
use super::indent::Indents;
use super::inline::{Formatting, InlineFormat};
use super::links::Link;
use super::lists::{List, ParseList};
use super::macros::Macro;
use super::math::{Math, ParseMath};
use super::quotes::{ParseQuote, Quote};
use super::subst_text::{ParseSubstText, SubstText};
use super::tables::{ParseTable, Table};
use super::text::ParseText;

#[allow(unused)]
pub enum Node {
    Text(StrSlice),
    SubstText(SubstText),
    Escape(Escape), // can't be followed by Text, SubstrText or Comment
    Limiter(Limiter),
    Braces(Braces),
    Math(Math),
    InlineFormat(InlineFormat),
    Attribute(Attribute), // can't be before Text, SubstrText, Limiter, Escape, Comment
    Link(Link),
    Image(Image),
    Macro(Macro),

    Comment(Comment),
    HorizontalLine(HorizontalLine),
    CodeBlock(CodeBlock),
    Heading(Heading),
    List(List),
    Quote(Quote),
    Table(Table),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(unused)]
pub enum NodeParentKind {
    Braces,
    Math,
    InlineFormat { formatting: Formatting },
    Heading { level: u8 },
    List,
    Quote,
    Table,
    Link,
    Image,
    Global,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ParseNode<'a> {
    parent: NodeParentKind,
    ind: Indents<'a>,
}

impl Parse for ParseNode<'_> {
    type Output = Node;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let ind = self.ind;
        let parent = self.parent;

        fn parse_inline(
            ind: Indents<'_>,
            parent: NodeParentKind,
            input: &mut Input,
        ) -> Option<Node> {
            Some({
                if let Some(text) = input.parse(ParseText { ind, parent }) {
                    Node::Text(text)
                } else if let Some(text) = input.parse(ParseSubstText) {
                    Node::SubstText(text)
                } else if let Some(esc) = input.parse(ParseEscape) {
                    Node::Escape(esc)
                } else if let Some(limiter) = input.parse(ParseLimiter) {
                    Node::Limiter(limiter)
                } else if let Some(attr) = input.parse(ParseAttribute { ind }) {
                    Node::Attribute(attr)
                } else if let Some(block) = input.parse(ParseBraces { ind }) {
                    Node::Braces(block)
                } else if let Some(math) = input.parse(ParseMath) {
                    Node::Math(math)
                } else {
                    return None;
                }
            })
        }

        match parent {
            | NodeParentKind::Math
            | NodeParentKind::InlineFormat { formatting: Formatting::Code } => {
                if let Some(esc) = input.parse(ParseEscape) {
                    Some(Node::Escape(esc))
                } else if let Some(text) = input.parse(ParseText { ind, parent }) {
                    Some(Node::Text(text))
                } else {
                    None
                }
            }

            | NodeParentKind::Heading { .. }
            | NodeParentKind::InlineFormat { .. }
            | NodeParentKind::Link
            | NodeParentKind::Image => parse_inline(ind, parent, input),

            _ => {
                if let Some(comment) = input.parse(ParseComment { ind }) {
                    Some(Node::Comment(comment))
                } else if let Some(hr) = input.parse(ParseHorizontalLine { ind }) {
                    Some(Node::HorizontalLine(hr))
                } else if let Some(block) = input.parse(ParseCodeBlock { ind }) {
                    Some(Node::CodeBlock(block))
                } else if let Some(heading) = input.parse(ParseHeading { ind }) {
                    Some(Node::Heading(heading))
                } else if let Some(list) = input.parse(ParseList { ind }) {
                    Some(Node::List(list))
                } else if let Some(quote) = input.parse(ParseQuote { ind }) {
                    Some(Node::Quote(quote))
                } else if let Some(table) = input.parse(ParseTable { ind }) {
                    Some(Node::Table(table))
                } else {
                    parse_inline(ind, parent, input)
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct ParseNodes<'a> {
    pub parent: NodeParentKind,
    pub ind: Indents<'a>,
}

impl Parse for ParseNodes<'_> {
    type Output = Vec<Node>;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let parent = self.parent;
        let ind = self.ind;
        let parser = ParseNode { parent, ind };

        let mut v = Vec::new();
        while let Some(node) = input.parse(parser) {
            v.push(node);
        }
        Some(v)
    }
}
