use crate::indent::Indents;
use crate::items::LineBreak;
use crate::{Input, Parse};

/// A math block, e.g.
///
/// ```markdown
/// The solution is %{a + b = 5}.
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Math {
    pub text: String,
}

impl Math {
    pub fn parser(ind: Indents<'_>) -> ParseMath<'_> {
        ParseMath { ind }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ParseMath<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseMath<'_> {
    type Output = Math;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse("%{")?;
        let text = input.parse(ParseMathContent { ind: self.ind })?;
        input.parse('}')?;

        input.apply();
        Some(Math { text })
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct ParseMathContent<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseMathContent<'_> {
    type Output = String;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        let mut text = String::new();
        let mut esc = false;

        while let Some(c) = input.peek_char() {
            if esc {
                if !matches!(c, '[' | '(' | '{' | '\\' | '\n' | ')' | ']' | '}') {
                    text.push('\\');
                }
                text.push(c);
                input.bump(c.len_utf8() as usize);
                esc = false;
            } else {
                match c {
                    ')' | ']' | '}' => {
                        break;
                    }
                    '\n' => {
                        if input.parse(LineBreak::parser(self.ind)).is_some() {
                            text.push('\n');
                            continue;
                        }
                        break;
                    }
                    '[' | '(' | '{' => {
                        text.push(c);
                        input.bump(1);
                        let inner = input.parse(*self)?;
                        text.push_str(&inner);

                        let end_char = closing(c);
                        input.parse(end_char)?;
                        text.push(end_char);
                    }
                    '\\' => {
                        input.bump(1);
                        esc = true;
                    }
                    _ => {
                        text.push(c);
                        input.bump(c.len_utf8() as usize);
                    }
                }
            }
        }

        input.apply();
        Some(text)
    }
}

fn closing(brace: char) -> char {
    match brace {
        '(' => ')',
        '[' => ']',
        _ => '}',
    }
}

#[test]
fn test_math() {
    let mut input = Input::new(r#"%{A}%{f() + g(h(%{}))}%{\}\(()}%{ \A\B + \(A\B\) }"#);

    assert_eq!(input.parse(ParseMath::default()), Some(Math { text: "A".into() }));
    assert_eq!(
        input.parse(ParseMath::default()),
        Some(Math { text: "f() + g(h(%{}))".into() })
    );
    assert_eq!(input.parse(ParseMath::default()), Some(Math { text: "}(()".into() }));
    assert_eq!(
        input.parse(ParseMath::default()),
        Some(Math { text: r#" \A\B + (A\B) "#.into() })
    );
}
