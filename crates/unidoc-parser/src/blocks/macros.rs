use crate::inlines::macros::{ParseMacroArgs, ParseMacroName};
use crate::utils::{Indents, ParseLineBreak, ParseLineEnd, ParseSpaces};
use crate::{Block, Context, Input, Parse, StrSlice};

/// A block macro
///
/// ### Example
///
/// ````md
/// @SOME_MACRO(args)
/// The macro applies to this paragraph
/// ````
#[derive(Debug, Clone, PartialEq)]
pub enum BlockMacro {
    AttrMacro { name: StrSlice, args: Option<StrSlice>, content: Box<Block> },
    BraceMacro { name: StrSlice, args: Option<StrSlice>, content: Vec<Block> },
}

impl BlockMacro {
    pub fn parser(context: Context, ind: Indents<'_>) -> ParseBlockMacro<'_> {
        ParseBlockMacro { context, ind }
    }
}

pub struct ParseBlockMacro<'a> {
    context: Context,
    ind: Indents<'a>,
}

impl Parse for ParseBlockMacro<'_> {
    type Output = BlockMacro;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        let ind = self.ind.push_indent(input.parse_i(ParseSpaces));

        input.parse('@')?;
        let name = input.parse(ParseMacroName)?;
        let args = input.parse(ParseMacroArgs);

        let mac = if input.parse(ParseLineBreak(ind)).is_some() {
            let content = Box::new(input.parse(Block::parser(self.context, ind))?);

            BlockMacro::AttrMacro { name, args, content }
        } else if input.parse(ParseOpeningBrace(self.ind)).is_some() {
            let content = input.parse(Block::multi_parser(Context::BlockBraces, ind))?;
            input.try_parse(ParseClosingBrace(self.ind));

            BlockMacro::BraceMacro { name, args, content }
        } else {
            return None;
        };

        input.apply();
        Some(mac)
    }

    fn can_parse(&self, input: &mut Input) -> bool {
        let mut input = input.start();

        let ind = self.ind.push_indent(input.parse_i(ParseSpaces));

        if input.parse('@').is_none() || input.parse(ParseMacroName).is_none() {
            return false;
        }
        input.try_parse(ParseMacroArgs);

        if input.parse(ParseLineBreak(ind)).is_some() {
            input.can_parse(Block::parser(Context::Global, ind))
        } else if input.parse(ParseOpeningBrace(self.ind)).is_some() {
            input.can_parse(Block::multi_parser(Context::BlockBraces, ind))
        } else {
            false
        }
    }
}

pub(crate) struct ParseOpeningBrace<'a>(pub(crate) Indents<'a>);

impl Parse for ParseOpeningBrace<'_> {
    type Output = ();

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('{')?;
        input.parse_i(ParseSpaces);
        input.parse(ParseLineEnd)?;
        input.try_parse(ParseLineBreak(self.0));

        input.apply();
        Some(())
    }
}

pub(crate) struct ParseClosingBrace<'a>(pub(crate) Indents<'a>);

impl Parse for ParseClosingBrace<'_> {
    type Output = ();

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse_i(ParseSpaces);
        input.parse('}')?;
        input.parse_i(ParseSpaces);
        input.parse(ParseLineEnd)?;
        input.try_parse(ParseLineBreak(self.0));

        input.apply();
        Some(())
    }
}
