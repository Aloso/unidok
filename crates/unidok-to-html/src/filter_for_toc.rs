use unidok_repr::ir::html::HtmlNode;
use unidok_repr::ir::macros::Macro;
use unidok_repr::ir::segments::{Braces, Code, Image, InlineFormat, Link, Segment};

pub fn filter_for_toc<'a>(s: &[Segment<'a>]) -> Vec<Segment<'a>> {
    s.iter()
        .filter_map(|s| match s {
            Segment::Braces(Braces { macros, segments }) => {
                if is_allowed_toc(macros) {
                    Some(Segment::Braces(Braces {
                        macros: macros.clone(),
                        segments: filter_for_toc(&segments),
                    }))
                } else {
                    None
                }
            }
            Segment::Link(Link { macros, href, text, title, .. }) => {
                if is_allowed_toc(macros) {
                    Some(Segment::Link(Link {
                        macros: macros.clone(),
                        href: href.clone(),
                        text: filter_for_toc(text),
                        title: title.clone(),
                        footnote: None,
                    }))
                } else {
                    None
                }
            }
            Segment::Image(Image { macros, href, alt, title }) => {
                if is_allowed_toc(macros) {
                    Some(Segment::Image(Image {
                        macros: macros.clone(),
                        href: href.clone(),
                        alt: filter_for_toc(alt),
                        title: title.clone(),
                    }))
                } else {
                    None
                }
            }
            Segment::Math(m) => {
                if is_allowed_toc(&m.macros) {
                    Some(Segment::Math(m.clone()))
                } else {
                    None
                }
            }
            Segment::InlineHtml(HtmlNode::Element(e)) => {
                if is_allowed_toc(&e.macros) {
                    Some(Segment::InlineHtml(HtmlNode::Element(e.clone())))
                } else {
                    None
                }
            }
            Segment::Code(c) => {
                if is_allowed_toc(&c.macros) {
                    Some(Segment::Code(Code {
                        macros: c.macros.clone(),
                        segments: filter_for_toc(&c.segments),
                    }))
                } else {
                    None
                }
            }

            Segment::Format(f) => Some(Segment::Format(InlineFormat {
                formatting: f.formatting,
                segments: filter_for_toc(&f.segments),
            })),

            s => Some(s.clone()),
        })
        .collect()
}

fn is_allowed_toc(m: &[Macro<'_>]) -> bool {
    m.iter().all(|m| !matches!(m, Macro::NoToc))
}
