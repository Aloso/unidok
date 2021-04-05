use unidoc_parser::items::Node;
use unidoc_parser::Input;

fn main() {
    let input = std::env::args().nth(1).unwrap();

    let mut input = Input::new(input);
    let res = input.parse(Node::global_parser());
    dbg!(res);
}
