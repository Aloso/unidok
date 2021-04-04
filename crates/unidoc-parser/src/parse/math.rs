use crate::{Input, Parse, StrSlice};

/// A math block, e.g.
///
/// ```markdown
/// The solution is %{a + b = 5}.
/// ```
pub struct Math {
    pub text: StrSlice,
}

pub struct ParseMath;

impl Parse for ParseMath {
    type Output = Math;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();
        input.parse("%{")?;
        let text = input.parse(MathContent)?;
        input.parse('}')?;
        input.apply();
        Some(Math { text })
    }
}

pub struct MathContent;
impl Parse for MathContent {
    type Output = StrSlice;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();
        while let Some(c) = input.peek_char() {
            match c {
                '[' => {
                    input.parse(MathContent)?;
                    input.parse(']')?;
                }
                '(' => {
                    input.parse(MathContent)?;
                    input.parse(')')?;
                }
                '{' => {
                    input.parse(MathContent)?;
                    input.parse('}')?;
                }
                ')' | ']' | '}' => {
                    break;
                }
                c => {
                    input.bump(c.len_utf8() as usize);
                }
            }
        }
        let text = input.apply();
        let text = text.get(2..text.len() - 1);
        Some(text)
    }
}
