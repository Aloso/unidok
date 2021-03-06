use crate::ast::segments::*;
use crate::ast::AstData;
use crate::ir::segments::*;
use crate::quotes::ClosingQuotes;
use crate::IntoIR;

use super::utils::collapse_text;

impl<'a> IntoIR<'a> for SegmentAst {
    type IR = Segment<'a>;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR {
        match self {
            SegmentAst::LineBreak => Segment::LineBreak,
            SegmentAst::Text(t) => Segment::Text(t.into_ir(text, data)),
            SegmentAst::Text2(t) => Segment::Text(t),
            SegmentAst::Text3(t) => Segment::Text2(t),
            SegmentAst::Escaped(esc) => Segment::EscapedText(esc.text.into_ir(text, data)),
            SegmentAst::Substitution(s) => Segment::Text(s.into_ir(text, data)),
            SegmentAst::Limiter => Segment::Limiter,
            SegmentAst::Braces(b) => Segment::Braces(b.into_ir(text, data)),
            SegmentAst::Math(b) => Segment::Math(b.into_ir(text, data)),
            SegmentAst::Link(b) => Segment::Link(b.into_ir(text, data)),
            SegmentAst::Image(b) => Segment::Image(b.into_ir(text, data)),
            SegmentAst::InlineMacro(b) => b.into_ir(text, data),
            SegmentAst::InlineHtml(h) => Segment::InlineHtml(h.into_ir(text, data)),
            SegmentAst::HtmlEntity(e) => Segment::HtmlEntity(e),
            SegmentAst::Format(b) => Segment::Format(b.into_ir(text, data)),
            SegmentAst::Code(b) => Segment::Code(b.into_ir(text, data)),
        }
    }
}

impl Default for SegmentAst {
    fn default() -> Self {
        SegmentAst::Text2("")
    }
}

impl<'a> IntoIR<'a> for BracesAst {
    type IR = Braces<'a>;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR {
        Braces { macros: vec![], segments: collapse_text(self.segments).into_ir(text, data) }
    }
}

impl<'a> IntoIR<'a> for MathAst {
    type IR = Math<'a>;

    fn into_ir(self, _: &str, data: &mut AstData) -> Self::IR {
        data.contains_math = true;
        Math { macros: vec![], text: self.text }
    }
}

impl<'a> IntoIR<'a> for LinkAst {
    type IR = Link<'a>;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR {
        match self.target {
            LinkTarget::Url { href, title } => {
                let segments = self.text.unwrap_or_else(|| vec![SegmentAst::Text3(href.clone())]);
                Link {
                    macros: vec![],
                    href: Some(href),
                    text: collapse_text(segments).into_ir(text, data),
                    title,
                    footnote: None,
                }
            }
            LinkTarget::Reference(r) => {
                let reference = r.to_str(text);
                match data.link_ref_defs.get(reference) {
                    Some(lrd) => {
                        let href = lrd.url.to_str(text);
                        let segments = self.text.unwrap_or_else(|| vec![SegmentAst::Text(r)]);

                        let title = lrd.title.clone();
                        Link {
                            macros: vec![],
                            href: Some(href.to_string()),
                            text: collapse_text(segments).into_ir(text, data),
                            title,
                            footnote: None,
                        }
                    }
                    None => {
                        let text = if let Some(mut segments) = self.text {
                            let len = segments.len();
                            segments.push(SegmentAst::Text2("["));
                            segments.rotate_left(len);
                            segments.push(SegmentAst::Text3(format!("][{}]", reference)));
                            collapse_text(segments).into_ir(text, data)
                        } else {
                            vec![Segment::Text2(format!("[{}]", reference))]
                        };
                        Link { macros: vec![], href: None, text, title: None, footnote: None }
                    }
                }
            }
            LinkTarget::Footnote => {
                data.footnotes.push(LinkAst { text: self.text, target: self.target });
                let n = data.next_footnote;
                data.next_footnote += 1;

                Link {
                    macros: vec![],
                    href: Some(format!("#{}", n)),
                    text: vec![Segment::Text2(format!("[{}]", n))],
                    title: None,
                    footnote: Some(n),
                }
            }
        }
    }
}

impl<'a> IntoIR<'a> for ImageAst {
    type IR = Image<'a>;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR {
        match self.target {
            LinkTarget::Url { href, title } => {
                let segments = self.alt.unwrap_or_else(|| vec![SegmentAst::Text3(href.clone())]);
                Image {
                    macros: vec![],
                    href: Some(href),
                    alt: collapse_text(segments).into_ir(text, data),
                    title,
                }
            }
            LinkTarget::Reference(r) => {
                let reference = r.to_str(text);
                match data.link_ref_defs.get(reference) {
                    Some(lrd) => {
                        let href = lrd.url.to_str(text);
                        let segments = self.alt.unwrap_or_else(|| vec![SegmentAst::Text(r)]);

                        let title = lrd.title.clone();
                        Image {
                            macros: vec![],
                            href: Some(href.to_string()),
                            alt: collapse_text(segments).into_ir(text, data),
                            title,
                        }
                    }
                    None => {
                        let alt = if let Some(mut segments) = self.alt {
                            let len = segments.len();
                            segments.push(SegmentAst::Text2("!["));
                            segments.rotate_left(len);
                            segments.push(SegmentAst::Text3(format!("][{}]", reference)));
                            collapse_text(segments).into_ir(text, data)
                        } else {
                            vec![Segment::Text2(format!("![{}]", reference))]
                        };
                        Image { macros: vec![], href: None, alt, title: None }
                    }
                }
            }
            LinkTarget::Footnote => panic!("Images can't refer to a footnote"),
        }
    }
}

impl<'a> IntoIR<'a> for InlineFormatAst {
    type IR = InlineFormat<'a>;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR {
        InlineFormat {
            formatting: self.formatting,
            segments: collapse_text(self.segments).into_ir(text, data),
        }
    }
}

impl<'a> IntoIR<'a> for CodeAst {
    type IR = Code<'a>;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR {
        Code { macros: vec![], segments: collapse_text(self.segments).into_ir(text, data) }
    }
}

impl<'a> IntoIR<'a> for Substitution {
    type IR = &'a str;

    fn into_ir(self, _: &str, data: &mut AstData) -> Self::IR {
        match self {
            Substitution::Text(text) => text,
            Substitution::OpenDoubleQuote => data.config.quote_style.double_start.to_str(),
            Substitution::OpenSingleQuote => data.config.quote_style.single_start.to_str(),
            Substitution::CloseDoubleQuote => data.config.quote_style.double_end.to_str(),
            Substitution::CloseSingleQuote => data.config.quote_style.single_end.to_str(),
            Substitution::Apostrophe => {
                if data.config.quote_style.is_english() {
                    ClosingQuotes::EnglishSingle.to_str()
                } else {
                    "'"
                }
            }
        }
    }
}
