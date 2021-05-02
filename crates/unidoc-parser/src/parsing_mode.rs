#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsingMode(u16);

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

    pub fn new_all() -> Self {
        Self(!0)
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
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PmParam(u16);
