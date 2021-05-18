use std::time::{Duration, Instant};
use std::{fs, process};

use similar::{ChangeTag, TextDiff};

const RED: &str = "\u{001b}[0;31m";
const YELLOW: &str = "\u{001b}[0;33m";
const GREEN: &str = "\u{001b}[0;32m";
const CYAN: &str = "\u{001b}[0;36m";
const BOLD: &str = "\u{001b}[1m";
const RESET: &str = "\u{001b}[0m";

fn main() {
    if run_test() != 0 {
        process::abort();
    }
}

fn run_test() -> usize {
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
        let unidok = split.next().unwrap();
        let expected = split.next();

        let (result, parsing_time, rendering_time) = test_case(
            file_name.to_string(),
            unidok.to_string(),
            expected.map(ToString::to_string),
            update,
        );

        match result {
            TcResult::Write(s) => {
                print_line(CYAN, "write", &file_name, parsing_time, rendering_time);
                fs::write(path, format!("{}{}{}", unidok, SPLIT, s)).unwrap();
                c_write += 1;
            }
            TcResult::Update(s) => {
                print_line(YELLOW, "update", &file_name, parsing_time, rendering_time);
                fs::write(path, format!("{}{}{}", unidok, SPLIT, s)).unwrap();
                c_update += 1;
            }
            TcResult::Success => {
                print_line(GREEN, "success", &file_name, parsing_time, rendering_time);
            }
            TcResult::Failure { got, expected } => {
                print_line(RED, "error", &file_name, parsing_time, rendering_time);
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
        eprintln!("{}{}{} test case(s) failed", RED, c_failure, RESET);
    }
    if c_write > 0 {
        eprintln!("{}{}{} test case(s) were written", CYAN, c_write, RESET);
    }
    if c_update > 0 {
        eprintln!("{}{}{} test case(s) were updated", YELLOW, c_update, RESET);
    }

    c_failure
}

const SPLIT: &str = "\n............................................................\n";

#[allow(dead_code)]
enum TcResult {
    Write(String),
    Update(String),
    Success,
    Failure { got: String, expected: String },
}

fn test_case(
    file_name: String,
    unidok: String,
    expected: Option<String>,
    update: bool,
) -> (TcResult, Duration, Duration) {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::thread;

    let finished = Arc::new(AtomicBool::new(false));
    let finished2 = Arc::clone(&finished);

    let handle = thread::spawn(move || {
        let start = Instant::now();
        let res = unidok_parser::parse(&unidok, false);
        let parsing_time = start.elapsed();

        let nodes = unidok_to_html::convert(res);
        let html = unidok_to_html::to_string(&nodes);
        let rendering_time = start.elapsed() - parsing_time;

        finished.store(true, Ordering::Release);
        (html, parsing_time, rendering_time)
    });

    thread::spawn(move || {
        for _ in 0..1000 {
            thread::sleep(Duration::from_millis(5));
            if finished2.load(Ordering::Acquire) {
                return;
            }
        }
        eprintln!("  ...   {}{}{}  is taking very long!", CYAN, file_name, RESET);
    });

    let (mut html, parsing_time, rendering_time) = handle.join().unwrap();
    if html.ends_with("\n\n") {
        html.pop();
    }

    (
        match expected {
            None => TcResult::Write(html),
            Some(expected) if html == expected => TcResult::Success,
            Some(_) if update => TcResult::Update(html),
            Some(expected) => TcResult::Failure { got: html, expected },
        },
        parsing_time,
        rendering_time,
    )
}

fn print_line(
    color: &str,
    status: &str,
    file_name: &str,
    parsing_time: Duration,
    rendering_time: Duration,
) {
    eprintln!(
        "{}{:7}{} {:25} parsed in {:.1?}, rendered in {:.1?}",
        color, status, RESET, file_name, parsing_time, rendering_time,
    );
}
