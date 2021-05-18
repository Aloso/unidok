mod blocks_impl;
mod code_blocks;
mod comments;
mod headings;
mod link_ref_defs;
mod lists;
mod quotes;
mod tables;
mod thematic_breaks;

pub(crate) use blocks_impl::ParseBlock;
pub(crate) use code_blocks::ParseCodeBlock;
pub(crate) use comments::ParseComment;
pub(crate) use headings::{ParseHeading, Underline};
pub(crate) use link_ref_defs::ParseLinkRefDef;
pub(crate) use lists::ParseList;
pub(crate) use quotes::ParseQuote;
pub(crate) use tables::ParseTable;
pub(crate) use thematic_breaks::ParseThematicBreak;

use unidok_repr::ast::html::ElemName;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum Context {
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
