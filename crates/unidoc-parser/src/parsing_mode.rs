#[derive(Debug, Clone, Copy)]
pub(crate) enum ParsingMode {
    Nothing,
    Macros,
    Everything,
}

impl ParsingMode {
    pub(crate) fn try_from_macro(macro_name: &str, args: Option<&str>) -> Option<ParsingMode> {
        match macro_name {
            "PASS" => match args {
                Some("@" | "macros") => Some(ParsingMode::Macros),
                Some(_) => None,
                None => Some(ParsingMode::Everything),
            },
            "NOPASS" if args.is_none() => Some(ParsingMode::Nothing),
            _ => None,
        }
    }
}

// Eventually, parse modes will be fine-grained:
//
// - inline (i)
// - codeblocks (c)
// - headings (h)
// - thematicbreaks (b)
// - substitutions (s)
// - lists (l)
// - limiter ($)
// - macros (@)
// - math (%)
// - tables (|)
// - quotes (>)
// - html (<)
