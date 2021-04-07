use crate::indent::{ParseNSpaces, ParseSpaces};
use crate::marker::{ParseLineEnd, ParseLineStart};
use crate::Input;

/// The trait to implement for parsers.
pub trait Parse {
    type Output;

    /// The parse function.
    fn parse(&self, input: &mut Input) -> Option<Self::Output>;

    /// Line start marker (doesn't bump the input).
    ///
    /// This ignores required indentation after a line break, e.g. list
    /// indentation and quote markers.
    const LINE_START: ParseLineStart = ParseLineStart;

    /// Line end marker (doesn't bumpt the input).
    ///
    /// Although Unidoc allows trailing whitespace almost everywhere, this
    /// parser only works if trailing whitespace has already been bumped.
    const LINE_END: ParseLineEnd = ParseLineEnd;

    /// Whitespace parser. Parses spaces and tabs.
    ///
    /// Returns the number of spaces. One tab corresponds to 4 spaces.
    const WS: ParseSpaces = ParseSpaces;

    /// A parser that parses an exact amount of spaces or tabs. One tab
    /// corresponds to 4 spaces.
    fn spaces(n: u8) -> ParseNSpaces {
        ParseNSpaces(n)
    }
}
