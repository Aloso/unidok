use std::fmt;

use crate::{Element, Node};

impl fmt::Debug for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Node::Element(ref e) => fmt::Debug::fmt(e, f),
            Node::Text(t) => fmt::Debug::fmt(t, f),
            Node::Text2(ref t) | Node::Verbatim(ref t) => fmt::Debug::fmt(t, f),
            Node::Cdata(d) => write!(f, "<![CDATA[{}]]>", d),
            Node::Comment(ref c) => write!(f, "<!--{}-->", c),
            Node::Doctype(d) => fmt::Display::fmt(d, f),
            Node::Fragment(ref d) => fmt::Debug::fmt(d, f),
        }
    }
}

impl fmt::Debug for Element<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        s.push('<');
        s.push_str(self.name.as_str());
        for a in &self.attrs {
            s.push(' ');
            s.push_str(a.key);
            if let Some(value) = &a.value {
                s.push_str(&format!("={:?}", value));
            }
        }

        if let Some(content) = &self.content {
            s.push('>');

            let mut t = &mut f.debug_tuple(&s);
            for b in content {
                t = t.field(b);
            }
            t.finish()
        } else {
            write!(f, "{} />", s)
        }
    }
}
