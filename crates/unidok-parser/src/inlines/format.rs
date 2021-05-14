use std::cmp::Ordering;
use std::convert::TryFrom;

use unidok_repr::ast::segments::Formatting;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Flanking {
    /// ` **Hello`
    Left,
    /// `Hello** `
    Right,
    /// `Hello**world`
    Both,
}

impl Flanking {
    pub(crate) fn new(left: FlankType, right: FlankType) -> Flanking {
        match left.cmp(&right) {
            Ordering::Less => Flanking::Left,
            Ordering::Equal => Flanking::Both,
            Ordering::Greater => Flanking::Right,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum FlankType {
    Limiter = 0,
    Whitespace = 1,
    Punctuation = 2,
    Alphanumeric = 3,
}

impl FlankType {
    pub(crate) fn from_char(c: char) -> FlankType {
        if c.is_whitespace() {
            FlankType::Whitespace
        } else if c == '$' {
            FlankType::Limiter
        } else if !c.is_alphanumeric() {
            FlankType::Punctuation
        } else {
            FlankType::Alphanumeric
        }
    }
}

pub(crate) fn is_in_word(prev: Option<char>, next: Option<char>) -> bool {
    prev.filter(|c| c.is_alphanumeric()).is_some() && next.filter(|c| c.is_alphanumeric()).is_some()
}

pub(crate) fn is_not_flanking(prev: Option<char>, next: Option<char>) -> bool {
    #[inline]
    fn is_white(c: Option<char>) -> bool {
        match c {
            Some(c) => c.is_whitespace(),
            None => true,
        }
    }

    is_white(prev) && is_white(next)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FormatDelim {
    /// Italic -> bold
    Star,
    /// Italic -> bold
    Underscore,
    /// Strikethrough
    Tilde,
    /// Superscript
    Caret,
    /// Subscript
    NumberSign,
}

impl FormatDelim {
    pub fn to_str(self) -> &'static str {
        match self {
            FormatDelim::Star => "*",
            FormatDelim::Underscore => "_",
            FormatDelim::Tilde => "~",
            FormatDelim::Caret => "^",
            FormatDelim::NumberSign => "#",
        }
    }

    pub fn to_format(self) -> Formatting {
        match self {
            FormatDelim::Star | FormatDelim::Underscore => Formatting::Italic,
            FormatDelim::Tilde => Formatting::StrikeThrough,
            FormatDelim::Caret => Formatting::Superscript,
            FormatDelim::NumberSign => Formatting::Subscript,
        }
    }
}

impl TryFrom<char> for FormatDelim {
    type Error = ();

    #[inline]
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '*' => FormatDelim::Star,
            '_' => FormatDelim::Underscore,
            '~' => FormatDelim::Tilde,
            '^' => FormatDelim::Caret,
            '#' => FormatDelim::NumberSign,
            _ => return Err(()),
        })
    }
}
