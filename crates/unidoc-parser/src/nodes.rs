use crate::items::*;
use crate::{Input, Parse};

#[derive(Debug, Clone)]
pub enum Node {
    Text(Text),
    LineBreak(LineBreak),
    Escape(Escaped), // can't be followed by Text, SubstrText or Comment
    Limiter(Limiter),
    Braces(Braces),
    Math(Math),
    InlineFormat(InlineFormat),
    Attribute(Attribute), // can't be before Text, Limiter, Escape, Comment
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

impl Node {
    pub fn parser(
        parent: ParentKind,
        ind: Indents<'_>,
        multiline: bool,
    ) -> ParseNode<'_> {
        ParseNode { parent, ind, multiline }
    }

    pub fn multi_parser(
        parent: ParentKind,
        ind: Indents<'_>,
        multiline: bool,
    ) -> ParseNodes<'_> {
        ParseNodes { parent, ind, multiline }
    }

    pub fn global_parser<'a>() -> ParseNodes<'a> {
        ParseNodes {
            parent: ParentKind::Global,
            ind: Default::default(),
            multiline: true,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ParentKind {
    Braces,
    InlineFormat { formatting: Formatting },
    Heading { level: u8 },
    List,
    Quote,
    Table,
    LinkOrImg,
    Attribute,
    Global,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ParseNode<'a> {
    parent: ParentKind,
    ind: Indents<'a>,
    multiline: bool,
}

impl Parse for ParseNode<'_> {
    type Output = Node;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let ind = self.ind;
        let parent = self.parent;

        match parent {
            ParentKind::InlineFormat { formatting: Formatting::Code } => {
                if let Some(esc) = input.parse(Escaped::parser()).map(Node::Escape) {
                    Some(esc)
                } else {
                    input.parse(Text::parser()).map(Node::Text)
                }
            }

            | ParentKind::Heading { .. }
            | ParentKind::InlineFormat { .. }
            | ParentKind::LinkOrImg => parse_inline(ind, parent, input),

            _ => {
                if self.multiline && input.parse(LineBreak::parser(ind)).is_some() {
                    Some(Node::LineBreak(LineBreak))
                } else if let Some(comment) = input.parse(Comment::parser()) {
                    Some(Node::Comment(comment))
                } else if let Some(hr) = input.parse(HorizontalLine::parser(ind)) {
                    Some(Node::HorizontalLine(hr))
                } else if let Some(block) = input.parse(CodeBlock::parser(ind)) {
                    Some(Node::CodeBlock(block))
                } else if let Some(heading) = input.parse(Heading::parser(ind)) {
                    Some(Node::Heading(heading))
                } else if let Some(list) = input.parse(List::parser(ind)) {
                    Some(Node::List(list))
                } else if let Some(quote) = input.parse(Quote::parser(ind)) {
                    Some(Node::Quote(quote))
                } else if let Some(table) = input.parse(Table::parser(ind)) {
                    Some(Node::Table(table))
                } else {
                    parse_inline(ind, parent, input)
                }
            }
        }
    }
}

fn parse_inline(ind: Indents<'_>, parent: ParentKind, input: &mut Input) -> Option<Node> {
    if let Some(text) = input.parse(Text::parser()) {
        Some(Node::Text(text))
    } else if let Some(esc) = input.parse(Escaped::parser()) {
        Some(Node::Escape(esc))
    } else if let Some(limiter) = input.parse(Limiter::parser()) {
        Some(Node::Limiter(limiter))
    } else if let Some(attr) = input.parse(Attribute::parser(ind)) {
        Some(Node::Attribute(attr))
    } else if let Some(block) = input.parse(Braces::parser(ind)) {
        Some(Node::Braces(block))
    } else if let Some(math) = input.parse(Math::parser(ind)) {
        Some(Node::Math(math))
    } else if !input.is_empty() {
        match input.peek_char().unwrap() {
            ']' if parent == ParentKind::Attribute => None,
            '|' if parent == ParentKind::Table => None,
            '}' if parent == ParentKind::Braces => None,
            '>' if parent == ParentKind::LinkOrImg => None,
            '\n' => None,
            c => Some(Node::Text(Text(input.bump(c.len_utf8() as usize)))),
        }
    } else {
        None
    }
}

#[derive(Debug)]
pub struct ParseNodes<'a> {
    parent: ParentKind,
    ind: Indents<'a>,
    multiline: bool,
}

impl Parse for ParseNodes<'_> {
    type Output = Vec<Node>;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let parser = Node::parser(self.parent, self.ind, self.multiline);

        let mut v = Vec::new();
        while let Some(node) = input.parse(parser) {
            v.push(node);
        }
        Some(v)
    }
}
