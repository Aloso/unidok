use std::convert::TryFrom;

use super::format::{is_in_word, is_not_flanking, FlankType, Flanking, FormatDelim};
use super::*;
use crate::blocks::macros::ParseClosingBrace;
use crate::blocks::*;
use crate::html::{HtmlElem, HtmlNode};
use crate::input::Input;
use crate::parsing_mode::ParsingMode;
use crate::utils::{Indents, ParseLineBreak, While};
use crate::{Parse, StrSlice};

#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    LineBreak(LineBreak),
    Text(StrSlice),
    Text2(&'static str),
    Escaped(Escaped),
    Limiter(Limiter),
    Braces(Braces),
    Math(Math),
    Link(Link),
    Image(Image),
    InlineMacro(InlineMacro),
    InlineHtml(HtmlNode),
    Format(InlineFormat),
    Code(Code),
}

impl Segment {
    // pub fn is_closing_tag_for(&self, name: ElemName) -> bool {
    //     matches!(*self, Segment::InlineHtml(HtmlNode::ClosingTag(n)) if n ==
    // name) }

    pub fn strip_space_start(&mut self, input: &Input) -> bool {
        match self {
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

    pub fn strip_space_end(&mut self, input: &Input) -> bool {
        match self {
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
}

impl Default for Segment {
    fn default() -> Self {
        Segment::Text2("")
    }
}

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

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        use Context::*;

        let (items, underline) = self.lex_items(input)?;
        if items.is_empty() {
            return Some(Segments::Empty);
        }

        let stack = parse_paragraph_items(items);
        let mut segments = stack_to_segments(stack);

        if let BlockBraces | Heading | Global = self.context {
            while input.parse(ParseLineBreak(self.ind)).is_some() && !input.is_empty() {
                segments.push(Segment::LineBreak(LineBreak));
            }
        }

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
                        stack.push(StackItem::FormatDelim {
                            delim,
                            flanking: Flanking::Left,
                            count,
                        });
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
            | ('%' | '[' | ']' | '!' | '@' | '\\' | '$' | '<' | '\n' | '\r')
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
    use Context::*;

    match context {
        Global | Heading | Html(_) => s.find(find_special),
        BlockBraces | Braces => s.find(find_special_in_braces),
        Table => s.find(find_special_in_table),
        LinkOrImg => s.find(find_special_in_link_or_img),
        Code(_) => s.find(find_special_in_code),
    }
}

impl ParseSegments<'_> {
    fn lex_items(&self, input: &mut Input) -> Option<(Vec<Item>, Option<Underline>)> {
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
            let cs = input.parse_i(While(sym));
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
                        let backticks = input.parse_i(While('`')).len();
                        if backticks == len as usize {
                            return Some(true);
                        } else {
                            items.push(Item::Text(input.parse_i(While('`'))));
                        }
                    } else if let Some(code) =
                        input.parse(Code::parser(ind, ParsingMode::new_nothing()))
                    {
                        items.push(Item::Code(code));
                    } else {
                        items.push(Item::Text(input.parse_i(While('`'))));
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
                    if let Some(mac) = input.parse(InlineMacro::parser(ind, self.mode)) {
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
                '<' => {
                    if let Some(html) = input.parse(HtmlNode::parser(ind)) {
                        items.push(Item::Html(html));
                    } else if let Context::Html(elem) = context {
                        if input.can_parse(HtmlElem::closing_tag_parser(elem)) {
                            return Some(true);
                        } else {
                            items.push(Item::Text(input.bump(1)));
                        }
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
                ']' => {
                    if *open_brackets > 0 {
                        *open_brackets -= 1;
                    }
                    items.push(Item::Text(input.bump(1)));
                }
                '}' if context == Context::Braces => {
                    return Some(true);
                }
                '}' if context == Context::BlockBraces => {
                    if matches!(items.last(), Some(Item::LineBreak) | None)
                        && input.can_parse(ParseClosingBrace(ind))
                    {
                        return Some(true);
                    } else {
                        items.push(Item::Text(input.bump(1)));
                    }
                }

                '\n' | '\r' => {
                    if let Context::Table | Context::LinkOrImg | Context::Heading = context {
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

                        if input.can_parse(ParseLineBreak(ind)) || self.can_parse_block(input) {
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

    fn can_parse_block(&self, input: &mut Input) -> bool {
        let ind = self.ind;
        input.can_parse(CodeBlock::parser(ind))
            || input.can_parse(Comment::parser(ind))
            || input.can_parse(Heading::parser(ind))
            || input.can_parse(Table::parser(ind))
            || input.can_parse(List::parser(ind))
            || input.can_parse(ThematicBreak::parser(ind))
            || input.can_parse(Quote::parser(ind))
    }
}
