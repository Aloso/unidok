use crate::input::Input;
use crate::{Parse, ParseInfallible, StrSlice};

/// Parses until the given pattern (a `&str`, `char` or a closure accepting a
/// `char`) matches.
///
/// The match itself isn't included. This parser can be used either as fallible
/// or as infallible parser:
///
/// * In the fallible version, [`None`] is returned when the pattern isn't found
/// * In the infallible version, the entire string is returned when the pattern
///   isn't found
///
/// ### Example for the infallible version
///
/// ```compile_fail
/// let mut input = Input::new("****!");
/// input.parse_i(Until('!'));
/// assert_eq!(input.rest(), "!");
/// ```
///
/// ### Example for the fallible version
///
/// ```compile_fail
/// let mut input = Input::new("****!");
/// assert!(input.parse(Until('!')).is_some());
/// assert!(input.parse(Until('#')).is_none());
/// ```
pub struct Until<T>(pub T);

impl<F: Fn(char) -> bool> ParseInfallible for Until<F> {
    type Output = StrSlice;

    #[inline]
    fn parse_infallible(&self, input: &mut Input) -> Self::Output {
        match input.rest().find(&self.0) {
            Some(i) => input.bump(i),
            None => input.bump(input.len()),
        }
    }
}

impl ParseInfallible for Until<char> {
    type Output = StrSlice;

    #[inline]
    fn parse_infallible(&self, input: &mut Input) -> Self::Output {
        match input.rest().find(self.0) {
            Some(i) => input.bump(i),
            None => input.bump(input.len()),
        }
    }
}

impl<'a> ParseInfallible for Until<&'a str> {
    type Output = StrSlice;

    #[inline]
    fn parse_infallible(&self, input: &mut Input) -> Self::Output {
        match input.rest().find(self.0) {
            Some(i) => input.bump(i),
            None => input.bump(input.len()),
        }
    }
}

impl<F: Fn(char) -> bool> Parse for Until<F> {
    type Output = StrSlice;

    #[inline]
    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        input.rest().find(&self.0).map(|i| input.bump(i))
    }
}

impl Parse for Until<char> {
    type Output = StrSlice;

    #[inline]
    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        input.rest().find(self.0).map(|i| input.bump(i))
    }
}

impl<'a> Parse for Until<&'a str> {
    type Output = StrSlice;

    #[inline]
    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        input.rest().find(self.0).map(|i| input.bump(i))
    }
}

#[test]
fn test1() {
    let mut input = Input::new("****!");
    input.parse_i(Until('!'));
    assert_eq!(input.rest(), "!");
}

#[test]
fn test2() {
    let mut input = Input::new("****!");
    assert!(input.parse(Until('!')).is_some());
    assert!(input.parse(Until('#')).is_none());
}
