use detached_str::StrSlice;

use crate::utils::ParseLineBreak;
use crate::{Input, Parse};

use super::Indents;

/// Parses ASCII text case-insensitively
pub(crate) struct AsciiCI<T>(pub T);

impl<'a> Parse for AsciiCI<&'a str> {
    type Output = StrSlice;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let len = self.0.len();
        let slice = input.rest().get(0..len)?;

        if self.0.eq_ignore_ascii_case(slice) {
            Some(input.bump(len))
        } else {
            None
        }
    }
}

pub(crate) struct QuoteMark;

pub(crate) struct ClosingQuoteMark(pub QuoteMarkType);

#[derive(PartialEq)]
pub(crate) enum QuoteMarkType {
    Single,
    Double,
}

impl Parse for QuoteMark {
    type Output = QuoteMarkType;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        match input.peek_char() {
            Some('"') => {
                input.bump(1);
                Some(QuoteMarkType::Double)
            }
            Some('\'') => {
                input.bump(1);
                Some(QuoteMarkType::Single)
            }
            _ => None,
        }
    }
}

impl Parse for ClosingQuoteMark {
    type Output = ();

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        match self.0 {
            QuoteMarkType::Single => input.parse('\'')?,
            QuoteMarkType::Double => input.parse('"')?,
        };
        Some(())
    }
}

pub(crate) struct QuotedString<'a>(pub Indents<'a>);

impl Parse for QuotedString<'_> {
    type Output = String;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();
        let quote = input.parse(QuoteMark)?;
        let mut content = String::new();

        loop {
            let rest = input.rest();
            let idx = rest.find(|c| matches!(c, '"' | '\'' | '\n' | '\r'))?;
            match rest[idx..].bytes().next() {
                Some(b'"') if quote == QuoteMarkType::Double => {
                    content.push_str(&rest[..idx]);
                    input.bump(idx);
                    break;
                }
                Some(b'\'') if quote == QuoteMarkType::Single => {
                    content.push_str(&rest[..idx]);
                    input.bump(idx);
                    break;
                }
                Some(b'\n' | b'\r') => {
                    content.push_str(&rest[..idx]);
                    input.bump(idx);
                    input.parse(ParseLineBreak(self.0))?;
                }
                _ => {
                    content.push_str(&rest[..idx + 1]);
                    input.bump(idx + 1);
                }
            }
        }

        input.parse(ClosingQuoteMark(quote))?;
        input.apply();
        Some(content)
    }
}

pub(crate) struct QuotedStringWithEscapes<'a>(pub Indents<'a>);

impl Parse for QuotedStringWithEscapes<'_> {
    type Output = String;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();
        let quote = input.parse(QuoteMark)?;
        let mut content = String::new();

        loop {
            let rest = input.rest();
            let idx = rest.find(|c| matches!(c, '"' | '\'' | '\\' | '\n' | '\r'))?;
            match rest[idx..].as_bytes() {
                [b'"', ..] if quote == QuoteMarkType::Double => {
                    content.push_str(&rest[..idx]);
                    input.bump(idx);
                    break;
                }
                [b'\'', ..] if quote == QuoteMarkType::Single => {
                    content.push_str(&rest[..idx]);
                    input.bump(idx);
                    break;
                }
                [b'\\', b'"' | b'\'' | b'\\', ..] => {
                    content.push_str(&rest[..idx]);
                    input.bump(idx + 1);
                    let escaped = input.bump(1);
                    content.push_str(&input[escaped]);
                }
                [b'\n' | b'\r', ..] => {
                    content.push_str(&rest[..idx]);
                    input.bump(idx);
                    input.parse(ParseLineBreak(self.0))?;
                }
                _ => {
                    content.push_str(&rest[..idx + 1]);
                    input.bump(idx + 1);
                }
            }
        }

        input.parse(ClosingQuoteMark(quote))?;
        input.apply();
        Some(content)
    }
}
