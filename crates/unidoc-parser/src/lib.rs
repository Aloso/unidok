#[cfg(test)]
pub mod statics;
#[cfg(test)]
#[macro_use]
mod test_macros;

pub mod containers;
pub mod inlines;
pub mod leaves;

mod utils;

mod nodes;

mod input;
mod parse;

pub use detached_str as str;

pub use crate::input::Input;
pub use crate::nodes::{Node, NodeCtx};
pub use crate::parse::Parse;

use crate::utils::cond::{UntilChar, WhileChar};
