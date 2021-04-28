use crate::{Input, Parse, ParseInfallible};

/// Parses 0 or more spaces or tabs. This parser never fails.
///
/// TODO: This is sometimes used in situations where line breaks are desirable.
/// Add another parser for _arbitrary_ whitespace?
pub struct ParseSpaces;

/// Parses 1 whitespace character.
pub struct ParseOneWS;

/// Parses exactly _n_ spaces. It can also parse tabs, where 1 tab corresponds
/// to 4 spaces.
pub struct ParseNSpaces(pub u8);

/// Parses at most _n_ spaces. It can also parse tabs, where 1 tab corresponds
/// to 4 spaces.
pub struct ParseAtMostNSpaces(pub u8);

impl Parse for ParseOneWS {
    type Output = ();

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        if let Some(b' ' | b'\t') = input.rest().bytes().next() {
            input.bump(1);
            Some(())
        } else {
            None
        }
    }
}

impl ParseInfallible for ParseSpaces {
    type Output = u8;

    #[inline]
    fn parse_infallible(&self, input: &mut Input) -> Self::Output {
        let mut res = 0;
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
        res
    }
}

impl Parse for ParseNSpaces {
    type Output = ();

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
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
                input.bump(bytes);
                return Some(());
            }
        }
        None
    }
}

impl Parse for ParseAtMostNSpaces {
    type Output = u8;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
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
