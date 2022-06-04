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

fn main() {
    let Cli {path, outfile, force} = Cli::parse();
}
