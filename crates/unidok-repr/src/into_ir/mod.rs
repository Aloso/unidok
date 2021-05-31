mod blocks;
mod foreign_impls;
mod html;
mod macros;
mod segments;
mod utils;

use crate::ast::AstData;

pub trait IntoIR<'a> {
    type IR: 'a;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR;
}
