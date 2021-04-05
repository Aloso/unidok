use crate::items::*;
use crate::{Input, Parse};

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Node {
    Text(Text),
    LineBreak(LineBreak),
    Escape(Escaped), // can't be followed by Text, SubstrText or Comment
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

impl Node {
    pub fn parser(parent: ParentKind, ind: Indents<'_>) -> ParseNode<'_> {
        ParseNode { parent, ind }
    }

    pub fn multi_parser(parent: ParentKind, ind: Indents<'_>) -> ParseNodes<'_> {
        ParseNodes { parent, ind }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(unused)]
pub enum ParentKind {
    Braces,
    InlineFormat { formatting: Formatting },
    Heading { level: u8 },
    List,
    Quote,
    Table,
    LinkOrImg,
    Global,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ParseNode<'a> {
    parent: ParentKind,
    ind: Indents<'a>,
}

impl Parse for ParseNode<'_> {
    type Output = Node;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let ind = self.ind;
        let parent = self.parent;

        fn parse_inline(
            ind: Indents<'_>,
            parent: ParentKind,
            input: &mut Input,
        ) -> Option<Node> {
            Some({
                if let Some(text) = input.parse(Text::parser(ind)) {
                    Node::Text(text)
                } else if let Some(esc) = input.parse(Escaped::parser()) {
                    Node::Escape(esc)
                } else if let Some(limiter) = input.parse(Limiter::parser()) {
                    Node::Limiter(limiter)
                } else if let Some(attr) = input.parse(Attribute::parser(ind)) {
                    Node::Attribute(attr)
                } else if let Some(block) = input.parse(Braces::parser(ind)) {
                    Node::Braces(block)
                } else if let Some(math) = input.parse(Math::parser(ind)) {
                    Node::Math(math)
                } else if !input.is_empty() {
                    use ParentKind as Npk;

                    match input.peek_char().unwrap() {
                        '|' if parent == Npk::Table => return None,
                        '}' if parent == Npk::Braces => return None,
                        '>' if parent == Npk::LinkOrImg => return None,
                        '\n' => return None,
                        c => Node::Text(Text(input.bump(c.len_utf8() as usize))),
                    }
                } else {
                    return None;
                }
            })
        }

        match parent {
            ParentKind::InlineFormat { formatting: Formatting::Code } => {
                if let Some(esc) = input.parse(Escaped::parser()).map(Node::Escape) {
                    Some(esc)
                } else {
                    input.parse(Text::parser(ind)).map(Node::Text)
                }
            }

            | ParentKind::Heading { .. }
            | ParentKind::InlineFormat { .. }
            | ParentKind::LinkOrImg => parse_inline(ind, parent, input),

            _ => {
                if let Some(lb) = input.parse(LineBreak::parser(ind)) {
                    Some(Node::LineBreak(lb))
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

#[derive(Debug)]
pub struct ParseNodes<'a> {
    pub parent: ParentKind,
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
