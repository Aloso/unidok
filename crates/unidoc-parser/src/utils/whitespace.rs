use crate::{Input, Parse};

pub struct ParseSpaces;

pub struct ParseNSpaces(pub u8);

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
        const SPACES: &str = "                                                                                                                                                                                                                                                                ";
        let spaces = &SPACES[..self.0 as usize];
        input.parse(spaces)?;
        Some(())
    }
}
