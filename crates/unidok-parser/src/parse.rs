use crate::Input;

/// The trait to implement for parsers.
pub trait Parse {
    type Output;

    /// The parse function.
    fn parse(&mut self, input: &mut Input) -> Option<Self::Output>;

    /// Checks if the output can be successfully parsed. However, it only
    /// returns `true` or `false`, so it is potentially cheaper than
    /// `.parse().is_some()`.
    ///
    /// The input must NOT be bumped, irregardless of whether or not the parser
    /// succeeds.
    fn can_parse(&mut self, input: &mut Input) -> bool {
        self.parse(&mut input.start()).is_some()
    }
}

/// The trait to implement for parsers.
pub trait ParseInfallible {
    type Output;

    /// The parse function.
    fn parse_infallible(&self, input: &mut Input) -> Self::Output;
}
