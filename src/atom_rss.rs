/**
 * Mainly adapted from https://github.com/rust-syndication/syndication/blob/master/src/lib.rs
 * (but they dont seem to update the dependencies on either rss or atom...)
 *
 * Copyright (c) 2022 Lars Quentin, 2015 The rust-syndication Developers
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */


use std::str::FromStr;

use log::warn;

#[derive(Debug)]
pub enum Feed {
    Atom(atom_syndication::Feed),
    RSS(rss::Channel),
}

impl FromStr for Feed {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(feed) = atom_syndication::Feed::from_str(s) {
            return Ok(Feed::Atom(feed));
        }
        warn!("{} was not a Atom feed. Trying RSS now...", s);
        if let Ok(feed) = rss::Channel::from_str(s) {
            return Ok(Feed::RSS(feed));
        }
        log::error!("{} was not a Atom or RSS feed!", s);
        Err("Could not parse feed")
    }
}
