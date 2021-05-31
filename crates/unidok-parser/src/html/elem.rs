use aho_corasick::AhoCorasick;
use unidok_repr::ast::html::{ElemClose, ElemContentAst, ElemName, HtmlElemAst};
use unidok_repr::ast::segments::SegmentAst;

use crate::blocks::ParseBlock;
use crate::inlines::Segments;
use crate::parsing_mode::ParsingMode;
use crate::state::ParsingState;
use crate::utils::{ParseLineBreak, ParseLineEnd, ParseSpaces, Until};
use crate::{Context, Indents, Input, Parse};

use super::attr::ParseAttributes;
use super::elem_name::ParseElemName;

pub(crate) struct ParseHtmlElem<'a> {
    pub ind: Indents<'a>,
    pub mode: Option<ParsingMode>,
    pub ac: &'a AhoCorasick,
}

impl ParseHtmlElem<'_> {
    pub(crate) fn closing_tag(elem: ElemName) -> ParseClosingTag {
        ParseClosingTag { elem }
    }
}

impl Parse for ParseHtmlElem<'_> {
    type Output = HtmlElemAst;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('<')?;
        let name = input.parse(ParseElemName)?;
        input.parse_i(ParseSpaces);

        let attrs = input.parse(ParseAttributes { ind: self.ind })?;

        if input.parse("/>").is_some() {
            input.apply();
            Some(HtmlElemAst { name, attrs, content: None, close: ElemClose::SelfClosing })
        } else if name.is_self_closing() {
            input.parse('>')?;
            input.apply();
            Some(HtmlElemAst { name, attrs, close: ElemClose::AutoSelfClosing, content: None })
        } else {
            input.parse('>')?;

            let content = if name.contains_plaintext() {
                let mut input2 = input.start();
                let mut content = String::new();
                loop {
                    let s = input2.parse_i(Until(|c| matches!(c, '<' | '\n' | '\r')));
                    content.push_str(s.to_str(&input2.text));

                    match input2.peek_char() {
                        Some('<') => {
                            if input2.can_parse(ParseClosingTag { elem: name }) {
                                break;
                            } else {
                                input2.bump(1);
                                content.push('<');
                            }
                        }
                        None => {
                            break;
                        }
                        _ => {
                            content.push('\n');
                            if input2.parse(ParseLineBreak(self.ind)).is_none() {
                                break;
                            }
                        }
                    }
                }
                input2.apply();
                input.try_parse(ParseClosingTag { elem: name });
                ElemContentAst::Verbatim(content)
            } else if name.must_contain_blocks()
                || (name.can_contain_blocks() && input.parse(ParseLineBreak(self.ind)).is_some())
            {
                let blocks = input.parse(ParseBlock::new_multi(
                    self.mode,
                    ParsingState::new(self.ind, Context::BlockHtml(name), self.ac),
                ))?;
                input.try_parse(ParseClosingTag { elem: name });
                ElemContentAst::Blocks(blocks)
            } else {
                let nl = if input.can_parse(ParseLineEnd) {
                    input.parse(ParseLineBreak(self.ind))?;
                    true
                } else {
                    false
                };

                let mut segments = input
                    .parse(Segments::parser(
                        self.ind,
                        Context::InlineHtml(name),
                        self.mode.unwrap_or_else(ParsingMode::new_all),
                        self.ac,
                    ))?
                    .into_segments_no_underline_zero()?;
                input.try_parse(ParseClosingTag { elem: name });

                if nl && matches!(segments.last(), Some(SegmentAst::LineBreak)) {
                    segments.pop();
                }

                ElemContentAst::Inline(segments)
            };
            let content = Some(content);

            input.apply();
            Some(HtmlElemAst { name, attrs, content, close: ElemClose::Normal })
        }
    }
}

pub(crate) struct ParseClosingTag {
    elem: ElemName,
}

impl Parse for ParseClosingTag {
    type Output = ();

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse("</")?;
        let name = input.parse(ParseElemName)?;
        if name != self.elem {
            return None;
        }
        input.parse_i(ParseSpaces);
        input.parse('>')?;

        input.apply();
        Some(())
    }
}
