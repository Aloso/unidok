use crate::{Input, Parse, StrSlice};

impl Parse for char {
    type Output = StrSlice;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        if input.rest().starts_with(*self) {
            Some(input.bump(self.len_utf8() as usize))
        } else {
            None
        }
    }
}

impl Parse for &str {
    type Output = StrSlice;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        if input.rest().starts_with(*self) {
            Some(input.bump(self.len()))
        } else {
            None
        }
    }
}

impl Parse for String {
    type Output = StrSlice;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        if input.rest().starts_with(self.as_str()) {
            Some(input.bump(self.len()))
        } else {
            None
        }
    }
}
