use std::fs::File;
use std::io::{BufRead,BufReader};
use std::path::Path;

use anyhow::Result;
use clap::Parser;

const NAME: &str = "rssg";
const AUTHOR: &str = "github.com/lquenti";
const VERSION: &str = "0.1";
const ABOUT: &str = "A zero dependency single binary static site generator for RSS/Atom feeds";

#[derive(Parser, Debug)]
#[clap(name=NAME, author=AUTHOR, version=VERSION,about=ABOUT,long_about=None)]
struct Cli {
    /// Path to the newline-seperated list of RSS or Atom feeds
    path: String,
    /// Where to safe the generated RSS feed
    #[clap(short, long, default_value_t = String::from("./rss.html"))]
    outfile: String,
    /// Whether to override the current outfile path
    #[clap(short, long)]
    force: bool,
}

fn read_lines<P: AsRef<Path>>(filename: P) -> Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines()
        .filter_map(|l| l.ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(lines)
}

fn main() {
    let Cli {path, outfile, force} = Cli::parse();
    let urls = read_lines(path);
}
