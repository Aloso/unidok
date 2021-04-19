use std::time::Instant;

use unidoc_parser::parse;

fn main() {
    let input = std::env::args().nth(1).unwrap();

    let start = Instant::now();
    let res = parse(&input);
    let time = start.elapsed();

    dbg!(res);
    eprintln!("Parsed in {:.1?}", time);
}
