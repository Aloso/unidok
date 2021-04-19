macro_rules! parse {
    ($text:literal, $parser:expr, None) => {{
        use crate::ir::IntoIR;

        let str = $text;
        let mut input = crate::Input::new(str);
        let parsed = input.parse($parser);
        if let None = parsed {
        } else {
            let ir = parsed.into_ir(str);
            eprintln!("INPUT: {:?}\n\nEXPECTED: None\n\nGOT: {:#?}\n", str, ir);
            panic!("assertion failed");
        }
    }};

    ($text:literal, $parser:expr, $expected:expr) => {{
        use crate::ir::IntoIR;

        let str = $text;
        let mut input = crate::Input::new(str);
        let parsed = input.parse($parser);
        let ir = parsed.into_ir(str);
        if ir != Some($expected) {
            eprintln!("INPUT: {:?}\n\nEXPECTED: {:#?}\n\nGOT: {:#?}\n", str, $expected, ir);
            panic!("assertion failed");
        }
    }};
}
