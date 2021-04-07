use crate::Input;

/// The trait to implement for parsers.
pub trait Parse {
    type Output;

    /// The parse function.
    fn parse(&self, input: &mut Input) -> Option<Self::Output>;
}
