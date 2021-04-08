use unidoc_parser::{Input, Node};

fn main() {
    let input = std::env::args().nth(1).unwrap();

    let mut input = Input::new(input);
    let res = input.parse(Node::global_parser());
    dbg!(res);
}
