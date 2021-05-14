use unidoc_repr::ir::html::ElemContentIr;
use unidoc_repr::ir::segments::SegmentIr;
use unidoc_repr::ir::IrState;

use crate::{IntoNode, IntoNodes, Node};

/// Converts the segments into nodes, while removing whitespace at the start and
/// end of the node.
pub(super) fn into_nodes_trimmed<'a>(
    mut segments: Vec<SegmentIr<'a>>,
    state: &IrState<'a>,
) -> Vec<Node<'a>> {
    while let Some(seg) = segments.last_mut() {
        if trim_segments_end(seg) {
            segments.pop();
        } else {
            break;
        }
    }

    let mut result = Vec::with_capacity(segments.len());
    let mut iter = segments.into_iter();

    while let Some(seg) = iter.next() {
        if let Some(seg) = trim_segments_start(seg) {
            result.push(seg.into_node(state));
            result.extend(iter.map(|n| n.into_node(state)));
            break;
        }
    }

    result
}

fn trim_segments_start(segment: SegmentIr<'_>) -> Option<SegmentIr<'_>> {
    match segment {
        SegmentIr::LineBreak | SegmentIr::Limiter => None,
        SegmentIr::Text(mut t) => {
            t = t.trim_start_matches(is_ws);
            Some(SegmentIr::Text(t)).filter(|_| !t.is_empty())
        }
        SegmentIr::Text2(t) => {
            let t = t.trim_start_matches(is_ws);
            if t.is_empty() {
                None
            } else {
                Some(SegmentIr::Text2(t.to_string()))
            }
        }
        SegmentIr::EscapedText(mut t) => {
            t = t.trim_start_matches(is_ws);
            Some(SegmentIr::EscapedText(t)).filter(|_| !t.is_empty())
        }
        s => Some(s),
    }
}

fn trim_segments_end(seg: &mut SegmentIr) -> bool {
    match seg {
        SegmentIr::LineBreak | SegmentIr::Limiter => true,
        SegmentIr::Text(t) | SegmentIr::EscapedText(t) => {
            *t = t.trim_end_matches(is_ws);
            t.is_empty()
        }
        SegmentIr::Text2(t) => {
            while t.ends_with(is_ws) {
                t.pop();
            }
            t.is_empty()
        }
        _ => false,
    }
}

pub(super) fn elem_content_ir_into_nodes<'a>(
    content: ElemContentIr<'a>,
    state: &IrState<'a>,
) -> Vec<Node<'a>> {
    match content {
        ElemContentIr::Blocks(b) => b.into_nodes(state),
        ElemContentIr::Inline(i) => i.into_nodes(state),
        ElemContentIr::Verbatim(v) => vec![Node::Verbatim(v)],
    }
}

/// Returns whether this is a space or tab.
#[inline]
fn is_ws(c: char) -> bool {
    matches!(c, ' ' | '\t')
}
