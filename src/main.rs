use unidoc_parser::{Input, NodeParentKind, ParseNodes};

fn main() {
    let input = std::env::args().nth(1).unwrap();

    let mut input = Input::new(input);
    let res = input
        .parse(ParseNodes { parent: NodeParentKind::Global, ind: Default::default() });
    dbg!(res);
}
