use std::time::Instant;

use unidoc_parser::parse;

fn main() {
    let input = std::env::args().nth(1).unwrap();

    let start = Instant::now();
    let res = parse(&input);
    let time1 = start.elapsed();
    let nodes = unidoc_to_html::convert(res);
    let html = unidoc_to_html::to_html(&nodes);
    let time2 = start.elapsed();

    println!("{}", html);

    for node in &nodes {
        eprintln!("{:#?}", node);
    }
    eprintln!("\nParsed in {:.1?}", time1);
    eprintln!("Rendered in {:.1?}", time2 - time1);
}
