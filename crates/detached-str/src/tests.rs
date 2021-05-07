use crate::{Str, StrSlice};

///////////////////// Range

#[test]
fn range() {
    let s: Str = "Hello, world!".into();
    assert_eq!(s.get(0..0), StrSlice { start: 0, end: 0 });
    assert_eq!(s.get(0..4), StrSlice { start: 0, end: 4 });
    assert_eq!(s.get(0..13), StrSlice { start: 0, end: 13 });
    assert_eq!(s.get(13..13), StrSlice { start: 13, end: 13 });

    assert!(s.get(0..0).is_empty());
    assert!(s.get(13..13).is_empty());
    assert!(!s.get(12..13).is_empty());

    assert_eq!(s.get(5..5).to_str(&s), "");
    assert_eq!(s.get(12..13).to_str(&s), "!");
}

#[test]
#[should_panic]
fn range_panicking1() {
    let s: Str = "Hello, world!".into();
    s.get(0..14);
}

#[test]
#[should_panic]
fn range_panicking2() {
    let s: Str = "Hello, world!".into();
    s.get(14..14);
}

#[test]
#[should_panic]
fn range_panicking3() {
    let s: Str = "Hello, world!".into();
    #[allow(clippy::reversed_empty_ranges)]
    s.get(3..1);
}

///////////////////// RangeFrom

#[test]
fn range_from() {
    let s: Str = "Hello, world!".into();
    assert_eq!(s.get(0..), StrSlice { start: 0, end: 13 });
    assert_eq!(s.get(13..), StrSlice { start: 13, end: 13 });

    assert!(s.get(13..).is_empty());
    assert!(!s.get(12..).is_empty());

    assert_eq!(s.get(5..).to_str(&s), ", world!");
}

#[test]
#[should_panic]
fn range_from_panicking1() {
    let s: Str = "Hello, world!".into();
    s.get(14..);
}

#[test]
#[should_panic]
fn range_from_panicking2() {
    let s: Str = "".into();
    s.get(1..);
}

///////////////////// RangeFull

#[test]
fn range_full() {
    let s: Str = "Hello, world!".into();
    assert_eq!(s.get(..), StrSlice { start: 0, end: 13 });
    assert_eq!(s.get(..).to_str(&s), "Hello, world!");

    let s: Str = "".into();
    assert_eq!(s.get(..), StrSlice { start: 0, end: 0 });
}

///////////////////// RangeInclusive

#[test]
fn range_inclusive() {
    let s: Str = "Hello, world!".into();
    assert_eq!(s.get(0..=0), StrSlice { start: 0, end: 1 });
    assert_eq!(s.get(0..=4), StrSlice { start: 0, end: 5 });
    assert_eq!(s.get(0..=12), StrSlice { start: 0, end: 13 });

    assert!(!s.get(0..=0).is_empty());

    assert_eq!(s.get(0..=0).to_str(&s), "H");
    assert_eq!(s.get(5..=5).to_str(&s), ",");
    assert_eq!(s.get(0..=12).to_str(&s), "Hello, world!");
}

#[test]
#[should_panic]
fn range_inclusive_panicking1() {
    let s: Str = "Hello, world!".into();
    s.get(0..=13);
}

#[test]
#[should_panic]
fn range_inclusive_panicking2() {
    let s: Str = "Hello, world!".into();
    s.get(13..=14);
}

#[test]
#[should_panic]
fn range_inclusive_panicking3() {
    let s: Str = "Hello, world!".into();
    #[allow(clippy::reversed_empty_ranges)]
    s.get(3..=1);
}

///////////////////// RangeTo

#[test]
fn range_to() {
    let s: Str = "Hello, world!".into();
    assert_eq!(s.get(..0), StrSlice { start: 0, end: 0 });
    assert_eq!(s.get(..5), StrSlice { start: 0, end: 5 });
    assert_eq!(s.get(..13), StrSlice { start: 0, end: 13 });

    assert!(s.get(..0).is_empty());
    assert!(!s.get(..1).is_empty());

    assert_eq!(s.get(..1).to_str(&s), "H");
    assert_eq!(s.get(..13).to_str(&s), "Hello, world!");
}

#[test]
#[should_panic]
fn range_to_panicking1() {
    let s: Str = "Hello, world!".into();
    s.get(..14);
}

///////////////////// RangeToInclusive

#[test]
fn range_to_inclusive() {
    let s: Str = "Hello, world!".into();
    assert_eq!(s.get(..=0), StrSlice { start: 0, end: 1 });
    assert_eq!(s.get(..=4), StrSlice { start: 0, end: 5 });
    assert_eq!(s.get(..=12), StrSlice { start: 0, end: 13 });

    assert!(!s.get(..=0).is_empty());

    assert_eq!(s.get(..=0).to_str(&s), "H");
}

#[test]
#[should_panic]
fn range_to_inclusive_panicking1() {
    let s: Str = "Hello, world!".into();
    s.get(0..=13);
}
