use crate::input::Input;
use crate::parse::Parse;
use crate::StrSlice;

/// Parses ASCII text case-insensitively
pub struct AsciiCI<T>(pub T);

impl<'a> Parse for AsciiCI<&'a str> {
    type Output = StrSlice;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let len = self.0.len();
        let slice = input.rest().get(0..len)?;

        if self.0.eq_ignore_ascii_case(slice) {
            Some(input.bump(len))
        } else {
            None
        }
    }
}

pub struct QuoteMark;

pub struct ClosingQuoteMark(pub QuoteMarkType);

pub enum QuoteMarkType {
    Single,
    Double,
}

impl Parse for QuoteMark {
    type Output = QuoteMarkType;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
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

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        match self.0 {
            QuoteMarkType::Single => input.parse('\'')?,
            QuoteMarkType::Double => input.parse('"')?,
        };
        Some(())
    }
}
