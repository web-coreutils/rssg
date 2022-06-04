use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::path::Path;

use anyhow::Result;
use clap::Parser;
use log::info;
use url::Url;

mod syndication;
mod rss_atom;

const NAME: &str = "rssg";
const AUTHOR: &str = "github.com/lquenti";
const VERSION: &str = "0.1";
const ABOUT: &str =
    "A (nearly) zero dependency single binary static site generator for RSS/Atom feeds";

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

// TODO: Log errors if sth isn't parsable
fn read_urls<P: AsRef<Path>>(filename: P) -> Result<Vec<Url>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let lines: Vec<Url> = reader
        .lines()
        .filter_map(|l| l.ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .map(|s| Url::parse(&s))
        .filter_map(|s| s.ok())
        .collect();

    Ok(lines)
}

// TODO make async for performance with a map to futures, which can later be joined
// (and the logging should be part of the join)
fn download_url(u: Url) -> Result<String> {
    Ok(reqwest::blocking::get(u)?.text()?)
}

fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );
    info!("Parsing CLI");
    let Cli { path, .. } = Cli::parse();

    info!("Reading URLs");
    let urls = read_urls(path).expect("Could not read url file...");
    info!("{} urls found.", urls.len());

    let mut xs = Vec::new();
    for url in urls {
        info!("Downloading {}", &url);
        match download_url(url) {
            Ok(html) => xs.push(html),
            Err(e) => log::warn!("Download failed with error \"{}\"", e),
        };
    }

    for html in xs {
        // TODO Error handling
        let feed = rss_atom::Feed::from_str(&html).unwrap();
        println!("{:?}", feed);
    }
}
