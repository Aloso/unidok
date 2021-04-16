use std::cmp::Ordering;

use super::*;
use crate::containers::*;
use crate::inlines::*;
use crate::str::StrSlice;
use crate::utils::{Indents, ParseLineBreak, ParseLineEnd, WhileChar};
use crate::{Context, Input, Parse};

#[derive(Debug, Clone, PartialEq)]
pub struct Paragraph {
    pub segments: Vec<Segment>,
}

pub(crate) struct ParseParagraph<'a> {
    ind: Indents<'a>,
    context: Context,
}

impl Paragraph {
    pub(crate) fn parser(ind: Indents<'_>, context: Context) -> ParseParagraph<'_> {
        ParseParagraph { ind, context }
    }
}

impl Parse for ParseParagraph<'_> {
    type Output = Paragraph;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let stack = generate_stack(input, self.context, self.ind)?;
        dbg!(&stack);

        let mut segments = Vec::new();
        parse_paragraph_part(stack, 0, &mut segments);

        Some(Paragraph { segments })
    }
}

fn parse_paragraph_part(_stack: Vec<Item>, _start: usize, _segments: &mut Vec<Segment>) {
    todo!()
}

#[derive(Debug, Clone)]
enum Item {
    Text(StrSlice),
    FormatDelim {
        /// the type of delimiter
        delim: FormatDelim,
        /// whether the delimiter is left-flanking, right-flanking, or both
        flanking: Flanking,
        /// true if delimiter run is multiple of 3
        mult_of_three: bool,
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

impl Default for Item {
    fn default() -> Self {
        Item::Text(StrSlice::default())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Flanking {
    /// ` **Hello`
    Left,
    /// `Hello** `
    Right,
    /// `Hello**world`
    Both,
}

impl Flanking {
    fn from(left: char, right: char) -> Flanking {
        fn binding_power(c: char) -> u8 {
            if c == '$' {
                0
            } else if c.is_whitespace() {
                1
            } else if !c.is_alphanumeric() {
                2
            } else {
                3
            }
        }

        let left_bp = binding_power(left);
        let right_bp = binding_power(right);

        match left_bp.cmp(&right_bp) {
            Ordering::Less => Flanking::Left,
            Ordering::Equal => Flanking::Both,
            Ordering::Greater => Flanking::Right,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum FormatDelim {
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

fn can_parse_block(input: &mut Input, ind: Indents) -> bool {
    input.can_parse(CodeBlock::parser(ind))
        || input.can_parse(Comment::parser(ind))
        || input.can_parse(Heading::parser(ind))
        || input.can_parse(ThematicBreak::parser(ind))
        || input.can_parse(Table::parser(ind))
        || input.can_parse(List::parser(ind))
        || input.can_parse(Quote::parser(ind))
}

macro_rules! push_fmt {
    ($input:expr, $stack:expr, $sym:literal, $id:ident) => {{
        let left = $input.prev_char();
        let cs = $input.parse(WhileChar($sym))?;
        let right = $input.peek_char();

        let flanking = Flanking::from(left.unwrap_or('$'), right.unwrap_or('$'));
        if $sym == '_' && flanking == Flanking::Both {
            $stack.push(Item::Text(cs));
        } else {
            let delim = FormatDelim::$id;
            let mult_of_three = cs.len() % 3 == 0;
            for _ in 0..cs.len() {
                $stack.push(Item::FormatDelim { delim, flanking, mult_of_three });
            }
        }
    }};
}

fn find_special(c: char) -> bool {
    matches!(c, '*' | '_' | '~' | '`' | '^' | '#')
        || matches!(c, '%' | '[' | '!' | '@' | '\\' | '$' | '\n' | '\r')
}

fn find_special_in_table(c: char) -> bool {
    matches!(c, '*' | '_' | '~' | '`' | '^' | '#')
        || matches!(c, '|')
        || matches!(c, '%' | '[' | '!' | '@' | '\\' | '$' | '\n' | '\r')
}

fn find_special_in_braces(c: char) -> bool {
    matches!(c, '*' | '_' | '~' | '`' | '^' | '#')
        || matches!(c, '}')
        || matches!(c, '%' | '[' | '!' | '@' | '\\' | '$' | '\n' | '\r')
}

fn find_special_in_link_or_img(c: char) -> bool {
    matches!(c, '*' | '_' | '~' | '`' | '^' | '#')
        || matches!(c, ']')
        || matches!(c, '%' | '[' | '!' | '@' | '\\' | '$' | '\n' | '\r')
}

fn generate_stack(input: &mut Input, context: Context, ind: Indents<'_>) -> Option<Vec<Item>> {
    let mut stack = Vec::new();

    while !input.is_empty() {
        let special_idx = input.rest().find(match context {
            Context::Global | Context::Heading => find_special,
            Context::Braces => find_special_in_braces,
            Context::Table => find_special_in_table,
            Context::LinkOrImg => find_special_in_link_or_img,
        });
        match special_idx {
            Some(0) => match input.peek_char().unwrap() {
                '*' => push_fmt!(input, stack, '*', Star),
                '_' => push_fmt!(input, stack, '_', Underscore),
                '~' => push_fmt!(input, stack, '~', Tilde),
                '^' => push_fmt!(input, stack, '^', Caret),
                '#' => push_fmt!(input, stack, '#', NumberSign),
                '`' => {
                    if let Some(code) = input.parse(Code::parser()) {
                        stack.push(Item::Code(code));
                    } else {
                        stack.push(Item::Text(input.bump(1)));
                    }
                }
                '%' => {
                    if let Some(math) = input.parse(Math::parser(ind)) {
                        stack.push(Item::Math(math));
                    } else {
                        stack.push(Item::Text(input.bump(1)));
                    }
                }
                '[' => {
                    if let Some(link) = input.parse(Link::parser(ind)) {
                        stack.push(Item::Link(link));
                    } else {
                        stack.push(Item::Text(input.bump(1)));
                    }
                }
                '!' => {
                    if let Some(img) = input.parse(Image::parser(ind)) {
                        stack.push(Item::Image(img));
                    } else {
                        stack.push(Item::Text(input.bump(1)));
                    }
                }
                '@' => {
                    if let Some(mac) = input.parse(Macro::parser(ind)) {
                        stack.push(Item::Macro(mac));
                    } else {
                        stack.push(Item::Text(input.bump(1)));
                    }
                }
                '\\' => {
                    if let Some(esc) = input.parse(Escaped::parser()) {
                        stack.push(Item::Escaped(esc));
                    } else {
                        stack.push(Item::Text(input.bump(1)));
                    }
                }
                '$' => {
                    if input.parse(Limiter::parser()).is_some() {
                        stack.push(Item::Limiter);
                    } else {
                        stack.push(Item::Text(input.bump(1)));
                    }
                }

                '|' if context == Context::Table => {
                    break;
                }
                ']' if context == Context::LinkOrImg => {
                    break;
                }
                '}' if context == Context::Braces => {
                    break;
                }

                '\n' | '\r' => {
                    if let Context::Table | Context::LinkOrImg | Context::Heading = context {
                        break;
                    }

                    if input.parse(ParseLineBreak(ind)).is_some() {
                        stack.push(Item::LineBreak);
                        if input.can_parse(ParseLineEnd) {
                            if input.parse(ParseLineBreak(ind)).is_some() {
                                stack.push(Item::LineBreak);
                                break;
                            } else if can_parse_block(input, ind) {
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                }
                c => unreachable!("{:?} matches none of the expected characters", c),
            },
            Some(bytes) => {
                stack.push(Item::Text(input.bump(bytes)));
            }
            None => {
                stack.push(Item::Text(input.bump(input.len())));
            }
        }
    }

    Some(stack)
}
