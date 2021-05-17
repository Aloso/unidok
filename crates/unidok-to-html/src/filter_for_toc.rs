use unidok_repr::ir::html::HtmlNodeIr;
use unidok_repr::ir::macros::MacroIr;
use unidok_repr::ir::segments::{BracesIr, CodeIr, ImageIr, InlineFormatIr, LinkIr, SegmentIr};

pub fn filter_for_toc<'a>(s: &[SegmentIr<'a>]) -> Vec<SegmentIr<'a>> {
    s.iter()
        .filter_map(|s| match s {
            SegmentIr::Braces(BracesIr { macros, segments }) => {
                if is_allowed_toc(macros) {
                    Some(SegmentIr::Braces(BracesIr {
                        macros: macros.clone(),
                        segments: filter_for_toc(&segments),
                    }))
                } else {
                    None
                }
            }
            SegmentIr::Link(LinkIr { macros, href, text, title, .. }) => {
                if is_allowed_toc(macros) {
                    Some(SegmentIr::Link(LinkIr {
                        macros: macros.clone(),
                        href: href.clone(),
                        text: filter_for_toc(text),
                        title: title.clone(),
                        name: None,
                        is_superscript: false,
                    }))
                } else {
                    None
                }
            }
            SegmentIr::Image(ImageIr { macros, href, alt, title }) => {
                if is_allowed_toc(macros) {
                    Some(SegmentIr::Image(ImageIr {
                        macros: macros.clone(),
                        href: href.clone(),
                        alt: filter_for_toc(alt),
                        title: title.clone(),
                    }))
                } else {
                    None
                }
            }
            SegmentIr::Math(m) => {
                if is_allowed_toc(&m.macros) {
                    Some(SegmentIr::Math(m.clone()))
                } else {
                    None
                }
            }
            SegmentIr::InlineHtml(HtmlNodeIr::Element(e)) => {
                if is_allowed_toc(&e.macros) {
                    Some(SegmentIr::InlineHtml(HtmlNodeIr::Element(e.clone())))
                } else {
                    None
                }
            }
            SegmentIr::Code(c) => {
                if is_allowed_toc(&c.macros) {
                    Some(SegmentIr::Code(CodeIr {
                        macros: c.macros.clone(),
                        segments: filter_for_toc(&c.segments),
                    }))
                } else {
                    None
                }
            }

            SegmentIr::Format(f) => Some(SegmentIr::Format(InlineFormatIr {
                formatting: f.formatting,
                segments: filter_for_toc(&f.segments),
            })),

            s => Some(s.clone()),
        })
        .collect()
}

fn is_allowed_toc(m: &[MacroIr<'_>]) -> bool {
    m.iter().all(|m| !matches!(m, MacroIr::NoToc))
}
