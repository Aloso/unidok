use crate::ast::segments::Segment;

pub fn collapse_text(mut segments: Vec<Segment>) -> Vec<Segment> {
    let len = segments.len();
    if len > 1 {
        let mut p1 = 0;
        let mut p2 = 1;
        while p2 < len {
            let (s1, s2) = get_mut_2(&mut segments, p1, p2);
            if let (Segment::Text(t1), Segment::Text(t2)) = (&mut *s1, &mut *s2) {
                if let Some(joined) = t1.try_join(*t2) {
                    *t1 = joined;
                    *s2 = Default::default();
                    p2 += 1;
                    continue;
                }
            }

            p1 += 1;
            if p1 < p2 {
                let (s1, s2) = get_mut_2(&mut segments, p1, p2);
                *s1 = std::mem::take(s2);
            }
            p2 += 1;
        }

        p1 += 1;
        if p1 < p2 {
            drop(segments.drain(p1..));
        }
    }

    segments
}

fn get_mut_2(segments: &mut Vec<Segment>, i1: usize, i2: usize) -> (&mut Segment, &mut Segment) {
    let (left, right) = segments.as_mut_slice().split_at_mut(i2);
    (&mut left[i1], &mut right[0])
}
