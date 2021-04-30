use std::fs;

use similar::{ChangeTag, TextDiff};

const RED: &str = "\u{001b}[0;31m";
const YELLOW: &str = "\u{001b}[0;33m";
const GREEN: &str = "\u{001b}[0;32m";
const CYAN: &str = "\u{001b}[0;36m";
const BOLD: &str = "\u{001b}[1m";
const RESET: &str = "\u{001b}[0m";

fn main() {
    let update = matches!(std::env::var("UPDATE_TESTS"), Ok(s) if s == "1");

    if update {
        eprintln!("Updating tests...");
    } else {
        eprintln!("Running test suite...");
    }

    let mut c_failure = 0;
    let mut c_write = 0;
    let mut c_update = 0;

    let dir = fs::read_dir("./tests/test_cases").unwrap();
    let mut paths = dir
        .map(Result::unwrap)
        .filter(|e| e.file_type().unwrap().is_file())
        .map(|d| d.path())
        .collect::<Vec<_>>();
    paths.sort_unstable();

    for path in paths {
        let file_name = path.file_name().unwrap().to_os_string();
        let file_name = file_name.to_string_lossy();

        let content = fs::read_to_string(&path).unwrap();

        let mut split = content.split(SPLIT);
        let unidoc = split.next().unwrap();
        let expected = split.next();

        match test_case(unidoc, expected, update) {
            TcResult::Write(s) => {
                eprintln!("{}write{}   {}", CYAN, RESET, file_name);
                fs::write(path, format!("{}{}{}", unidoc, SPLIT, s)).unwrap();
                c_write += 1;
            }
            TcResult::Update(s) => {
                eprintln!("{}update{}  {}", YELLOW, RESET, file_name);
                fs::write(path, format!("{}{}{}", unidoc, SPLIT, s)).unwrap();
                c_update += 1;
            }
            TcResult::Success => {
                eprintln!("{}success{} {}", GREEN, RESET, file_name);
            }
            TcResult::Failure { got, expected } => {
                eprintln!("{}{}error{}   {}", RED, BOLD, RESET, file_name);
                eprintln!("{}{}DIFF:{}", CYAN, BOLD, RESET);

                let diff = TextDiff::from_lines(&expected, &got);

                for change in diff.iter_all_changes() {
                    match change.tag() {
                        ChangeTag::Delete => {
                            eprint!("{}- {}{}", RED, change, RESET);
                        }
                        ChangeTag::Insert => {
                            eprint!("{}+ {}{}", GREEN, change, RESET);
                        }
                        ChangeTag::Equal => {
                            eprint!("  {}", change);
                        }
                    }
                }
                eprintln!();

                c_failure += 1;
            }
        }
    }

    eprintln!();
    if c_failure > 0 {
        eprintln!("{}{}{} test(s) failed", RED, c_failure, RESET);
    }
    if c_write > 0 {
        eprintln!("{}{}{} test(s) were written", CYAN, c_write, RESET);
    }
    if c_update > 0 {
        eprintln!("{}{}{} test(s) were updated", YELLOW, c_update, RESET);
    }
}

const SPLIT: &str = "\n............................................................\n";

#[allow(dead_code)]
enum TcResult {
    Write(String),
    Update(String),
    Success,
    Failure { got: String, expected: String },
}

fn test_case(unidoc: &str, expected: Option<&str>, update: bool) -> TcResult {
    let res = unidoc_parser::parse(unidoc);
    let nodes = unidoc_to_html::convert(res);
    let mut html = unidoc_to_html::to_html(&nodes);
    if html.ends_with("\n\n") {
        html.pop();
    }

    match expected {
        None => TcResult::Write(html),
        Some(expected) if html == expected => TcResult::Success,
        Some(_) if update => TcResult::Update(html),
        Some(expected) => TcResult::Failure { got: html, expected: expected.into() },
    }
}
