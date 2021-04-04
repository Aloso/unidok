use crate::Input;

pub use self::basic::{UntilChar, UntilStr};
pub use self::nodes::{Node, NodeParentKind, ParseNode, ParseNodes};

pub mod attributes;
pub mod basic;
pub mod braces;
pub mod code_blocks;
pub mod comments;
pub mod escapes_limiters;
pub mod headings;
pub mod hr;
pub mod images;
pub mod indent;
pub mod inline;
pub mod links;
pub mod lists;
pub mod macros;
pub mod marker;
pub mod math;
pub mod nodes;
pub mod quotes;
pub mod statics;
pub mod subst_text;
pub mod tables;
pub mod text;

/// The trait to implement for parsers.
pub trait Parse {
    type Output;

    fn parse(&self, input: &mut Input) -> Option<Self::Output>;
}
