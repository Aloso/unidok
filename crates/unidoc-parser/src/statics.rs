use crate::containers::*;
use crate::inlines::*;
use crate::leaves::*;
use crate::str::StrSlice;
use crate::Node;

pub trait IsStatic {
    type Static: 'static;

    fn is(&self, s: Self::Static, str: &str) -> bool;
}

impl IsStatic for StrSlice {
    type Static = &'static str;

    fn is(&self, s: &'static str, str: &str) -> bool {
        &str[self.range()] == s
    }
}

impl IsStatic for Text {
    type Static = &'static str;

    fn is(&self, s: &'static str, str: &str) -> bool {
        &str[self.0.range()] == s
    }
}

impl IsStatic for String {
    type Static = &'static str;

    fn is(&self, s: &'static str, _str: &str) -> bool {
        self.as_str() == s
    }
}

impl<T: IsStatic> IsStatic for Option<T> {
    type Static = Option<T::Static>;

    fn is(&self, s: Option<T::Static>, str: &str) -> bool {
        match (self, s) {
            (Some(a), Some(b)) => a.is(b, str),
            (None, None) => true,
            _ => false,
        }
    }
}

impl<T: IsStatic> IsStatic for Vec<T>
where
    T::Static: Copy,
{
    type Static = &'static [T::Static];

    fn is(&self, s: Self::Static, str: &str) -> bool {
        self.len() == s.len() && self.iter().zip(s.iter()).all(|(a, b)| a.is(*b, str))
    }
}

macro_rules! impl_is_static {
    () => {};

    // identities
    (identity $name:ty; $($rest:tt)*) => {
        impl IsStatic for $name {
            type Static = $name;

            fn is(&self, s: Self::Static, _str: &str) -> bool {
                *self == s
            }
        }

        impl_is_static!($($rest)*);
    };

    // structs
    (
        $v:vis struct $static_name:ident for $name:ty {
            $(
                $v2:vis $field:ident : $t:ty
            ),* $(,)?
        }
        $($rest:tt)*
    ) => {
        #[derive(Debug, Copy, Clone)]
        $v struct $static_name {
            $( $v2 $field : $t ),*
        }

        impl IsStatic for $name {
            type Static = $static_name;

            fn is(&self, _s: Self::Static, _str: &str) -> bool {
                $(
                    if !self.$field.is(_s.$field, _str) {
                        return false;
                    }
                )*
                true
            }
        }

        impl_is_static!($($rest)*);
    };

    // enums
    (
        $v:vis enum $static_name:ident for $name:ident {
            $(
                $variant:ident($t:ty)
            ),* $(,)?
        }
        $($rest:tt)*
    ) => {
        #[derive(Debug, Copy, Clone)]
        $v enum $static_name {
            $( $variant ($t) ),*
        }

        impl IsStatic for $name {
            type Static = $static_name;

            fn is(&self, _rhs: $static_name, _str: &str) -> bool {
                match (self, _rhs) {
                    $(
                        (
                            $name::$variant(a),
                            $static_name::$variant(b)
                        ) => a.is(b, _str),
                    )*
                    _ => false,
                }
            }
        }

        impl_is_static!($($rest)*);
    }
}

impl_is_static! {
    identity usize;
    identity u8;
    identity bool;
    identity ();

    pub struct StaticEscaped for Escaped {
        pub text: &'static str,
    }

    pub struct StaticBraces for Braces {
        pub first_line: Option<&'static [StaticSegment]>,
        pub content: &'static [StaticNode],
    }

    pub struct StaticMath for Math {
        pub text: &'static str,
    }

    pub struct StaticInlineFormat for InlineFormat {
        pub formatting: Formatting,
        pub content: &'static [StaticSegment],
    }

    identity Formatting;

    pub struct StaticLink for Link {
        pub href: &'static str,
        pub text: &'static [StaticSegment],
        pub title: Option<&'static str>,
    }

    pub struct StaticImage for Image {
        pub href: &'static str,
        pub alt: &'static [StaticSegment],
        pub title: Option<&'static str>,
    }

    pub struct StaticMacro for Macro {
        pub name: &'static str,
        pub args: Option<&'static str>,
        pub content: StaticBraces,
    }

    pub struct StaticComment for Comment {
        pub content: &'static str,
    }

    pub struct StaticThematicBreak for ThematicBreak {
        pub len: usize,
    }

    pub struct StaticCodeBlock for CodeBlock {
        pub info: &'static str,
        pub backticks: usize,
        pub lines: &'static [&'static str],
        pub indent: u8,
    }

    pub struct StaticHeading for Heading {
        pub level: u8,
        pub content: &'static [StaticSegment],
    }

    pub struct StaticList for List {
        pub indent_spaces: u8,
        pub bullet: Bullet,
        pub content: &'static [&'static [StaticNode]],
    }

    identity Bullet;

    pub struct StaticQuote for Quote {
        pub content: &'static [StaticNode],
    }

    pub struct StaticTable for Table {
        pub content: &'static [StaticTableRow],
    }

    pub enum StaticTableRow for TableRow {
        Content(&'static [&'static [StaticSegment]]),
        Line(&'static [ColumnKind]),
    }

    identity ColumnKind;

    pub struct StaticParagraph for Paragraph {
        pub segments: &'static [StaticSegment],
    }

    pub enum StaticSegment for Segment {
        LineBreak(LineBreak),
        Text(&'static str),
        Escaped(StaticEscaped),
        Limiter(Limiter),
        Braces(StaticBraces),
        Math(StaticMath),
        Link(StaticLink),
        Image(StaticImage),
        Macro(StaticMacro),
        InlineFormat(StaticInlineFormat),
    }

    pub enum StaticNode for Node {
        Paragraph(StaticParagraph),
        Comment(StaticComment),
        ThematicBreak(StaticThematicBreak),
        CodeBlock(StaticCodeBlock),
        Heading(StaticHeading),
        List(StaticList),
        Quote(StaticQuote),
        Table(StaticTable),
    }

    identity LineBreak;
    identity Limiter;
}
