use crate::{Element, Node};

pub trait ToHtml {
    fn to_html(&self, buf: &mut String);
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
    fn to_html(&self, buf: &mut String) {
        match self {
            Node::Element(e) => e.to_html(buf),
            &Node::Text(t) => push_esc(t, buf),
            Node::Text2(t) => push_esc(t, buf),
            &Node::Cdata(c) => {
                buf.push_str("<![CDATA[");
                push_noesc(c, buf);
                buf.push_str("]]>");
            }
            &Node::Comment(c) => {
                buf.push_str("<!--");
                push_noesc(c, buf);
                buf.push_str("-->");
            }
            &Node::Doctype(d) => push_noesc(d, buf),
        }
    }
}

impl ToHtml for Element<'_> {
    fn to_html(&self, buf: &mut String) {
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
            content.to_html(buf);

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
    fn to_html(&self, buf: &mut String) {
        for n in self {
            n.to_html(buf);
        }
    }
}
