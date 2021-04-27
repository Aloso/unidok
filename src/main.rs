use std::time::Instant;

use unidoc_parser::parse;

fn main() {
    let input = std::env::args().nth(1).unwrap();

    let start = Instant::now();
    let res = parse(&input);
    let time1 = start.elapsed();
    let html = unidoc_to_html::convert(res);
    let time2 = start.elapsed();

    dbg!(html);
    eprintln!("Parsed in {:.1?}", time1);
    eprintln!("Rendered in {:.1?}", time2 - time1);
}
