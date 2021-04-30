use unidoc_parser::ir::{ElemContentIr, SegmentIr};

use crate::{IntoNode, IntoNodes, Node};

pub(super) fn into_nodes_trimmed(mut segments: Vec<SegmentIr<'_>>) -> Vec<Node<'_>> {
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
            result.push(seg.into_node());
            result.extend(iter.map(IntoNode::into_node));
            break;
        }
    }

    result
}

fn trim_segments_start(segment: SegmentIr<'_>) -> Option<SegmentIr<'_>> {
    match segment {
        SegmentIr::LineBreak | SegmentIr::Limiter => None,
        SegmentIr::Text(mut t) => {
            t = t.trim_start_matches(|c| matches!(c, ' ' | '\t'));
            Some(SegmentIr::Text(t)).filter(|_| !t.is_empty())
        }
        SegmentIr::EscapedText(mut t) => {
            t = t.trim_start_matches(|c| matches!(c, ' ' | '\t'));
            Some(SegmentIr::EscapedText(t)).filter(|_| !t.is_empty())
        }
        s => Some(s),
    }
}

fn trim_segments_end(seg: &mut SegmentIr) -> bool {
    match seg {
        SegmentIr::LineBreak | SegmentIr::Limiter => true,
        SegmentIr::Text(t) | SegmentIr::EscapedText(t) => {
            *t = t.trim_end_matches(|c| matches!(c, ' ' | '\t'));
            t.is_empty()
        }
        _ => false,
    }
}

pub(super) fn elem_content_ir_into_nodes(content: ElemContentIr<'_>) -> Vec<Node<'_>> {
    match content {
        ElemContentIr::Blocks(b) => b.into_nodes(),
        ElemContentIr::Inline(i) => i.into_nodes(),
        ElemContentIr::Verbatim(v) => vec![Node::Text(v)],
    }
}
