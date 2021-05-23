use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ParsingMode(u16);

impl ParsingMode {
    /// inline, i
    pub const INLINE: PmParam = PmParam(1 << 0);
    /// codeblock, c
    pub const CODE_BLOCKS: PmParam = PmParam(1 << 1);
    /// heading, h
    pub const HEADINGS: PmParam = PmParam(1 << 2);
    /// tbreak, b
    pub const THEMATIC_BREAKS: PmParam = PmParam(1 << 3);
    /// subst, s
    pub const SUBSTITUTIONS: PmParam = PmParam(1 << 4);
    /// list, l
    pub const LISTS: PmParam = PmParam(1 << 5);
    /// limiter, $
    pub const LIMITER: PmParam = PmParam(1 << 6);
    /// macro, @
    pub const MACROS: PmParam = PmParam(1 << 7);
    /// math, %
    pub const MATH: PmParam = PmParam(1 << 8);
    /// table, |
    pub const TABLES: PmParam = PmParam(1 << 9);
    /// quote, >
    pub const QUOTES: PmParam = PmParam(1 << 10);
    /// html, <
    pub const HTML: PmParam = PmParam(1 << 11);
    /// comment, /
    pub const COMMENTS: PmParam = PmParam(1 << 12);
    /// link_img, li
    pub const LINKS_IMAGES: PmParam = PmParam(1 << 13);

    pub fn new_all() -> Self {
        Self(0b11_1111_1111_1111)
    }

    pub fn new_nothing() -> Self {
        Self(0)
    }

    pub fn set(mut self, PmParam(n): PmParam) -> Self {
        self.0 |= n;
        self
    }

    pub fn unset(mut self, PmParam(n): PmParam) -> Self {
        self.0 &= n ^ !0;
        self
    }

    pub fn is(&self, PmParam(n): PmParam) -> bool {
        (self.0 & n) != 0
    }

    pub fn is_nothing(&self) -> bool {
        self.0 == 0
    }

    pub fn parse_param(word: &str) -> Option<PmParam> {
        word.parse().ok()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct PmParam(u16);

impl FromStr for PmParam {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "inline" | "i" => ParsingMode::INLINE,
            "codeblock" | "c" => ParsingMode::CODE_BLOCKS,
            "heading" | "h" => ParsingMode::HEADINGS,
            "tbreak" | "b" => ParsingMode::THEMATIC_BREAKS,
            "subst" | "s" => ParsingMode::SUBSTITUTIONS,
            "list" | "l" => ParsingMode::LISTS,
            "limiter" | "$" => ParsingMode::LIMITER,
            "macro" | "@" => ParsingMode::MACROS,
            "math" | "%" => ParsingMode::MATH,
            "table" | "|" => ParsingMode::TABLES,
            "quote" | ">" => ParsingMode::QUOTES,
            "html" | "<" => ParsingMode::HTML,
            "link_img" | "li" => ParsingMode::LINKS_IMAGES,
            _ => return Err(()),
        })
    }
}

#[test]
fn test_parsing_modes() {
    let pm = ParsingMode::new_all();

    assert!(pm.is(ParsingMode::INLINE));
    assert!(pm.is(ParsingMode::CODE_BLOCKS));
    assert!(pm.is(ParsingMode::HEADINGS));
    assert!(pm.is(ParsingMode::THEMATIC_BREAKS));
    assert!(pm.is(ParsingMode::SUBSTITUTIONS));
    assert!(pm.is(ParsingMode::LISTS));
    assert!(pm.is(ParsingMode::LIMITER));
    assert!(pm.is(ParsingMode::MACROS));
    assert!(pm.is(ParsingMode::MATH));
    assert!(pm.is(ParsingMode::TABLES));
    assert!(pm.is(ParsingMode::QUOTES));
    assert!(pm.is(ParsingMode::HTML));
    assert!(pm.is(ParsingMode::COMMENTS));
    assert!(pm.is(ParsingMode::LINKS_IMAGES));

    let pm = ParsingMode::new_nothing();

    assert!(!pm.is(ParsingMode::INLINE));
    assert!(!pm.is(ParsingMode::CODE_BLOCKS));
    assert!(!pm.is(ParsingMode::HEADINGS));
    assert!(!pm.is(ParsingMode::THEMATIC_BREAKS));
    assert!(!pm.is(ParsingMode::SUBSTITUTIONS));
    assert!(!pm.is(ParsingMode::LISTS));
    assert!(!pm.is(ParsingMode::LIMITER));
    assert!(!pm.is(ParsingMode::MACROS));
    assert!(!pm.is(ParsingMode::MATH));
    assert!(!pm.is(ParsingMode::TABLES));
    assert!(!pm.is(ParsingMode::QUOTES));
    assert!(!pm.is(ParsingMode::HTML));
    assert!(!pm.is(ParsingMode::COMMENTS));
    assert!(!pm.is(ParsingMode::LINKS_IMAGES));

    let pm = ParsingMode::new_nothing().set(ParsingMode::MACROS).set(ParsingMode::COMMENTS);

    assert!(!pm.is(ParsingMode::INLINE));
    assert!(!pm.is(ParsingMode::CODE_BLOCKS));
    assert!(!pm.is(ParsingMode::HEADINGS));
    assert!(!pm.is(ParsingMode::THEMATIC_BREAKS));
    assert!(!pm.is(ParsingMode::SUBSTITUTIONS));
    assert!(!pm.is(ParsingMode::LISTS));
    assert!(!pm.is(ParsingMode::LIMITER));
    assert!(pm.is(ParsingMode::MACROS));
    assert!(!pm.is(ParsingMode::MATH));
    assert!(!pm.is(ParsingMode::TABLES));
    assert!(!pm.is(ParsingMode::QUOTES));
    assert!(!pm.is(ParsingMode::HTML));
    assert!(pm.is(ParsingMode::COMMENTS));
    assert!(!pm.is(ParsingMode::LINKS_IMAGES));
}
