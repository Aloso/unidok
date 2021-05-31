use detached_str::StrSlice;

use crate::ast::AstData;
use crate::IntoIR;

impl<'a> IntoIR<'a> for StrSlice {
    type IR = &'a str;

    fn into_ir(self, text: &'a str, _: &mut AstData) -> Self::IR {
        self.to_str(text)
    }
}

impl<'a> IntoIR<'a> for () {
    type IR = ();

    fn into_ir(self, _: &'a str, _: &mut AstData) -> Self::IR {}
}

impl<'a, T: IntoIR<'a>> IntoIR<'a> for Vec<T> {
    type IR = Vec<T::IR>;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR {
        self.into_iter().map(|t| t.into_ir(text, data)).collect()
    }
}

impl<'a, T: IntoIR<'a>> IntoIR<'a> for Box<T> {
    type IR = Box<T::IR>;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR {
        Box::new((*self).into_ir(text, data))
    }
}

impl<'a, T: IntoIR<'a>> IntoIR<'a> for Option<T> {
    type IR = Option<T::IR>;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR {
        self.map(|t| t.into_ir(text, data))
    }
}
