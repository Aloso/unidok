use crate::{Input, Parse};

/// Parses 0 or more spaces or tabs. This parser never fails.
pub struct ParseSpaces;

pub struct ParseNSpaces(pub u8);

pub struct ParseAtMostNSpaces(pub u8);

impl Parse for ParseSpaces {
    type Output = u8;

    #[inline]
    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
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
        Some(res)
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
