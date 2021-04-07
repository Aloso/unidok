#[cfg(test)]
pub mod statics;
#[cfg(test)]
#[macro_use]
mod test_macros;

pub mod containers;
pub mod inlines;
pub mod leaves;

mod utils;

mod attributes;
mod indent;
mod line_breaks;
mod nodes;

mod input;
mod parse;

pub use detached_str as str;

pub use crate::input::Input;
pub use crate::parse::Parse;

use crate::utils::cond::{UntilChar, WhileChar};

pub mod items {
    pub use crate::attributes::Attribute;
    pub use crate::indent::Indents;
    pub use crate::line_breaks::LineBreak;
    pub use crate::nodes::{Node, NodeCtx};
}
