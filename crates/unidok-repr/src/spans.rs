use std::convert::TryInto;

use detached_str::StrSlice;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyntaxSpan(pub SyntaxKind, pub Span);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn until(self, other: Span) -> Span {
        Span { start: self.start, end: other.end }
    }

    pub fn with(self, kind: SyntaxKind) -> SyntaxSpan {
        SyntaxSpan(kind, self)
    }
}

impl From<StrSlice> for Span {
    fn from(s: StrSlice) -> Self {
        Span { start: s.start().try_into().unwrap(), end: s.end().try_into().unwrap() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[non_exhaustive]
pub enum SyntaxKind {
    InlineFormatting,
    Italic,
    Bold,
    Strikethrough,
    Superscript,
    Subscript,
    InlineCode,

    Heading,
    AtxHeading,
    SetextHeading1,
    SetextHeading2,
    AtxHeadingMarker,
    SetextHeadingMarker,

    Link,
    LinkText,
    LinkRef,
    LinkHref,
    LinkTitle,
    LinkRefDef,

    Image,
    ImageAltText,
    ImageHref,
    ImageTitle,

    Footnote,

    Blockquote,
    BlockquoteMarker,

    List,
    OrderedList,
    UnorderedList,
    ListMarker,

    ThematicBreak,

    CodeBlock,
    CodeFence,
    InfoString,

    Table,
    TableCell,
    TableCellMeta,

    Math,
    MathContent,

    Limiter,

    Comment,

    HtmlTag,
    HtmlTagName,
    HtmlAttrName,
    HtmlAttrValue,

    HtmlDoctype,
    HtmlCdata,
    HtmlComment,
    HtmlEntity,

    Macro,
    MacroName,
    MacroArg,
    MacroKey,
    MacroArgString,
    MacroArgList,
    CurlyBraces,

    Escaped,
}

#[cfg(feature = "serde-spans")]
mod serde_impls {
    use serde::ser::SerializeTuple;
    use serde::Serialize;

    use crate::{Span, SyntaxKind, SyntaxSpan};

    impl Serialize for SyntaxSpan {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut tup = serializer.serialize_tuple(3)?;
            tup.serialize_element(&self.0)?;
            tup.serialize_element(&self.1.start)?;
            tup.serialize_element(&self.1.end)?;
            tup.end()
        }
    }

    impl Serialize for Span {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut tup = serializer.serialize_tuple(2)?;
            tup.serialize_element(&self.start)?;
            tup.serialize_element(&self.end)?;
            tup.end()
        }
    }

    impl Serialize for SyntaxKind {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_u8(*self as u8)
        }
    }
}
