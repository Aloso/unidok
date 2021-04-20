use std::convert::TryFrom;

use crate::blocks::paragraphs::ParseParagraph;
use crate::blocks::Underline;
use crate::str::StrSlice;
use crate::utils::{ParseLineBreak, ParseLineEnd, WhileChar};
use crate::{Context, Input};

use super::format::{is_in_word, is_not_flanking, FlankType, Flanking, FormatDelim};
use super::*;

#[derive(Debug, Clone)]
pub(crate) enum Item {
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
    Macro(Macro),
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
pub(crate) enum StackItem {
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
    Macro(Macro),
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

pub(crate) fn parse_paragraph_items(items: Vec<Item>) -> Vec<StackItem> {
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
            Item::Escaped(e) => stack.push(StackItem::Escaped(e)),
            Item::LineBreak => stack.push(StackItem::LineBreak),
            Item::Limiter => stack.push(StackItem::Limiter),
            Item::Underline(_) => unreachable!("Unexpected underline"),
        }
    }

    stack
}

pub(crate) fn stack_to_segments(stack: Vec<StackItem>) -> Vec<Segment> {
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
                        content: content_inner,
                    }) = popped
                    {
                        Segment::Format(InlineFormat {
                            formatting: Formatting::Bold,
                            content: content_inner,
                        })
                    } else {
                        content.push(popped);
                        Segment::Format(InlineFormat { formatting: delim.to_format(), content })
                    }
                } else {
                    Segment::Format(InlineFormat { formatting: delim.to_format(), content })
                }
            }
            StackItem::Code(c) => Segment::Code(c),
            StackItem::Math(m) => Segment::Math(m),
            StackItem::Link(l) => Segment::Link(l),
            StackItem::Image(i) => Segment::Image(i),
            StackItem::Macro(m) => Segment::Macro(m),
            StackItem::Escaped(e) => Segment::Escaped(e),
            StackItem::LineBreak => Segment::LineBreak(LineBreak),
            StackItem::Limiter => Segment::Limiter(Limiter),
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
fn find_special(c: char) -> bool {
    matches!(
        c,
        '*' | '_' | '~' | '^' | '#' | '`' | '%' | '[' | ']' | '!' | '@' | '\\' | '$' | '\n' | '\r'
    )
}

#[inline]
fn find_special_in_table(c: char) -> bool {
    matches!(c, '|') || find_special(c)
}

#[inline]
fn find_special_in_braces(c: char) -> bool {
    matches!(c, '}') || find_special(c)
}

#[inline]
fn find_special_in_link_or_img(c: char) -> bool {
    matches!(c, ']') || find_special(c)
}

#[inline]
fn find_special_in_code(c: char) -> bool {
    matches!(c, ']') || find_special(c)
}

fn find_special_for(s: &str, context: Context) -> Option<usize> {
    match context {
        Context::Global | Context::Heading => s.find(find_special),
        Context::Braces | Context::BracesFirstLine => s.find(find_special_in_braces),
        Context::Table => s.find(find_special_in_table),
        Context::LinkOrImg => s.find(find_special_in_link_or_img),
        Context::Code(_) => s.find(find_special_in_code),
    }
}

impl ParseParagraph<'_> {
    pub(crate) fn lex_paragraph_items(
        &self,
        input: &mut Input,
    ) -> Option<(Vec<Item>, Option<Underline>)> {
        let mut items = Vec::new();
        let mut open_brackets = 0;

        loop {
            let skip_bytes =
                find_special_for(input.rest(), self.context).unwrap_or_else(|| input.len());

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
    ) -> Option<bool> {
        let ind = self.ind;
        let context = self.context;

        if let Ok(delim) = FormatDelim::try_from(sym) {
            let left = input.prev_char();
            let cs = input.parse(WhileChar(sym))?;
            let right = input.peek_char();

            if (sym == '_' && is_in_word(left, right)) || is_not_flanking(left, right) {
                items.push(Item::Text(cs));
            } else {
                let count = (cs.len() % 3) as u8;

                let left_flank = left.map(FlankType::from_char).unwrap_or(FlankType::Whitespace);
                let right_flank = right.map(FlankType::from_char).unwrap_or(FlankType::Whitespace);
                let flanking = Flanking::new(left_flank, right_flank);

                for _ in 0..cs.len() {
                    items.push(Item::FormatDelim { delim, count, flanking });
                }
            }
        } else {
            match sym {
                '`' => {
                    if let Context::Code(len) = context {
                        let backticks = input.parse(WhileChar('`')).unwrap().len();
                        if backticks == len as usize {
                            return Some(true);
                        } else {
                            items.push(Item::Text(input.parse(WhileChar('`')).unwrap()));
                        }
                    } else if let Some(code) = input.parse(Code::parser(ind, false)) {
                        items.push(Item::Code(code));
                    } else {
                        items.push(Item::Text(input.parse(WhileChar('`')).unwrap()));
                    }
                }
                '%' => {
                    if let Some(math) = input.parse(Math::parser(ind)) {
                        items.push(Item::Math(math));
                    } else {
                        items.push(Item::Text(input.bump(1)));
                    }
                }
                '[' => {
                    if let Some(link) = input.parse(Link::parser(ind)) {
                        items.push(Item::Link(link));
                    } else {
                        *open_brackets += 1;
                        items.push(Item::Text(input.bump(1)));
                    }
                }
                '!' => {
                    if let Some(img) = input.parse(Image::parser(ind)) {
                        items.push(Item::Image(img));
                    } else {
                        items.push(Item::Text(input.bump(1)));
                    }
                }
                '@' => {
                    if let Some(mac) = input.parse(Macro::parser(ind)) {
                        items.push(Item::Macro(mac));
                    } else {
                        items.push(Item::Text(input.bump(1)));
                    }
                }
                '\\' => {
                    if let Some(esc) = input.parse(Escaped::parser()) {
                        items.push(Item::Escaped(esc));
                    } else {
                        items.push(Item::Text(input.bump(1)));
                    }
                }
                '$' => {
                    if input.parse(Limiter::parser()).is_some() {
                        items.push(Item::Limiter);
                    } else {
                        items.push(Item::Text(input.bump(1)));
                    }
                }

                '|' if context == Context::Table => {
                    return Some(true);
                }
                ']' if context == Context::LinkOrImg => {
                    if *open_brackets == 0 {
                        return Some(true);
                    } else {
                        items.push(Item::Text(input.bump(1)));
                        *open_brackets -= 1;
                    }
                }
                '}' if matches!(context, Context::Braces | Context::BracesFirstLine) => {
                    return Some(true);
                }

                '\n' | '\r' => {
                    if let Context::Table | Context::LinkOrImg | Context::Heading = context {
                        return Some(true);
                    }

                    if input.parse(ParseLineBreak(ind)).is_some() {
                        items.push(Item::LineBreak);

                        if let Context::Global | Context::Braces = context {
                            if let Some(u) = input.parse(Underline::parser(ind)) {
                                if let Some(Item::LineBreak) = items.last() {
                                    items.pop();
                                }
                                items.push(Item::Underline(u));
                                return Some(true);
                            }
                        }

                        if input.can_parse(ParseLineEnd)
                            && input.parse(ParseLineBreak(ind)).is_some()
                        {
                            items.push(Item::LineBreak);
                            return Some(true);
                        }

                        if self.can_parse_block(input) {
                            return Some(true);
                        }
                    } else {
                        return Some(true);
                    }
                }
                c => unreachable!("{:?} matches none of the expected characters", c),
            }
        }
        Some(false)
    }
}
