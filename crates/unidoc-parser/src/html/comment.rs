use crate::utils::Until;
use crate::{Parse, StrSlice};

#[derive(Debug, Clone, PartialEq)]
pub struct HtmlComment {
    pub text: StrSlice,
}

impl HtmlComment {
    pub(crate) fn parser() -> ParseHtmlComment {
        ParseHtmlComment
    }
}

pub(crate) struct ParseHtmlComment;

impl Parse for ParseHtmlComment {
    type Output = HtmlComment;

    fn parse(&self, input: &mut crate::input::Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse("<!--")?;
        let text = input.parse_i(Until("-->"));
        input.try_parse("-->");

        input.apply();
        Some(HtmlComment { text })
    }
}
