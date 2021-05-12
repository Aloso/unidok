use crate::ast::segments::*;
use crate::ast::AstState;
use crate::ir::segments::*;
use crate::IntoIR;

use super::utils::collapse_text;

impl<'a> IntoIR<'a> for Segment {
    type IR = SegmentIr<'a>;

    fn into_ir(self, text: &'a str, state: &AstState) -> Self::IR {
        match self {
            Segment::LineBreak => SegmentIr::LineBreak,
            Segment::Text(t) => SegmentIr::Text(t.into_ir(text, state)),
            Segment::Text2(t) => SegmentIr::Text(t),
            Segment::Text3(t) => SegmentIr::Text2(t),
            Segment::Escaped(esc) => SegmentIr::EscapedText(esc.text.into_ir(text, state)),
            Segment::Limiter => SegmentIr::Limiter,
            Segment::Braces(b) => SegmentIr::Braces(b.into_ir(text, state)),
            Segment::Math(b) => SegmentIr::Math(b.into_ir(text, state)),
            Segment::Link(b) => SegmentIr::Link(b.into_ir(text, state)),
            Segment::Image(b) => SegmentIr::Image(b.into_ir(text, state)),
            Segment::InlineMacro(b) => b.into_ir(text, state),
            Segment::InlineHtml(h) => SegmentIr::InlineHtml(h.into_ir(text, state)),
            Segment::HtmlEntity(e) => SegmentIr::HtmlEntity(e),
            Segment::Format(b) => SegmentIr::Format(b.into_ir(text, state)),
            Segment::Code(b) => SegmentIr::Code(b.into_ir(text, state)),
        }
    }
}

impl Default for Segment {
    fn default() -> Self {
        Segment::Text2("")
    }
}

impl<'a> IntoIR<'a> for Braces {
    type IR = BracesIr<'a>;

    fn into_ir(self, text: &'a str, state: &AstState) -> Self::IR {
        BracesIr {
            annotations: vec![],
            segments: collapse_text(self.segments).into_ir(text, state),
        }
    }
}

impl<'a> IntoIR<'a> for Math {
    type IR = MathIr<'a>;

    fn into_ir(self, _: &str, _: &AstState) -> Self::IR {
        MathIr { annotations: vec![], text: self.text }
    }
}

impl<'a> IntoIR<'a> for Link {
    type IR = LinkIr<'a>;

    fn into_ir(self, text: &'a str, state: &AstState) -> Self::IR {
        match self.target {
            LinkTarget::Url { href, title } => {
                let segments = self.text.unwrap_or_else(|| vec![Segment::Text3(href.clone())]);
                LinkIr {
                    annotations: vec![],
                    href: Some(href),
                    text: collapse_text(segments).into_ir(text, state),
                    title,
                }
            }
            LinkTarget::Reference(r) => {
                let reference = r.to_str(text);
                match state.link_ref_defs.get(reference) {
                    Some(lrd) => {
                        let href = lrd.url.to_str(text);
                        let segments = self.text.unwrap_or_else(|| vec![Segment::Text(r)]);

                        LinkIr {
                            annotations: vec![],
                            href: Some(href.to_string()),
                            text: collapse_text(segments).into_ir(text, state),
                            title: lrd.title.clone(),
                        }
                    }
                    None => {
                        let text = if let Some(mut segments) = self.text {
                            let len = segments.len();
                            segments.push(Segment::Text2("["));
                            segments.rotate_left(len);
                            segments.push(Segment::Text3(format!("][{}]", reference)));
                            collapse_text(segments).into_ir(text, state)
                        } else {
                            vec![SegmentIr::Text2(format!("[{}]", reference))]
                        };
                        LinkIr { annotations: vec![], href: None, text, title: None }
                    }
                }
            }
        }
    }
}

impl<'a> IntoIR<'a> for Image {
    type IR = ImageIr<'a>;

    fn into_ir(self, text: &'a str, state: &AstState) -> Self::IR {
        match self.target {
            LinkTarget::Url { href, title } => {
                let segments = self.alt.unwrap_or_else(|| vec![Segment::Text3(href.clone())]);
                ImageIr {
                    annotations: vec![],
                    href: Some(href),
                    alt: collapse_text(segments).into_ir(text, state),
                    title,
                }
            }
            LinkTarget::Reference(r) => {
                let reference = r.to_str(text);
                match state.link_ref_defs.get(reference) {
                    Some(lrd) => {
                        let href = lrd.url.to_str(text);
                        let segments = self.alt.unwrap_or_else(|| vec![Segment::Text(r)]);

                        ImageIr {
                            annotations: vec![],
                            href: Some(href.to_string()),
                            alt: collapse_text(segments).into_ir(text, state),
                            title: lrd.title.clone(),
                        }
                    }
                    None => {
                        let alt = if let Some(mut segments) = self.alt {
                            let len = segments.len();
                            segments.push(Segment::Text2("!["));
                            segments.rotate_left(len);
                            segments.push(Segment::Text3(format!("][{}]", reference)));
                            collapse_text(segments).into_ir(text, state)
                        } else {
                            vec![SegmentIr::Text2(format!("![{}]", reference))]
                        };
                        ImageIr { annotations: vec![], href: None, alt, title: None }
                    }
                }
            }
        }
    }
}

impl<'a> IntoIR<'a> for InlineFormat {
    type IR = InlineFormatIr<'a>;

    fn into_ir(self, text: &'a str, state: &AstState) -> Self::IR {
        InlineFormatIr {
            formatting: self.formatting,
            segments: collapse_text(self.segments).into_ir(text, state),
        }
    }
}

impl<'a> IntoIR<'a> for Code {
    type IR = CodeIr<'a>;

    fn into_ir(self, text: &'a str, state: &AstState) -> Self::IR {
        CodeIr { annotations: vec![], segments: collapse_text(self.segments).into_ir(text, state) }
    }
}
