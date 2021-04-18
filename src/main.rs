use std::time::Instant;

use unidoc_parser::{Input, Node};

fn main() {
    let input = std::env::args().nth(1).unwrap();

    let start = Instant::now();

    let mut input = Input::new(input);
    let res = input.parse(Node::global_parser());

    let time = start.elapsed();

    dbg!(res);
    eprintln!("Parsed in {:.1?}", time);
}
