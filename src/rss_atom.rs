use std::str::FromStr;

use crate::syndication::Feed as Synfeed;

use url::Url;

#[derive(Debug)]
pub struct Entry {
    title: String,
    updated: String, // TODO Time
    link: String, // TODO url
    summary: Option<String> // TODO
}

#[derive(Debug)]
pub struct Feed {
    pub title: String,
    pub entries: Vec<Entry>,
    pub updated: String, // TODO fime
}

impl Feed {
    fn from_atom(f: atom_syndication::Feed) -> Result<Self, String> {
        println!("{:?}", f.links());
        Ok(Feed {
            title: f.title().to_string(),
            entries: Vec::new(),
        })
    }

    fn from_rss(c: rss::Channel) -> Result<Self, String> {
        Ok(Feed {
            title: c.title().to_string(),
            entries: Vec::new(),
        })
    }
}

impl FromStr for Feed {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let syn_feed = Synfeed::from_str(s)?;
        match syn_feed {
            Synfeed::Atom(feed) => Self::from_atom(feed),
            Synfeed::RSS(channel) => Self::from_rss(channel),
        }
    }
}
