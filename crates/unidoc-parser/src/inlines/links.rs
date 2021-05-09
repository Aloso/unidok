use std::mem::replace;

use super::segments::{Segment, Segments};
use crate::parsing_mode::ParsingMode;
use crate::utils::Until;
use crate::{Context, Indents, Input, Parse, StrSlice};

/// A hyperlink.
///
/// ### Syntax
///
/// ```markdown
/// [Link text](https://www.example.com "a title")
/// ```
///
/// The title is optional. The link text can contain formatting.
///
/// The URL can be absolute or relative. It can contain whitespace, and even
/// parentheses, if they're balanced. Unbalanced parentheses must be escaped
/// with a backslash. Double quotes must be escaped if they're preceded by
/// whitespace.
///
/// The title must be wrapped in double quotes. It can't contain formatting.
/// Double quotes in the title can be escaped with a backslash.
#[derive(Debug, Clone, PartialEq)]
pub struct Link {
    pub text: Option<Vec<Segment>>,
    pub target: LinkTarget,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LinkTarget {
    Url { href: String, title: Option<String> },
    Reference(StrSlice),
}

impl Link {
    pub(crate) fn parser(ind: Indents<'_>) -> ParseLink<'_> {
        ParseLink { ind }
    }
}

pub(crate) struct ParseLink<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseLink<'_> {
    type Output = Link;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        if let Some(link) = input.parse(ParseFullLink { ind: self.ind }) {
            Some(link)
        } else {
            input.parse(ParseLinkTargetReference).map(|target| Link { text: None, target })
        }
    }
}

pub(super) struct ParseFullLink<'a> {
    pub(super) ind: Indents<'a>,
}

impl Parse for ParseFullLink<'_> {
    type Output = Link;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('[')?;
        let text = input
            .parse(Segments::parser(self.ind, Context::LinkOrImg, ParsingMode::new_all()))?
            .into_segments_no_underline_zero()?;
        input.parse(']')?;

        let target =
            input.parse(ParseLinkTargetUrl).or_else(|| input.parse(ParseLinkTargetReference))?;

        input.apply();
        Some(Link { text: Some(text), target })
    }
}

pub(super) struct ParseLinkTargetUrl;

impl Parse for ParseLinkTargetUrl {
    type Output = LinkTarget;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('(')?;
        let href = input.parse(ParseHref)?;
        let title = input.parse(ParseQuotedText);
        input.parse(')')?;

        input.apply();
        Some(LinkTarget::Url { href, title })
    }
}

pub(super) struct ParseLinkTargetReference;

impl Parse for ParseLinkTargetReference {
    type Output = LinkTarget;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('[')?;
        let reference = input.parse(Until(|c| matches!(c, ']' | '\r' | '\n')))?;
        input.parse(']')?;

        input.apply();
        Some(LinkTarget::Reference(reference))
    }
}

struct ParseHref;

impl Parse for ParseHref {
    type Output = String;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        let mut s = String::new();

        let mut parens = 0;
        let mut ws = false;
        let mut esc = false;

        while let Some(c) = input.peek_char() {
            let prev_esc = replace(&mut esc, false);
            let prev_ws = replace(&mut ws, false);

            match c {
                _ if c.is_ascii_whitespace() => {
                    if prev_esc {
                        s.push('\\');
                    }
                    s.push(c);
                    ws = true;
                }
                '\\' => {
                    if prev_esc {
                        s.push('\\');
                    } else {
                        esc = true;
                    }
                }
                '(' => {
                    s.push('(');
                    if !prev_esc {
                        parens += 1;
                    }
                }
                ')' => {
                    if prev_esc {
                        s.push(')');
                    } else if parens > 0 {
                        s.push(')');
                        parens -= 1;
                    } else {
                        break;
                    }
                }
                '"' if prev_ws => {
                    break;
                }
                _ => {
                    if prev_esc {
                        s.push('\\');
                    }
                    s.push(c);
                }
            }

            input.bump(c.len_utf8());
        }

        input.apply();
        Some(s)
    }
}

struct ParseQuotedText;

impl Parse for ParseQuotedText {
    type Output = String;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();
        input.parse('"')?;

        let mut s = String::new();
        let mut esc = false;

        while let Some(c) = input.peek_char() {
            let prev_esc = replace(&mut esc, false);

            match c {
                '\\' => {
                    if prev_esc {
                        s.push('\\');
                    } else {
                        esc = true;
                    }
                }
                '"' => {
                    if prev_esc {
                        s.push('"');
                    } else {
                        input.bump(1);
                        break;
                    }
                }
                _ => {
                    if prev_esc {
                        s.push('\\');
                    }
                    s.push(c);
                }
            }

            input.bump(c.len_utf8());
        }

        input.apply();
        Some(s)
    }
}
