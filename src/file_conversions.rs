use std::fs;
use std::path::Path;
use std::time::Instant;

use anyhow::Context;
use ignore::Walk;
use rayon::iter::{ParallelBridge, ParallelIterator};
use unidok_repr::config::{Config, UnsafeConfig};

pub fn convert_file(
    input_path: &Path,
    output_path: &Path,
    verbosity: u8,
    is_unsafe: bool,
) -> anyhow::Result<()> {
    let content = fs::read_to_string(input_path)
        .with_context(|| format!("File `{}` couldn't be read", input_path.display()))?;

    let start = Instant::now();

    let mut input = unidok_parser::Input::new(&content);
    let mut config = Config::default();
    if is_unsafe {
        let cwd = std::env::current_dir().context("Could not get current directory path")?;
        config.unsafe_config = Some(UnsafeConfig { root: Some(cwd) });
    }
    let res = unidok_parser::parse(&mut input, config);

    let time1 = start.elapsed();

    let nodes = unidok_to_html::convert(res);
    let html = unidok_to_html::to_string(&nodes);

    let time2 = start.elapsed();

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Directory `{}` couldn't be created", parent.display()))?;
    }
    fs::write(output_path, html)
        .with_context(|| format!("File `{}` couldn't be written", output_path.display()))?;

    if verbosity > 0 {
        eprintln!();
        eprintln!("File: {}", input_path.display());
        eprintln!("   -> {}", output_path.display());
        if verbosity == 1 {
            eprintln!(" Parsed and rendered in {:.1?}", time2);
        } else {
            eprintln!("   Parsed in {:.1?}", time1);
            eprintln!(" Rendered in {:.1?}", time2 - time1);
        }

        if verbosity > 1 {
            for node in &nodes {
                eprintln!("{:#?}", node);
            }
        }
    }

    Ok(())
}

pub fn convert_dir(
    input: &Path,
    output: &Path,
    verbosity: u8,
    is_unsafe: bool,
) -> anyhow::Result<()> {
    Walk::new(input).par_bridge().try_for_each(|entry| {
        let entry =
            entry.with_context(|| format!("An entry in `{}` couldn't be read", input.display()))?;
        let path = entry.into_path();

        if is_unidok_file(&path)? {
            if let Ok(rel_path) = path.strip_prefix(input) {
                let output = output.join(rel_path).with_extension("html");
                convert_file(&path, &output, verbosity, is_unsafe)?;
            }
        }

        Ok(())
    })
}

fn is_unidok_file(path: &Path) -> anyhow::Result<bool> {
    let path2;

    let meta = path.metadata().with_context(|| {
        format!("Metadata for input file `{}` couldn't be retrieved", path.display())
    })?;

    let ext = if meta.is_file() {
        path.extension()
    } else if path.is_dir() {
        return Ok(false);
    } else {
        path2 = path
            .canonicalize()
            .with_context(|| format!("Path `{}` couldn't be canonicalized", path.display()))?;
        if path2.is_file() {
            path.extension()
        } else {
            return Ok(false);
        }
    };
    Ok(matches!(ext.and_then(|e| e.to_str()), Some(e) if e.eq_ignore_ascii_case("ud")))
}
