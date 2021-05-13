use std::convert::TryFrom;

use unidoc_repr::ast::html::{HtmlEntity, HtmlNode};
use unidoc_repr::ast::macros::InlineMacro;
use unidoc_repr::ast::segments::*;

use super::code::ParseCode;
use super::escaped::ParseEscaped;
use super::format::{is_in_word, is_not_flanking, FlankType, Flanking, FormatDelim};
use super::images::ParseImage;
use super::limiters::ParseLimiter;
use super::links::ParseLink;
use super::math::ParseMath;
use crate::blocks::{
    Context, ParseCodeBlock, ParseComment, ParseHeading, ParseLinkRefDef, ParseList, ParseQuote,
    ParseTable, ParseThematicBreak, Underline,
};
use crate::html::elem::ParseHtmlElem;
use crate::html::entities::ParseHtmlEntity;
use crate::html::node::ParseHtmlNode;
use crate::macros::utils::ParseClosingBrace;
use crate::macros::ParseInlineMacro;
use crate::parsing_mode::ParsingMode;
use crate::utils::{ParseLineBreak, While};
use crate::{Indents, Input, Parse, StrSlice};

pub fn strip_space_start(segment: &mut Segment, input: &Input) -> bool {
    match segment {
        Segment::Text(s) if s.to_str(input.text()).starts_with(' ') => {
            *s = s.get(1..);
            true
        }
        Segment::Text2(s) if s.starts_with(' ') => {
            *s = &s[1..];
            true
        }
        _ => false,
    }
}

pub fn strip_space_end(segment: &mut Segment, input: &Input) -> bool {
    match segment {
        Segment::Text(s) if s.to_str(input.text()).ends_with(' ') => {
            *s = s.get(..s.len() - 1);
            true
        }
        Segment::Text2(s) if s.ends_with(' ') => {
            *s = &s[..s.len() - 1];
            true
        }
        _ => false,
    }
}

#[derive(Debug)]
pub enum Segments {
    Empty,
    Some { segments: Vec<Segment>, underline: Option<Underline> },
}

impl Segments {
    pub(crate) fn parser(
        ind: Indents<'_>,
        context: Context,
        mode: ParsingMode,
    ) -> ParseSegments<'_> {
        ParseSegments { ind, context, mode }
    }

    pub fn into_segments_no_underline(self) -> Option<Vec<Segment>> {
        match self {
            Segments::Some { segments, underline: None } => Some(segments),
            _ => None,
        }
    }

    pub fn into_segments_no_underline_zero(self) -> Option<Vec<Segment>> {
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
    Code(Code),
    Math(Math),
    Link(Link),
    Image(Image),
    Macro(InlineMacro),
    Html(HtmlNode),
    HtmlEntity(HtmlEntity),
    Escaped(Escaped),
    LineBreak,
    Limiter,
    Underline(Underline),
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
    Code(Code),
    Math(Math),
    Link(Link),
    Image(Image),
    Macro(InlineMacro),
    Html(HtmlNode),
    HtmlEntity(HtmlEntity),
    Escaped(Escaped),
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
            Item::LineBreak => stack.push(StackItem::LineBreak),
            Item::Limiter => stack.push(StackItem::Limiter),
            Item::Underline(_) => unreachable!("Unexpected underline"),
        }
    }

    stack
}

fn stack_to_segments(stack: Vec<StackItem>) -> Vec<Segment> {
    let mut result = Vec::with_capacity(stack.len());

    for it in stack {
        result.push(match it {
            StackItem::Text(t) => Segment::Text(t),
            StackItem::Text2(t) => Segment::Text2(t),
            StackItem::Formatted { delim, content } => {
                let delim_inner = match *content.as_slice() {
                    [StackItem::Formatted { delim, .. }] => Some(delim),
                    _ => None,
                };

                let mut content = stack_to_segments(content);
                if matches!(delim, FormatDelim::Underscore | FormatDelim::Star)
                    && content.len() == 1
                    && Some(delim) == delim_inner
                {
                    let popped = content.pop().unwrap();
                    if let Segment::Format(InlineFormat {
                        formatting: Formatting::Italic,
                        segments: content_inner,
                    }) = popped
                    {
                        Segment::Format(InlineFormat {
                            formatting: Formatting::Bold,
                            segments: content_inner,
                        })
                    } else {
                        content.push(popped);
                        Segment::Format(InlineFormat {
                            formatting: delim.to_format(),
                            segments: content,
                        })
                    }
                } else {
                    Segment::Format(InlineFormat {
                        formatting: delim.to_format(),
                        segments: content,
                    })
                }
            }
            StackItem::Code(c) => Segment::Code(c),
            StackItem::Math(m) => Segment::Math(m),
            StackItem::Link(l) => Segment::Link(l),
            StackItem::Image(i) => Segment::Image(i),
            StackItem::Macro(m) => Segment::InlineMacro(m),
            StackItem::Html(h) => Segment::InlineHtml(h),
            StackItem::HtmlEntity(e) => Segment::HtmlEntity(e),
            StackItem::Escaped(e) => Segment::Escaped(e),
            StackItem::LineBreak => Segment::LineBreak,
            StackItem::Limiter => Segment::Limiter,
            StackItem::FormatDelim { delim, .. } => Segment::Text2(delim.to_str()),
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

#[inline]
#[allow(unused_parens)]
fn find_special(c: char) -> bool {
    matches!(
        c,
        ('*' | '_' | '~' | '^' | '#' | '`')
            | ('%' | '|' | '[' | ']' | '{' | '}' | '!' | '@' | '\\' | '$' | '<' | '&')
            | ('\n' | '\r')
    )
}

impl ParseSegments<'_> {
    fn lex_items(&self, input: &mut Input) -> Option<(Vec<Item>, Option<Underline>)> {
        let mut items = Vec::new();
        let mut open_brackets = 0;
        let mut open_braces = 0;

        loop {
            let skip_bytes = input.rest().find(find_special).unwrap_or_else(|| input.len());

            if skip_bytes > 0 {
                items.push(Item::Text(input.bump(skip_bytes)));
            }

            if input.is_empty() {
                break;
            }

            if self.handle_char(
                input,
                &mut items,
                input.peek_char().unwrap(),
                &mut open_brackets,
                &mut open_braces,
            )? {
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
        sym: char,
        open_brackets: &mut u32,
        open_braces: &mut u32,
    ) -> Option<bool> {
        let ind = self.ind;
        let context = self.context;

        if self.mode.is(ParsingMode::INLINE) {
            if let Ok(delim) = FormatDelim::try_from(sym) {
                let left = input.prev_char();
                let cs = input.parse_i(While(sym));
                let right = input.peek_char();

                if (sym == '_' && is_in_word(left, right)) || is_not_flanking(left, right) {
                    items.push(Item::Text(cs));
                } else {
                    let count = (cs.len() % 3) as u8;

                    let left_flank =
                        left.map(FlankType::from_char).unwrap_or(FlankType::Whitespace);
                    let right_flank =
                        right.map(FlankType::from_char).unwrap_or(FlankType::Whitespace);
                    let flanking = Flanking::new(left_flank, right_flank);

                    for _ in 0..cs.len() {
                        items.push(Item::FormatDelim { delim, count, flanking });
                    }
                }
                return Some(false);
            }
        }

        match sym {
            '`' => {
                if let Context::Code(len) = context {
                    let backticks = input.parse_i(While('`')).len();
                    if backticks == len as usize {
                        return Some(true);
                    }
                }

                if self.mode.is(ParsingMode::INLINE) {
                    if let Some(code) = input.parse(ParseCode { ind, mode: None }) {
                        items.push(Item::Code(code));
                        return Some(false);
                    }
                }

                items.push(Item::Text(input.parse_i(While('`'))));
                return Some(false);
            }
            '%' => {
                if self.mode.is(ParsingMode::MATH) {
                    if let Some(math) = input.parse(ParseMath { ind }) {
                        items.push(Item::Math(math));
                        return Some(false);
                    }
                }
            }
            '!' => {
                if self.mode.is(ParsingMode::LINKS_IMAGES) {
                    if let Some(img) = input.parse(ParseImage { ind }) {
                        items.push(Item::Image(img));
                        return Some(false);
                    }
                }
            }
            '@' => {
                if self.mode.is(ParsingMode::MACROS) {
                    if let Some(mac) = input.parse(ParseInlineMacro { ind, mode: Some(self.mode) })
                    {
                        items.push(Item::Macro(mac));
                        return Some(false);
                    }
                }
            }
            '\\' => {
                if self.mode.is(ParsingMode::INLINE) {
                    if let Some(esc) = input.parse(ParseEscaped) {
                        items.push(Item::Escaped(esc));
                        return Some(false);
                    }
                }
            }
            '$' => {
                if self.mode.is(ParsingMode::LIMITER) && input.parse(ParseLimiter).is_some() {
                    items.push(Item::Limiter);
                    return Some(false);
                }
            }
            '<' => {
                if self.mode.is(ParsingMode::HTML) {
                    if let Some(html) = input.parse(ParseHtmlNode { ind }) {
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
            '&' => {
                if self.mode.is(ParsingMode::HTML) {
                    if let Some(entity) = input.parse(ParseHtmlEntity) {
                        items.push(Item::HtmlEntity(entity));
                        return Some(false);
                    }
                }
            }

            '|' => {
                if context == Context::Table {
                    return Some(true);
                }
            }

            '[' => {
                if self.mode.is(ParsingMode::LINKS_IMAGES) {
                    if let Some(link) = input.parse(ParseLink { ind }) {
                        items.push(Item::Link(link));
                        return Some(false);
                    }
                }

                *open_brackets += 1;
            }
            ']' => {
                if context == Context::LinkOrImg && *open_brackets == 0 {
                    return Some(true);
                }

                if *open_brackets > 0 {
                    *open_brackets -= 1;
                }
            }

            '{' => {
                *open_braces += 1;
            }
            '}' => {
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

            '\n' | '\r' => {
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

        self.mode.is(P::CODE_BLOCKS) && input.can_parse(ParseCodeBlock { ind, mode: None })
            || self.mode.is(P::COMMENTS) && input.can_parse(ParseComment)
            || self.mode.is(P::HEADINGS) && input.can_parse(ParseHeading { ind, no_toc: true })
            || self.mode.is(P::TABLES) && input.can_parse(ParseTable { ind })
            || self.mode.is(P::LISTS)
                && input.can_parse(ParseList { ind, is_loose: false, list_style: &mut None })
            || self.mode.is(P::THEMATIC_BREAKS) && input.can_parse(ParseThematicBreak { ind })
            || self.mode.is(P::QUOTES) && input.can_parse(ParseQuote { ind })
            || self.mode.is(P::LINKS_IMAGES) && input.can_parse(ParseLinkRefDef { ind })
    }
}

fn is_blank_line(s: &str) -> bool {
    let s = s.trim_start_matches(|c| matches!(c, ' ' | '\t'));
    matches!(s.bytes().next(), Some(b'\n' | b'\r') | None)
}
