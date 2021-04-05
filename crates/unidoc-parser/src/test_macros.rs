macro_rules! parse {
    ($text:literal, $parser:expr, None) => {{
        use crate::statics::IsStatic;

        let str = $text;
        let mut input = crate::Input::new(str);
        let parsed = input.parse($parser);
        if !parsed.is(None, str) {
            eprintln!("INPUT: {:?}\n\nEXPECTED: None\n\nGOT: {:#?}\n", str, parsed);
            panic!("assertion failed");
        }
    }};

    ($text:literal, $parser:expr, $expected:expr) => {{
        use crate::statics::IsStatic;

        let str = $text;
        let mut input = crate::Input::new(str);
        let parsed = input.parse($parser);
        if !parsed.is(Some($expected), str) {
            eprintln!(
                "INPUT: {:?}\n\nEXPECTED: {:#?}\n\nGOT: {:#?}\n",
                str, $expected, parsed
            );
            panic!("assertion failed");
        }
    }};
}

macro_rules! braces {
    ($( $e:expr ),* $(,)?) => {
        StaticBraces { content: &[ $($e),* ] }
    };
}
