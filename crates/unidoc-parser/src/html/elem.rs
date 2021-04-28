use crate::blocks::{Block, Context, Paragraph};
use crate::inlines::Segment;
use crate::input::Input;
use crate::parse::Parse;
use crate::utils::{Indents, ParseLineBreak, ParseSpaces, Until};
use crate::StrSlice;

use super::{ElemName, HtmlAttr};

#[derive(Debug, Clone, PartialEq)]
pub struct HtmlElem {
    pub name: ElemName,
    pub attrs: Vec<HtmlAttr>,
    pub content: Option<ElemContent>,
    pub close: ElemClose,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElemContent {
    Blocks(Vec<Block>),
    Inline(Vec<Segment>),
    Verbatim(StrSlice),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElemClose {
    /// `<br>`
    AutoSelfClosing,
    /// `<br />`
    SelfClosing,
    /// ```html
    /// <ul>
    ///     <li>Element</li>
    /// </ul>
    /// ```
    Normal,
    /// ```html
    /// <ul>
    ///     <li>Element
    /// </ul>
    /// ```
    AutoClosing,
}

impl HtmlElem {
    pub(crate) fn parser(ind: Indents<'_>) -> ParseElement<'_> {
        ParseElement { ind }
    }
    pub(crate) fn closing_tag_parser(elem: ElemName) -> ParseClosingTag {
        ParseClosingTag { elem }
    }
}

pub(crate) struct ParseElement<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseElement<'_> {
    type Output = HtmlElem;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('<')?;
        let name = input.parse(ElemName::parser())?;
        input.parse_i(ParseSpaces);

        let attrs = input.parse(HtmlAttr::multi_parser())?;

        if input.parse("/>").is_some() {
            input.apply();
            Some(HtmlElem { name, attrs, content: None, close: ElemClose::SelfClosing })
        } else if name.is_self_closing() {
            input.parse('>')?;
            input.apply();
            Some(HtmlElem { name, attrs, close: ElemClose::AutoSelfClosing, content: None })
        } else {
            input.parse('>')?;

            let context = Context::Html(name);

            let content = if name.must_contain_blocks()
                || (name.can_contain_blocks() && input.parse(ParseLineBreak(self.ind)).is_some())
            {
                let blocks = input.parse(Block::multi_parser(context, self.ind))?;
                ElemContent::Blocks(blocks)
            } else if name.contains_plaintext() {
                let mut input2 = input.start();
                loop {
                    input2.parse_i(Until('<'));
                    if input2.can_parse(HtmlElem::closing_tag_parser(name)) {
                        break;
                    } else {
                        input2.bump(1);
                    }
                }
                let content = input2.apply();
                input.try_parse(HtmlElem::closing_tag_parser(name));
                ElemContent::Verbatim(content)
            } else {
                let mut segments = input.parse(Paragraph::parser(self.ind, context))?.segments;
                if segments.last().map(|segment| segment.is_closing_tag_for(name)) == Some(true) {
                    segments.pop();
                }
                ElemContent::Inline(segments)
            };
            let content = Some(content);

            input.apply();
            Some(HtmlElem { name, attrs, content, close: ElemClose::Normal })
        }
    }
}

pub(crate) struct ParseClosingTag {
    elem: ElemName,
}

impl Parse for ParseClosingTag {
    type Output = ();

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse("</")?;
        let name = input.parse(ElemName::parser())?;
        if name != self.elem {
            return None;
        }
        input.parse_i(ParseSpaces);
        input.parse('>')?;

        input.apply();
        Some(())
    }
}
