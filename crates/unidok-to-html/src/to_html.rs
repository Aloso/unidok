use crate::{Element, Node};

pub trait ToHtml {
    fn to_html(&self, buf: &mut String, within_inline: bool);
}

fn push_esc(s: &str, buf: &mut String) {
    for c in s.chars() {
        match c {
            '<' => buf.push_str("&lt;"),
            '>' => buf.push_str("&gt;"),
            '&' => buf.push_str("&amp;"),
            '"' => buf.push_str("&quot;"),
            '\0' => buf.push('\u{FFFD}'),
            _ => buf.push(c),
        }
    }
}

fn push_noesc(s: &str, buf: &mut String) {
    for c in s.chars() {
        if c == '\0' {
            buf.push('\u{FFFD}');
        } else {
            buf.push(c);
        }
    }
}

impl ToHtml for Node<'_> {
    fn to_html(&self, buf: &mut String, within_inline: bool) {
        match self {
            Node::Element(e) => e.to_html(buf, within_inline),
            &Node::Text(t) => push_esc(t, buf),
            Node::Text2(t) => push_esc(t, buf),
            &Node::Entity(t) => {
                buf.push('&');
                buf.push_str(t);
            }
            Node::Verbatim(t) => push_noesc(t, buf),
            &Node::Cdata(c) => {
                buf.push_str("<![CDATA[");
                push_noesc(c, buf);
                buf.push_str("]]>");
            }
            Node::Comment(content) => {
                buf.push_str("<!--");
                push_noesc(content, buf);
                buf.push_str("-->");
                if !within_inline {
                    buf.push('\n');
                }
            }
            &Node::Doctype(d) => push_noesc(d, buf),
            Node::Fragment(f) => {
                for n in f {
                    n.to_html(buf, within_inline);
                }
            }
        }
    }
}

impl ToHtml for Element<'_> {
    fn to_html(&self, buf: &mut String, within_inline: bool) {
        if within_inline && self.is_block_level {
            buf.push('\n');
        }
        buf.push('<');
        buf.push_str(self.name.as_str());

        for attr in &self.attrs {
            buf.push(' ');
            push_noesc(attr.key, buf);

            if let Some(value) = &attr.value {
                buf.push_str("=\"");
                push_esc(value, buf);
                buf.push('"');
            }
        }

        if let Some(content) = &self.content {
            buf.push('>');

            if self.contains_blocks {
                buf.push('\n');
            }
            content.to_html(buf, !self.contains_blocks);

            buf.push_str("</");
            buf.push_str(self.name.as_str());
            buf.push('>');
        } else {
            buf.push_str("/>");
        }
        if self.is_block_level {
            buf.push('\n');
        }
    }
}

impl ToHtml for [Node<'_>] {
    fn to_html(&self, buf: &mut String, within_inline: bool) {
        for n in self {
            n.to_html(buf, within_inline);
        }
    }
}
