use std::fs;
use std::path::Path;
use std::time::Instant;

use anyhow::{bail, Context};
use clap::{App, AppSettings, Arg, SubCommand};
use unidok_repr::config::{Config, UnsafeConfig};

use crate::file_conversions::{convert_dir, convert_file};

mod file_conversions;

fn app() -> clap::App<'static, 'static> {
    App::new("unidok")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Ludwig Stecher <ludwig.stecher@gmx.de>")
        .about("Unidok document converter")
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .long("verbose")
                .help("Makes output more verbose. Can be specified up to 3 times.")
                .multiple(true)
                .takes_value(false)
                .global(true),
        )
        .subcommand(
            SubCommand::with_name("to-html")
                .visible_alias("t")
                .aliases(&["to_html", "tohtml", "to-htm", "ot-html", "to-hmtl"])
                .about("Convert a file or directory to HTML")
                .args(&[
                    Arg::with_name("in")
                        .short("i")
                        .long("in")
                        .value_name("PATH")
                        .help("The file or directory to convert")
                        .required(true),
                    Arg::with_name("out")
                        .short("o")
                        .long("out")
                        .value_name("PATH")
                        .help("The file or directory where the HTML output should be saved")
                        .required(true),
                    Arg::with_name("unsafe")
                        .takes_value(false)
                        .help("Enable unsafe mode, which allows things like file system access"),
                ]),
        )
        .subcommand(
            SubCommand::with_name("stdio")
                .visible_alias("s")
                .aliases(&["stio", "sdio", "sdtio", "tsdio", "stdoi", "stido"])
                .about("Convert the standard input to HTML and prints it")
                .args(&[
                    Arg::with_name("input")
                        .value_name("INPUT")
                        .help("The Unidok text to convert")
                        .required(true),
                    Arg::with_name("unsafe")
                        .takes_value(false)
                        .help("Enable unsafe mode, which allows things like file system access"),
                ]),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
}

fn main() -> anyhow::Result<()> {
    let start = Instant::now();

    let args = app().get_matches();

    let verbosity = args.occurrences_of("verbosity");
    if verbosity > 3 {
        bail!("Verbosity level must be between 0 and 3");
    }
    let verbosity = verbosity as u8;

    if let Some(args) = args.subcommand_matches("to-html") {
        let input = args.value_of_os("in").context("missing --in")?;
        let output = args.value_of_os("out").context("missing --out")?;
        let is_unsafe = args.is_present("unsafe");

        let input = Path::new(input);
        let output = Path::new(output);

        let input = input
            .canonicalize()
            .with_context(|| format!("Path `{}` couldn't be canonicalized", input.display()))?;

        let meta = fs::metadata(&input).with_context(|| {
            format!("Metadata for input file `{}` couldn't be retrieved", input.display())
        })?;

        let file_type = meta.file_type();

        if file_type.is_file() {
            convert_file(&input, &output, verbosity, is_unsafe)?;
        } else if file_type.is_dir() {
            convert_dir(&input, &output, verbosity, is_unsafe)?;
        } else {
            bail!("The specified path `{}` is not a file or directory", input.display());
        }
    } else if let Some(args) = args.subcommand_matches("stdio") {
        let input_str = args.value_of("input").context("missing input")?;
        let is_unsafe = args.is_present("unsafe");

        let mut input = unidok_parser::Input::new(&input_str);
        let mut config = Config::default();
        if is_unsafe {
            config.unsafe_config = Some(UnsafeConfig { root: None });
        }

        let res = unidok_parser::parse(&mut input, config);
        let nodes = unidok_to_html::convert(res);
        let html = unidok_to_html::to_string(&nodes);
        println!("{}", html);
    }

    let time = start.elapsed();
    if verbosity > 0 {
        eprintln!();
        eprintln!("Whole job completed in {:.1?}", time);
    }

    Ok(())
}
