use unidoc_parser::items::{Node, ParentKind};
use unidoc_parser::Input;

fn main() {
    let input = std::env::args().nth(1).unwrap();

    let mut input = Input::new(input);
    let res = input.parse(Node::multi_parser(ParentKind::Global, Default::default()));
    dbg!(res);
}
