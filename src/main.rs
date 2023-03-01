use clap::Parser;
use std::path::PathBuf;
use std::io::{BufRead, BufReader};
use std::fs::File;
use anyhow::{Context, Result};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let path = args.path.to_str().with_context(|| "Could not understand file path.")?;
    let file = File::open(path).with_context(|| format!("Could not open file `{path}`"))?;
    let reader = BufReader::new(file);

    let mut pattern_count: usize = 0;
    for (line_number, line) in reader.lines().enumerate() {
        // Add a +1 offset
        let line_number = line_number + 1;
        let line = line.with_context(|| format!("Could not read file `{path}`"))?;

        if line.contains(&args.pattern) {
            pattern_count += 1;
            println!("{line_number}: {line}");
        }
    }
    println!("Pattern occurs {pattern_count} times.");

    Ok(())
}
