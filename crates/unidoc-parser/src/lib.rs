#[cfg(test)]
pub mod statics;
#[cfg(test)]
#[macro_use]
mod test_macros;

mod attributes;
mod braces;
mod code_blocks;
mod comments;
mod escapes;
mod headings;
mod hr;
mod images;
mod indent;
mod inline;
mod limiters;
mod line_breaks;
mod links;
mod lists;
mod macros;
mod marker;
mod math;
mod nodes;
mod quotes;
mod tables;
mod text;

mod basic;
mod input;
mod parse;

pub use detached_str as str;

pub use crate::input::Input;
pub use crate::parse::Parse;

use crate::basic::{UntilChar, WhileChar};

pub mod items {
    pub use crate::attributes::Attribute;
    pub use crate::braces::Braces;
    pub use crate::code_blocks::CodeBlock;
    pub use crate::comments::Comment;
    pub use crate::escapes::Escaped;
    pub use crate::headings::Heading;
    pub use crate::hr::HorizontalLine;
    pub use crate::images::Image;
    pub use crate::indent::Indents;
    pub use crate::inline::{Formatting, InlineFormat};
    pub use crate::limiters::Limiter;
    pub use crate::line_breaks::LineBreak;
    pub use crate::links::Link;
    pub use crate::lists::{List, ListKind};
    pub use crate::macros::Macro;
    pub use crate::math::Math;
    pub use crate::nodes::{Node, ParentKind};
    pub use crate::quotes::Quote;
    pub use crate::tables::Table;
    pub use crate::text::Text;
}
