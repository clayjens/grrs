use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about)]
#[command(propagate_version = true)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: PathBuf,
    /// Whether to search case sensitive
    #[arg(short, long, default_value_t = false)]
    case_sensitive: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let path = args
        .path
        .to_str()
        .with_context(|| "Could not understand file path.")?;
    let file = File::open(path).with_context(|| format!("Could not open file `{path}`"))?;
    let reader = BufReader::new(file);

    dbg!(&args);

    let mut pattern_count: usize = 0;
    for (line_number, line) in reader.lines().enumerate() {
        // Add a +1 offset
        let line_number = line_number + 1;
        let mut line = line.with_context(|| format!("Could not read file `{path}`"))?;
        let mut pattern = args.pattern.to_owned();

        if !args.case_sensitive {
            line.make_ascii_lowercase();
            pattern.make_ascii_lowercase();
        }

        if line.contains(&pattern) {
            pattern_count += 1;

            println!("{line_number}: {line}");
        }
    }
    println!("Pattern occurs {pattern_count} times.");

    Ok(())
}
