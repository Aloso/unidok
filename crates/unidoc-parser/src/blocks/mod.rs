mod blocks_impl;
mod code_blocks;
mod comments;
mod headings;
mod link_ref_defs;
mod lists;
mod paragraphs;
mod quotes;
mod tables;
mod thematic_breaks;

pub use blocks_impl::Block;
pub use code_blocks::{CodeBlock, Fence};
pub use comments::Comment;
pub use headings::{Heading, HeadingKind, Underline};
pub use link_ref_defs::LinkRefDef;
pub use lists::{Bullet, List};
pub use paragraphs::Paragraph;
pub use quotes::Quote;
pub use tables::{Bius, CellAlignment, CellMeta, Table, TableCell, TableRow};
pub use thematic_breaks::{ThematicBreak, ThematicBreakKind};

use crate::html::ElemName;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Context {
    InlineBraces,
    BlockBraces,
    Table,
    LinkOrImg,
    Code(u8),
    CodeBlock,
    Heading,
    InlineHtml(ElemName),
    BlockHtml(ElemName),
    Global,
}

impl Context {
    pub fn can_contain_block_macro(self) -> bool {
        !matches!(self, Context::InlineBraces | Context::LinkOrImg | Context::Code(_))
    }

    pub fn get_parent(self) -> Option<ElemName> {
        match self {
            Context::InlineHtml(e) => Some(e),
            _ => None,
        }
    }
}
