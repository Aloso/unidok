use crate::Input;

/// The trait to implement for parsers.
pub trait Parse {
    type Output;

    fn parse(&self, input: &mut Input) -> Option<Self::Output>;
}
