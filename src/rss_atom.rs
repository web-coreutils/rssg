use std::cmp::{self, Ordering};
use std::str::FromStr;

use crate::syndication::Feed as Synfeed;

// TODO: Check if chrono is even needed

pub use atom_syndication::FixedDateTime;
use sailfish::TemplateOnce;

// TODO type me correctly and implement from
// TODO: Needs time to be sorted by relevance
// TODO less ownership
#[derive(Debug, Eq)]
pub struct Entry {
    title: String,
    link: String,
    icon: String,
    source_name: String,
    source_link: String,
    updated: FixedDateTime,
}

impl Entry {
    // TODO proper lifetimes
    fn from_atom(f: atom_syndication::Feed) -> Result<Vec<Self>, String> {
        Ok(f.entries.iter()
            .map(|atom_syndication::Entry { title, links, updated, .. }| {
                Entry {
                    title: title.to_string(),
                    link: links.first().unwrap().href.clone(),
                    //icon: f.icon.as_ref().unwrap().to_string(),
                    icon: String::new(),
                    source_name: f.title.to_string(),
                    source_link: f.links.first().unwrap().href.clone(),
                    updated: updated.clone(),
                }
            }).collect()
        )
    }

    fn from_rss(_c: rss::Channel) -> Result<Vec<Self>, String> {
        Err(String::new())
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.updated.cmp(&other.updated)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.updated == other.updated
    }
}

#[derive(Debug, TemplateOnce)]
#[template(path = "feed_template.html")]
pub struct Feed {
    pub entries: Vec<Entry>,
    pub updated: FixedDateTime,
}

impl Feed {
    fn from_atom(f: atom_syndication::Feed) -> Result<Self, String> {
        println!("{:?}", &f.links());
        Ok(Feed {
            updated: f.updated.clone(),
            entries: Entry::from_atom(f)?,
        })
    }

    fn from_rss(c: rss::Channel) -> Result<Self, String> {
        Err(String::new())
        /*
        Ok(Feed {
            entries: Entry::from_rss(c)?,
            updated: FixedDateTime::new(),
        })
        */
    }

    // TODO: Maybe less copies
    pub fn join(mut self, mut other: Self)-> Self{
        let updated = cmp::max(self.updated, other.updated);
        // TODO merge iterators in O(n)
        // TODO: Why the ref?
        self.entries.append(&mut other.entries);
        self.entries.sort();
        Self {entries: self.entries, updated }
    }


    pub fn join_many(xs: &[Self]){
        let res = xs.iter().reduce(|xs, y| xs.join(y));
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
