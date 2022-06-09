use std::fs::File;
use std::io::{Write, BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

use anyhow::Result;
use clap::Parser;
use log::info;
use url::Url;
use sailfish::TemplateOnce;

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
    /// Where to save the generated feed
    #[clap(short, long, default_value_t = String::from("./feed.html"))]
    outfile: String,
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
    let Cli { path, outfile } = Cli::parse();

    info!("Reading URLs");
    let urls = read_urls(path).expect("Could not read url file...");
    info!("{} urls found.", urls.len());

    let mut xs = Vec::new();
    for url in urls {
        info!("Downloading {}", &url);
        match download_url(url) {
            Ok(xml) => xs.push(xml),
            Err(e) => log::warn!("Download failed with error \"{}\"", e),
        };
    }

    info!("Generating HTML");
    // check what happens if file exist bc of force cli
    // TODO unwrap
    let mut fp = std::fs::File::create(outfile).unwrap();
    // TODO join all
    write!(fp, "{}", rss_atom::Feed::from_str(xs.first().unwrap()).unwrap().render_once().unwrap()).unwrap();
}
