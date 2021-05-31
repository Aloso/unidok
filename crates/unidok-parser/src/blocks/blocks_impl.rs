use unidok_repr::ast::blocks::{BlockAst, HeadingAst, HeadingKind, ParagraphAst};

use crate::blocks::*;
use crate::inlines::Segments;
use crate::macros::ParseBlockMacro;
use crate::parsing_mode::ParsingMode;
use crate::state::ParsingState;
use crate::utils::ParseLineBreak;
use crate::{Context, Input, Parse};

#[derive(Debug, Clone, Copy)]
pub(crate) struct ParseBlock<'a> {
    mode: Option<ParsingMode>,
    state: ParsingState<'a>,
}

impl<'a> ParseBlock<'a> {
    pub(crate) fn new(mode: Option<ParsingMode>, state: ParsingState<'a>) -> Self {
        ParseBlock { mode, state }
    }

    pub(crate) fn new_multi(mode: Option<ParsingMode>, state: ParsingState<'a>) -> ParseBlocks<'a> {
        ParseBlocks { mode, state }
    }
}

impl ParseBlock<'_> {
    fn consume_empty_lines(&mut self, input: &mut Input) {
        let context = self.state.context();
        if let Context::BlockBraces | Context::Heading | Context::BlockHtml(_) | Context::Global =
            context
        {
            let ind = self.state.ind();
            while input.parse(ParseLineBreak(ind)).is_some() && !input.is_empty() {}
        }
    }
}

impl Parse for ParseBlock<'_> {
    type Output = BlockAst;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let ind = self.state.ind();
        let mode = self.mode.unwrap_or_else(ParsingMode::new_all);
        let ac = self.state.special_chars();
        let context = self.state.context();

        if mode.is(ParsingMode::COMMENTS) {
            if let Some(comment) = input.parse(ParseComment) {
                self.consume_empty_lines(input);
                return Some(BlockAst::Comment(comment));
            }
        }

        if mode.is(ParsingMode::THEMATIC_BREAKS) {
            if let Some(tb) = input.parse(ParseThematicBreak { ind }) {
                self.consume_empty_lines(input);
                return Some(BlockAst::ThematicBreak(tb));
            }
        }

        if mode.is(ParsingMode::CODE_BLOCKS) {
            if let Some(block) = input.parse(ParseCodeBlock { ind, mode: self.mode, ac }) {
                self.consume_empty_lines(input);
                return Some(BlockAst::CodeBlock(block));
            }
        }

        if mode.is(ParsingMode::TABLES) {
            if let Some(table) = input.parse(ParseTable { ind, ac }) {
                self.consume_empty_lines(input);
                return Some(BlockAst::Table(table));
            }
        }

        if mode.is(ParsingMode::HEADINGS) {
            if let Some(heading) = input.parse(ParseHeading { ind, ac }) {
                self.consume_empty_lines(input);
                return Some(BlockAst::Heading(heading));
            }
        }

        if mode.is(ParsingMode::LISTS) {
            if let Some(list) = input.parse(ParseList { ind, mode: self.mode, ac }) {
                self.consume_empty_lines(input);
                return Some(BlockAst::List(list));
            }
        }

        if mode.is(ParsingMode::QUOTES) {
            if let Some(quote) = input.parse(ParseQuote { ind, mode: self.mode, ac }) {
                self.consume_empty_lines(input);
                return Some(BlockAst::Quote(quote));
            }
        }

        if mode.is(ParsingMode::LINKS_IMAGES) {
            if let Some(lrd) = input.parse(ParseLinkRefDef { ind }) {
                self.consume_empty_lines(input);
                return Some(BlockAst::LinkRefDef(lrd));
            }
        }

        if mode.is(ParsingMode::MACROS) {
            let parser = ParseBlockMacro::new(self.mode, self.state);
            if let Some(mac) = input.parse(parser) {
                self.consume_empty_lines(input);
                return Some(BlockAst::BlockMacro(mac));
            }
        }

        let segments = input.parse(Segments::parser(ind, context, mode, ac))?;
        self.consume_empty_lines(input);

        match segments {
            Segments::Empty if context == Context::CodeBlock && !input.is_empty() => {
                Some(BlockAst::Paragraph(ParagraphAst { segments: vec![] }))
            }
            Segments::Empty => None,
            Segments::Some { segments, underline: None } => {
                Some(BlockAst::Paragraph(ParagraphAst { segments }))
            }
            Segments::Some { segments, underline: Some(u) } if mode.is(ParsingMode::HEADINGS) => {
                Some(BlockAst::Heading(HeadingAst {
                    level: u.level(),
                    kind: HeadingKind::Setext,
                    segments,
                }))
            }
            _ => panic!("Parsed an underlined heading where no headings are allowed"),
        }
    }

    fn can_parse(&mut self, _: &mut Input) -> bool {
        true
    }
}

#[derive(Debug)]
pub(crate) struct ParseBlocks<'a> {
    mode: Option<ParsingMode>,
    state: ParsingState<'a>,
}

impl Parse for ParseBlocks<'_> {
    type Output = Vec<BlockAst>;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        loop {
            if input.parse(ParseLineBreak(self.state.ind())).is_none() {
                break;
            }
            if input.is_empty() {
                return Some(vec![]);
            }
        }

        let parser = ParseBlock { mode: self.mode, state: self.state };

        let mut v = Vec::new();
        while let Some(node) = input.parse(parser) {
            v.push(node);
        }
        Some(v)
    }
}
