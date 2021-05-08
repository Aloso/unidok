use crate::{Input, Parse};

/// The special "limiter" character, `$`. When it's next to a node that is not
/// text, a comment, line break or escape sequence, it isn't rendered.
///
/// The limiter can be inserted in certain places to logically divide them,
/// without affecting the generated document. For example, if you want to make
/// part of a word bold:
///
/// ```markdown
/// Bord**eau**x
/// ```
///
/// This doesn't work, because inline formatting must be at word boundaries.
/// However, inserting `$` makes it work:
///
/// ```markdown
/// Bord$**eau**$x
/// ```
///
/// Another example is if you want to write a macro, like `@TOC`, and then
/// braces immediately afterwards. To ensure that the braces are not parsed as
/// part of the macro, insert a `$` in between: `@TOC${...}`
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limiter;

impl Limiter {
    pub(crate) fn parser() -> ParseLimiter {
        ParseLimiter
    }
}

pub(crate) struct ParseLimiter;

impl Parse for ParseLimiter {
    type Output = Limiter;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        input.parse('$')?;
        Some(Limiter)
    }
}
