use std::convert::TryInto;

use crate::utils::ParseLineBreak;
use crate::{Input, Parse, ParseInfallible};

use super::{Indents, While};

/// Returns whether this is a space or tab.
#[inline]
pub fn is_ws(c: char) -> bool {
    matches!(c, ' ' | '\t')
}

/// Parses 0-255 spaces or tabs. One tab counts as 4 spaces. This parser never
/// fails.
pub struct ParseSpacesU8;

/// Parses 0 or more spaces or tabs. This parser never fails.
pub struct ParseSpaces;

/// Parses 1 whitespace character.
pub struct ParseOneWS;

/// Parses at least _n_ spaces. It can also parse tabs, where 1 tab corresponds
/// to 4 spaces. It _tries_ to parse _exactly_ n spaces, but this is not always
/// possible in the presence of tabs.
pub struct ParseNSpaces(pub u8);

/// Parses at most _n_ spaces. It can also parse tabs, where 1 tab corresponds
/// to 4 spaces.
pub struct ParseAtMostNSpaces(pub u8);

impl Parse for ParseOneWS {
    type Output = ();

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        if let Some(b' ' | b'\t') = input.rest().bytes().next() {
            input.bump(1);
            Some(())
        } else {
            None
        }
    }
}

impl Parse for ParseSpacesU8 {
    type Output = u8;

    #[inline]
    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut res = 0usize;
        let mut len = 0;
        let rest = input.rest();
        for c in rest.bytes() {
            match c {
                b' ' => {
                    res += 1;
                    len += 1;
                }
                b'\t' => {
                    res += 4;
                    len += 1;
                }
                _ => break,
            }
        }
        if len > 0 {
            input.bump(len);
        }
        res.try_into().ok()
    }
}

impl ParseInfallible for ParseSpaces {
    type Output = ();

    #[inline]
    fn parse_infallible(&self, input: &mut Input) -> Self::Output {
        input.parse_i(While(is_ws));
    }
}

impl Parse for ParseNSpaces {
    type Output = ();

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut visual_spaces = 0u8;
        let mut bytes = 0;

        for c in input.rest().bytes() {
            match c {
                b' ' => {
                    visual_spaces += 1;
                    bytes += 1;
                }
                b'\t' => {
                    visual_spaces += 4;
                    bytes += 1;
                }
                _ => break,
            }
            if visual_spaces >= self.0 {
                input.bump(bytes);
                return Some(());
            }
        }
        None
    }
}

impl Parse for ParseAtMostNSpaces {
    type Output = u8;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut visual_spaces = 0u8;
        let mut bytes = 0;

        for c in input.rest().bytes() {
            match c {
                b' ' => {
                    visual_spaces += 1;
                    bytes += 1;
                }
                b'\t' => {
                    visual_spaces += 4;
                    bytes += 1;
                }
                _ => break,
            }
            if visual_spaces == self.0 {
                break;
            }
        }

        if bytes > 0 {
            input.bump(bytes);
        }
        Some(visual_spaces)
    }
}

pub struct ParseWs<'a>(pub Indents<'a>);

impl ParseInfallible for ParseWs<'_> {
    type Output = ();

    fn parse_infallible(&self, input: &mut Input) -> Self::Output {
        input.parse_i(While(is_ws));

        while matches!(input.peek_char(), Some('\n' | '\r')) {
            if input.parse(ParseLineBreak(self.0)).is_none() {
                break;
            }

            input.parse_i(While(is_ws));
        }
    }
}

pub struct ParseWsNoBlankLinkes<'a>(pub Indents<'a>);

impl Parse for ParseWsNoBlankLinkes<'_> {
    type Output = ();

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        input.parse_i(While(is_ws));

        if input.parse(ParseLineBreak(self.0)).is_some() {
            input.parse_i(While(is_ws));

            if matches!(input.peek_char(), Some('\n' | '\r')) {
                return None;
            }
        }
        Some(())
    }
}
