macro_rules! parse {
    ($text:literal, $parser:expr, None) => {{
        use unidok_repr::ast::AstData;
        use unidok_repr::config::Config;
        use unidok_repr::IntoIR;

        let str = $text;
        let mut input = crate::Input::new(str);
        let parsed = input.parse($parser);
        if let None = parsed {
        } else {
            let ir = parsed.into_ir(str, &mut AstData::new(Config::default()));
            eprintln!("INPUT: {:?}\n\nEXPECTED: None\n\nGOT: {:#?}\n", str, ir);
            panic!("assertion failed");
        }
    }};

    ($text:literal, $parser:expr, $expected:expr) => {{
        use unidok_repr::ast::AstData;
        use unidok_repr::config::Config;
        use unidok_repr::IntoIR;

        let str = $text;
        let mut input = crate::Input::new(str);
        let parsed = input.parse($parser);
        let ir = parsed.into_ir(str, &mut AstData::new(Config::default()));
        if ir != Some($expected) {
            eprintln!("INPUT: {:?}\n\nEXPECTED: {:#?}\n\nGOT: {:#?}\n", str, $expected, ir);
            panic!("assertion failed");
        }
    }};
}
