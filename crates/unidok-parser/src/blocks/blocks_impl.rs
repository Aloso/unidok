use aho_corasick::AhoCorasick;
use unidok_repr::ast::blocks::{BlockAst, HeadingAst, HeadingKind, ParagraphAst};

use crate::blocks::*;
use crate::inlines::{segments, Segments};
use crate::macros::ParseBlockMacro;
use crate::parsing_mode::ParsingMode;
use crate::utils::ParseLineBreak;
use crate::{Indents, Input, Parse};

use super::Context;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ParseBlock<'a> {
    context: Context,
    ind: Indents<'a>,
    mode: Option<ParsingMode>,
    no_toc: bool,
    ac: &'a AhoCorasick,
}

impl<'a> ParseBlock<'a> {
    pub(crate) fn new(
        context: Context,
        ind: Indents<'a>,
        mode: Option<ParsingMode>,
        no_toc: bool,
        ac: &'a AhoCorasick,
    ) -> Self {
        ParseBlock { context, ind, mode, no_toc, ac }
    }

    pub(crate) fn new_multi(
        context: Context,
        ind: Indents<'a>,
        ac: &'a AhoCorasick,
    ) -> ParseBlocks<'a> {
        ParseBlocks { context, ind, ac }
    }

    pub(crate) fn new_global(patterns: &'a AhoCorasick) -> ParseBlocks<'a> {
        ParseBlocks { context: Context::Global, ind: Indents::new(), ac: patterns }
    }

    pub(crate) fn get_global_patterns() -> AhoCorasick {
        AhoCorasick::new_auto_configured(segments::PATTERNS)
    }
}

impl ParseBlock<'_> {
    fn consume_empty_lines(&mut self, input: &mut Input) {
        if let Context::BlockBraces | Context::Heading | Context::BlockHtml(_) | Context::Global =
            self.context
        {
            while input.parse(ParseLineBreak(self.ind)).is_some() && !input.is_empty() {}
        }
    }
}

impl Parse for ParseBlock<'_> {
    type Output = BlockAst;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let ind = self.ind;

        let mode = self.mode.unwrap_or_else(ParsingMode::new_all);

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
            if let Some(block) = input.parse(ParseCodeBlock { ind, mode: self.mode, ac: self.ac }) {
                self.consume_empty_lines(input);
                return Some(BlockAst::CodeBlock(block));
            }
        }

        if mode.is(ParsingMode::TABLES) {
            if let Some(table) = input.parse(ParseTable { ind, ac: self.ac }) {
                self.consume_empty_lines(input);
                return Some(BlockAst::Table(table));
            }
        }

        if mode.is(ParsingMode::HEADINGS) {
            if let Some(heading) =
                input.parse(ParseHeading { ind, no_toc: self.no_toc, ac: self.ac })
            {
                self.consume_empty_lines(input);
                return Some(BlockAst::Heading(heading));
            }
        }

        if mode.is(ParsingMode::LISTS) {
            if let Some(list) = input.parse(ParseList { ind, ac: self.ac }) {
                self.consume_empty_lines(input);
                return Some(BlockAst::List(list));
            }
        }

        if mode.is(ParsingMode::QUOTES) {
            if let Some(quote) = input.parse(ParseQuote { ind, ac: self.ac }) {
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
            let parser = ParseBlockMacro::new(self.context, ind, self.mode, self.no_toc, self.ac);
            if let Some(mac) = input.parse(parser) {
                self.consume_empty_lines(input);
                return Some(BlockAst::BlockMacro(mac));
            }
        }

        let segments = input.parse(Segments::parser(ind, self.context, mode, self.ac))?;
        self.consume_empty_lines(input);

        match segments {
            Segments::Empty if self.context == Context::CodeBlock && !input.is_empty() => {
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
    context: Context,
    ind: Indents<'a>,
    ac: &'a AhoCorasick,
}

impl Parse for ParseBlocks<'_> {
    type Output = Vec<BlockAst>;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        loop {
            if input.parse(ParseLineBreak(self.ind)).is_none() {
                break;
            }
            if input.is_empty() {
                return Some(vec![]);
            }
        }

        let parser = ParseBlock {
            context: self.context,
            ind: self.ind,
            mode: None,
            no_toc: false,
            ac: self.ac,
        };

        let mut v = Vec::new();
        while let Some(node) = input.parse(parser) {
            v.push(node);
        }
        Some(v)
    }
}
