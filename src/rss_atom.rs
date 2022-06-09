use std::str::FromStr;

use crate::syndication::Feed as Synfeed;

use sailfish::TemplateOnce;

// TODO type me correctly and implement from
#[derive(Debug)]
pub struct Entry {
    title: String,
    link: String,
    icon: String,
    source_name: String,
    source_link: String,
}

impl Entry {
    // TODO proper lifetimes
    fn from_atom(f: atom_syndication::Feed) -> Result<Vec<Self>, String> {
        Ok(f.entries.iter()
            .map(|atom_syndication::Entry { title, links, .. }| {
                Entry {
                    title: title.to_string(),
                    link: links.first().unwrap().href.clone(),
                    //icon: f.icon.as_ref().unwrap().to_string(),
                    icon: String::new(),
                    source_name: f.title.to_string(),
                    source_link: f.links.first().unwrap().href.clone(),
                }
            }).collect()
        )
    }

    fn from_rss(_c: rss::Channel) -> Result<Vec<Self>, String> {
        Err(String::new())
    }
}

#[derive(Debug, TemplateOnce)]
#[template(path = "feed_template.html")]
pub struct Feed {
    pub entries: Vec<Entry>,
    pub updated: String, // TODO: Maybe change type?
}

impl Feed {
    fn from_atom(f: atom_syndication::Feed) -> Result<Self, String> {
        println!("{:?}", &f.links());
        Ok(Feed {
            updated: f.updated.to_string(),
            entries: Entry::from_atom(f)?,
        })
    }

    fn from_rss(c: rss::Channel) -> Result<Self, String> {
        Ok(Feed {
            entries: Entry::from_rss(c)?,
            updated: String::new(),
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
