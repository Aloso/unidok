use std::convert::TryFrom;

use aho_corasick::AhoCorasick;
use detached_str::StrSlice;
use unidok_repr::ast::html::{HtmlEntity, HtmlNodeAst};
use unidok_repr::ast::macros::InlineMacroAst;
use unidok_repr::ast::segments::*;

use super::code::ParseCode;
use super::escaped::ParseEscaped;
use super::format::{is_in_word, is_not_flanking, FlankType, Flanking, FormatDelim};
use super::images::ParseImage;
use super::limiters::ParseLimiter;
use super::links::ParseLink;
use super::math::ParseMath;
use crate::blocks::{
    ParseCodeBlock, ParseComment, ParseHeading, ParseLinkRefDef, ParseList, ParseQuote, ParseTable,
    ParseThematicBreak, Underline,
};
use crate::html::elem::ParseHtmlElem;
use crate::html::entities::ParseHtmlEntity;
use crate::html::node::ParseHtmlNode;
use crate::macros::utils::ParseClosingBrace;
use crate::macros::ParseInlineMacro;
use crate::parsing_mode::ParsingMode;
use crate::utils::{is_ws, ParseLineBreak, While};
use crate::{Context, Indents, Input, Parse};

pub fn strip_space_start(segment: &mut SegmentAst, input: &Input) -> bool {
    match segment {
        SegmentAst::Text(s) if s.to_str(input.text()).starts_with(' ') => {
            *s = s.get(1..);
            true
        }
        SegmentAst::Text2(s) if s.starts_with(' ') => {
            *s = &s[1..];
            true
        }
        _ => false,
    }
}

pub fn strip_space_end(segment: &mut SegmentAst, input: &Input) -> bool {
    match segment {
        SegmentAst::Text(s) if s.to_str(input.text()).ends_with(' ') => {
            *s = s.get(..s.len() - 1);
            true
        }
        SegmentAst::Text2(s) if s.ends_with(' ') => {
            *s = &s[..s.len() - 1];
            true
        }
        _ => false,
    }
}

#[derive(Debug)]
pub(crate) enum Segments {
    Empty,
    Some { segments: Vec<SegmentAst>, underline: Option<Underline> },
}

impl Segments {
    pub(crate) fn parser<'a>(
        ind: Indents<'a>,
        context: Context,
        mode: ParsingMode,
        ac: &'a AhoCorasick,
    ) -> ParseSegments<'a> {
        ParseSegments { ind, context, mode, ac }
    }

    pub fn into_segments_no_underline(self) -> Option<Vec<SegmentAst>> {
        match self {
            Segments::Some { segments, underline: None } => Some(segments),
            _ => None,
        }
    }

    pub fn into_segments_no_underline_zero(self) -> Option<Vec<SegmentAst>> {
        match self {
            Segments::Some { segments, underline: None } => Some(segments),
            _ => Some(vec![]),
        }
    }
}

pub(crate) struct ParseSegments<'a> {
    ind: Indents<'a>,
    context: Context,
    mode: ParsingMode,
    ac: &'a AhoCorasick,
}

impl Parse for ParseSegments<'_> {
    type Output = Segments;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let (items, underline) = self.lex_items(input)?;
        if items.is_empty() {
            return Some(Segments::Empty);
        }

        let stack = parse_paragraph_items(items);
        let segments = stack_to_segments(stack);

        Some(Segments::Some { segments, underline })
    }
}

#[derive(Debug, Clone)]
enum Item {
    Text(StrSlice),
    FormatDelim {
        /// the type of delimiter
        delim: FormatDelim,
        /// whether the delimiter is left-flanking, right-flanking or both
        flanking: Flanking,
        /// number of characters in the delimiter (mod 3)
        count: u8,
    },
    Code(CodeAst),
    Math(MathAst),
    Link(LinkAst),
    Image(ImageAst),
    Macro(InlineMacroAst),
    Html(HtmlNodeAst),
    HtmlEntity(HtmlEntity),
    Escaped(Escaped),
    Substitution(Substitution),
    LineBreak,
    Limiter,
    Underline(Underline),
}

impl Item {
    fn is_blank_text(&self, input: &Input) -> bool {
        matches!(*self, Item::Text(t) if input[t].trim_start_matches(is_ws).is_empty())
    }

    fn can_appear_before_limiter(&self) -> bool {
        matches!(
            self,
            Item::Code(_)
                | Item::Escaped(_)
                | Item::FormatDelim { .. }
                | Item::Link(LinkAst { text: None, .. })
        )
    }
}

impl Default for Item {
    fn default() -> Self {
        Item::Text(StrSlice::default())
    }
}

#[derive(Debug)]
enum StackItem {
    Text(StrSlice),
    Text2(&'static str),
    Formatted {
        delim: FormatDelim,
        content: Vec<StackItem>,
    },
    FormatDelim {
        /// the type of delimiter
        delim: FormatDelim,
        /// whether the delimiter is left-flanking, right-flanking, or both
        flanking: Flanking,
        /// number of characters in the delimiter (mod 3)
        count: u8,
    },
    Code(CodeAst),
    Math(MathAst),
    Link(LinkAst),
    Image(ImageAst),
    Macro(InlineMacroAst),
    Html(HtmlNodeAst),
    HtmlEntity(HtmlEntity),
    Escaped(Escaped),
    Substitution(Substitution),
    LineBreak,
    Limiter,
}

impl StackItem {
    fn eliminate_delims(self) -> Self {
        match self {
            StackItem::FormatDelim { delim, .. } => StackItem::Text2(delim.to_str()),
            it => it,
        }
    }
}

fn parse_paragraph_items(items: Vec<Item>) -> Vec<StackItem> {
    let mut stack = Vec::new();
    for it in items {
        match it {
            Item::Text(t) => stack.push(StackItem::Text(t)),
            Item::FormatDelim { delim, flanking, count } => {
                if let Flanking::Right | Flanking::Both = flanking {
                    if let Some(i) = find_matching_opening_delim(&stack, delim, flanking, count) {
                        let content =
                            stack.drain(i + 1..).map(StackItem::eliminate_delims).collect();
                        stack.pop();
                        stack.push(StackItem::Formatted { delim, content });
                    } else if flanking == Flanking::Both {
                        stack.push(StackItem::FormatDelim { delim, flanking, count });
                    } else {
                        stack.push(StackItem::Text2(delim.to_str()));
                    }
                } else {
                    stack.push(StackItem::FormatDelim { delim, flanking, count });
                }
            }
            Item::Code(c) => stack.push(StackItem::Code(c)),
            Item::Math(m) => stack.push(StackItem::Math(m)),
            Item::Link(l) => stack.push(StackItem::Link(l)),
            Item::Image(i) => stack.push(StackItem::Image(i)),
            Item::Macro(m) => stack.push(StackItem::Macro(m)),
            Item::Html(h) => stack.push(StackItem::Html(h)),
            Item::HtmlEntity(e) => stack.push(StackItem::HtmlEntity(e)),
            Item::Escaped(e) => stack.push(StackItem::Escaped(e)),
            Item::Substitution(s) => stack.push(StackItem::Substitution(s)),
            Item::LineBreak => stack.push(StackItem::LineBreak),
            Item::Limiter => stack.push(StackItem::Limiter),
            Item::Underline(_) => unreachable!("Unexpected underline"),
        }
    }

    stack
}

fn stack_to_segments(stack: Vec<StackItem>) -> Vec<SegmentAst> {
    let mut result = Vec::with_capacity(stack.len());

    for it in stack {
        result.push(match it {
            StackItem::Text(t) => SegmentAst::Text(t),
            StackItem::Text2(t) => SegmentAst::Text2(t),
            StackItem::Formatted { delim, content } => {
                let is_same_delim = matches!(
                    *content.as_slice(),
                    [StackItem::Formatted { delim: delim_inner, .. }] if delim == delim_inner
                );

                let mut segments = stack_to_segments(content);

                if is_same_delim && segments.len() == 1 {
                    match segments.pop().unwrap() {
                        SegmentAst::Format(InlineFormatAst {
                            formatting: Formatting::Italic,
                            segments,
                        }) => SegmentAst::Format(InlineFormatAst {
                            formatting: Formatting::Bold,
                            segments,
                        }),
                        SegmentAst::Format(f) if f.formatting != Formatting::Bold => {
                            SegmentAst::Format(f)
                        }
                        segment => {
                            segments.push(segment);
                            SegmentAst::Format(InlineFormatAst {
                                formatting: delim.to_format(),
                                segments,
                            })
                        }
                    }
                } else {
                    SegmentAst::Format(InlineFormatAst { formatting: delim.to_format(), segments })
                }
            }
            StackItem::Code(c) => SegmentAst::Code(c),
            StackItem::Math(m) => SegmentAst::Math(m),
            StackItem::Link(l) => SegmentAst::Link(l),
            StackItem::Image(i) => SegmentAst::Image(i),
            StackItem::Macro(m) => SegmentAst::InlineMacro(m),
            StackItem::Html(h) => SegmentAst::InlineHtml(h),
            StackItem::HtmlEntity(e) => SegmentAst::HtmlEntity(e),
            StackItem::Escaped(e) => SegmentAst::Escaped(e),
            StackItem::Substitution(e) => SegmentAst::Substitution(e),
            StackItem::LineBreak => SegmentAst::LineBreak,
            StackItem::Limiter => SegmentAst::Limiter,
            StackItem::FormatDelim { delim, .. } => SegmentAst::Text2(delim.to_str()),
        })
    }

    result
}

fn find_matching_opening_delim(
    stack: &[StackItem],
    right_delim: FormatDelim,
    right_flanking: Flanking,
    right_count: u8,
) -> Option<usize> {
    let mut same_delim_run = true;

    for (i, el) in stack.iter().enumerate().rev() {
        if let StackItem::FormatDelim {
            delim: left_delim,
            flanking: left_flanking,
            count: left_count,
        } = *el
        {
            if left_delim == right_delim {
                if let Flanking::Left | Flanking::Both = left_flanking {
                    if !same_delim_run
                        && is_compatible(left_flanking, right_flanking, left_count, right_count)
                    {
                        return Some(i);
                    }
                }
            } else {
                same_delim_run = false;
            }
        } else if let StackItem::Limiter = *el {
        } else {
            same_delim_run = false;
        }
    }

    None
}

/// <https://spec.commonmark.org/0.29/#emphasis-and-strong-emphasis>:
///
/// > If one of the delimiters can both open and close strong emphasis, then
/// > the sum of the lengths of the delimiter runs containing the opening and
/// > closing delimiters must not be a multiple of 3 unless both lengths are
/// > multiples of 3.
///
/// Note that left_count and right_count were taken (mod 3), so they're in
/// {0, 1, 2}.
fn is_compatible(left: Flanking, right: Flanking, left_count: u8, right_count: u8) -> bool {
    if let (Flanking::Left, Flanking::Right) = (left, right) {
        true
    } else {
        left_count + right_count != 3
    }
}

pub(crate) static PATTERNS: &[&str] = &[
    "*", "_", "~", "^", "#", "`", "%{", "|", "[", "]", "{", "}", "!", "@", "\\", "$", "<", "&",
    "\n", "\r", "'", "\"", "...", "--",
];

pub(crate) fn get_global_patterns() -> AhoCorasick {
    AhoCorasick::new_auto_configured(PATTERNS)
}

mod patterns {
    pub const STAR: u32 = 0;
    pub const UNDERSCORE: u32 = 1;
    pub const TILDE: u32 = 2;
    pub const CARET: u32 = 3;
    pub const NUMBER_SIGN: u32 = 4;
    pub const BACKTICK: u32 = 5;
    pub const PERCENT_BRACE: u32 = 6;
    pub const PIPE: u32 = 7;
    pub const OPEN_BRACKET: u32 = 8;
    pub const CLOSE_BRACKET: u32 = 9;
    pub const OPEN_BRACE: u32 = 10;
    pub const CLOSE_BRACE: u32 = 11;
    pub const EXCL_MARK: u32 = 12;
    pub const AT: u32 = 13;
    pub const BACKSLASH: u32 = 14;
    pub const DOLLAR: u32 = 15;
    pub const OPEN_ANGLE: u32 = 16;
    pub const AMPERSAND: u32 = 17;
    pub const LINE_FEED: u32 = 18;
    pub const CARRIAGE_RETURN: u32 = 19;
    pub const SINGLE_QUOTE: u32 = 20;
    pub const DOUBLE_QUOTE: u32 = 21;
    pub const ELLIPSIS: u32 = 22;
    pub const EM_DASH: u32 = 23;
}

fn pattern_to_format_delim(n: u32) -> Option<FormatDelim> {
    Some(match n {
        patterns::STAR => FormatDelim::Star,
        patterns::UNDERSCORE => FormatDelim::Underscore,
        patterns::TILDE => FormatDelim::Tilde,
        patterns::CARET => FormatDelim::Caret,
        patterns::NUMBER_SIGN => FormatDelim::NumberSign,
        _ => return None,
    })
}

impl ParseSegments<'_> {
    fn lex_items(&self, input: &mut Input) -> Option<(Vec<Item>, Option<Underline>)> {
        let mut items = Vec::new();
        let mut open_brackets = 0;
        let mut open_braces = 0;

        loop {
            let r#match = self.ac.find(input.rest());
            let (skip_bytes, sym) =
                r#match.map(|m| (m.start(), m.pattern())).unwrap_or_else(|| (input.len(), 0));

            if skip_bytes > 0 {
                items.push(Item::Text(input.bump(skip_bytes)));
            }

            if input.is_empty() {
                break;
            }

            let sym = sym as u32;
            if self.handle_char(input, &mut items, sym, &mut open_brackets, &mut open_braces)? {
                break;
            }
        }

        let underline = if let Some(&Item::Underline(u)) = items.last() {
            items.pop();
            Some(u)
        } else {
            None
        };

        Some((items, underline))
    }

    /// Returns `true` if the loop should be exited
    fn handle_char(
        &self,
        input: &mut Input,
        items: &mut Vec<Item>,
        sym: u32,
        open_brackets: &mut u32,
        open_braces: &mut u32,
    ) -> Option<bool> {
        let ind = self.ind;
        let context = self.context;

        if self.mode.is(ParsingMode::INLINE) {
            if let Some(delim) = pattern_to_format_delim(sym) {
                let c = input.peek_char().unwrap();

                let left = input.prev_char();
                let delim_run = input.parse_i(While(c));
                let right = input.peek_char();

                if (delim == FormatDelim::Underscore && is_in_word(left, right))
                    || is_not_flanking(left, right)
                {
                    items.push(Item::Text(delim_run));
                } else {
                    let count = (delim_run.len() % 3) as u8;

                    let left_flank =
                        left.map(FlankType::from_char).unwrap_or(FlankType::Whitespace);
                    let right_flank =
                        right.map(FlankType::from_char).unwrap_or(FlankType::Whitespace);
                    let flanking = Flanking::new(left_flank, right_flank);

                    for _ in 0..delim_run.len() {
                        items.push(Item::FormatDelim { delim, count, flanking });
                    }
                }
                return Some(false);
            }
        }

        match sym {
            patterns::BACKTICK => {
                if let Context::Code(len) = context {
                    let backticks = input.parse_i(While('`')).len();
                    if backticks == len as usize {
                        return Some(true);
                    }
                }

                if self.mode.is(ParsingMode::INLINE) {
                    if let Some(code) = input.parse(ParseCode { ind, mode: None, ac: self.ac }) {
                        items.push(Item::Code(code));
                        return Some(false);
                    }
                }

                items.push(Item::Text(input.parse_i(While('`'))));
                return Some(false);
            }
            patterns::PERCENT_BRACE => {
                if self.mode.is(ParsingMode::MATH) {
                    if let Some(math) = input.parse(ParseMath { ind }) {
                        items.push(Item::Math(math));
                        return Some(false);
                    }
                }
            }
            patterns::EXCL_MARK => {
                if self.mode.is(ParsingMode::LINKS_IMAGES) {
                    if let Some(img) =
                        input.parse(ParseImage { ind, ac: self.ac, mode: Some(self.mode) })
                    {
                        items.push(Item::Image(img));
                        return Some(false);
                    }
                }
            }
            patterns::AT => {
                if self.mode.is(ParsingMode::MACROS) {
                    if let Some(mac) =
                        input.parse(ParseInlineMacro { ind, mode: Some(self.mode), ac: self.ac })
                    {
                        items.push(Item::Macro(mac));
                        return Some(false);
                    }
                }
            }
            patterns::BACKSLASH => {
                if self.mode.is(ParsingMode::INLINE) {
                    if let Some(esc) = input.parse(ParseEscaped) {
                        items.push(Item::Escaped(esc));
                        return Some(false);
                    }
                }
            }
            patterns::DOLLAR => {
                if self.mode.is(ParsingMode::LIMITER) {
                    let parser_state = {
                        if matches!(items.last(), Some(i) if i.can_appear_before_limiter())
                            || matches!(input.rest()[1..].chars().next(),
                                Some(c) if FormatDelim::try_from(c).is_ok())
                        {
                            Some(false)
                        } else {
                            match items.iter().rev().find(|i| !i.is_blank_text(input)) {
                                Some(Item::LineBreak) | None => Some(true),
                                _ => None,
                            }
                        }
                    };

                    if let Some(require_line_end) = parser_state {
                        if input.parse(ParseLimiter { require_line_end }).is_some() {
                            items.push(Item::Limiter);
                            return Some(false);
                        }
                    }
                }
            }
            patterns::OPEN_ANGLE => {
                if self.mode.is(ParsingMode::HTML) {
                    if let Some(html) =
                        input.parse(ParseHtmlNode { ind, mode: Some(self.mode), ac: self.ac })
                    {
                        items.push(Item::Html(html));
                        return Some(false);
                    }
                }

                if let Context::InlineHtml(elem) | Context::BlockHtml(elem) = context {
                    if input.can_parse(ParseHtmlElem::closing_tag(elem)) {
                        return Some(true);
                    }
                }
            }
            patterns::AMPERSAND => {
                if self.mode.is(ParsingMode::HTML) {
                    if let Some(entity) = input.parse(ParseHtmlEntity) {
                        items.push(Item::HtmlEntity(entity));
                        return Some(false);
                    }
                }
            }

            patterns::PIPE => {
                if context == Context::Table {
                    return Some(true);
                }
            }

            patterns::OPEN_BRACKET => {
                if self.mode.is(ParsingMode::LINKS_IMAGES) {
                    if let Some(link) =
                        input.parse(ParseLink { ind, ac: self.ac, mode: Some(self.mode) })
                    {
                        items.push(Item::Link(link));
                        return Some(false);
                    }
                }

                *open_brackets += 1;
            }
            patterns::CLOSE_BRACKET => {
                if context == Context::LinkOrImg && *open_brackets == 0 {
                    return Some(true);
                }

                if *open_brackets > 0 {
                    *open_brackets -= 1;
                }
            }

            patterns::OPEN_BRACE => {
                *open_braces += 1;
            }
            patterns::CLOSE_BRACE => {
                if context == Context::InlineBraces && *open_braces == 0 {
                    return Some(true);
                }

                if context == Context::BlockBraces
                    && *open_braces == 0
                    && matches!(items.last(), Some(Item::LineBreak) | None)
                    && input.can_parse(ParseClosingBrace(ind))
                {
                    return Some(true);
                }

                if *open_braces > 0 {
                    *open_braces -= 1;
                }
            }

            patterns::SINGLE_QUOTE => {
                if self.mode.is(ParsingMode::SUBSTITUTIONS) {
                    let prev = input.prev_char();
                    input.bump(1);

                    if matches!(prev, Some(c) if c.is_alphabetic()) {
                        items.push(Item::Substitution(Substitution { text: "’" }));
                    } else if matches!(input.peek_char(), Some(c) if c.is_alphabetic()) {
                        items.push(Item::Substitution(Substitution { text: "‘" }));
                    } else {
                        items.push(Item::Substitution(Substitution { text: "’" }));
                    }
                    return Some(false);
                }
            }
            patterns::DOUBLE_QUOTE => {
                if self.mode.is(ParsingMode::SUBSTITUTIONS) {
                    let prev = input.prev_char();
                    input.bump(1);

                    if matches!(prev, Some(c) if c.is_alphabetic()) {
                        items.push(Item::Substitution(Substitution { text: "”" }));
                    } else if matches!(input.peek_char(), Some(c) if c.is_alphabetic()) {
                        items.push(Item::Substitution(Substitution { text: "“" }));
                    } else {
                        items.push(Item::Substitution(Substitution { text: "”" }));
                    }
                    return Some(false);
                }
            }
            patterns::ELLIPSIS => {
                if self.mode.is(ParsingMode::SUBSTITUTIONS) {
                    input.bump(3);
                    items.push(Item::Substitution(Substitution { text: "…" }));
                    return Some(false);
                }
            }
            patterns::EM_DASH => {
                if self.mode.is(ParsingMode::SUBSTITUTIONS) {
                    input.bump(2);
                    items.push(Item::Substitution(Substitution { text: "—" }));
                    return Some(false);
                }
            }

            patterns::LINE_FEED | patterns::CARRIAGE_RETURN => {
                if let Context::Table | Context::LinkOrImg | Context::Heading | Context::CodeBlock =
                    context
                {
                    return Some(true);
                }

                if input.parse(ParseLineBreak(ind)).is_some() {
                    items.push(Item::LineBreak);

                    if let Context::Global | Context::BlockBraces = context {
                        if let Some(u) = input.parse(Underline::parser(ind)) {
                            items.pop();
                            items.push(Item::Underline(u));
                            return Some(true);
                        }
                    }

                    if is_blank_line(input.rest()) || self.can_interrupt_paragraph(input) {
                        return Some(true);
                    }

                    return Some(false);
                } else {
                    return Some(true);
                }
            }
            _ => {}
        }

        items.push(Item::Text(input.bump(1)));
        Some(false)
    }

    fn can_interrupt_paragraph(&self, input: &mut Input) -> bool {
        use ParsingMode as P;

        let ind = self.ind;
        let ac = self.ac;

        self.mode.is(P::CODE_BLOCKS) && input.can_parse(ParseCodeBlock { ind, mode: None, ac })
            || self.mode.is(P::COMMENTS) && input.can_parse(ParseComment)
            || self.mode.is(P::HEADINGS) && input.can_parse(ParseHeading { ind, no_toc: true, ac })
            || self.mode.is(P::TABLES) && input.can_parse(ParseTable { ind, ac })
            || self.mode.is(P::LISTS) && input.can_parse(ParseList { ind, ac, mode: None })
            || self.mode.is(P::THEMATIC_BREAKS) && input.can_parse(ParseThematicBreak { ind })
            || self.mode.is(P::QUOTES) && input.can_parse(ParseQuote { ind, ac, mode: None })
            || self.mode.is(P::LINKS_IMAGES) && input.can_parse(ParseLinkRefDef { ind })
    }
}

fn is_blank_line(s: &str) -> bool {
    let s = s.trim_start_matches(is_ws);
    matches!(s.bytes().next(), Some(b'\n' | b'\r') | None)
}
