# RSS SSG

## Text format specification
- Just write the urls down one at a time.
- Comments are not allowed since they could all be valid URLs
- empty lines are okay

Note that this is still a WIP, we will probably switch to sth like yml/toml soon...

Example: 
```
<URL_TO_RSS_OR_ATOM_FEED>
<URL_TO_RSS_OR_ATOM_FEED>
<URL_TO_RSS_OR_ATOM_FEED>
...
```

## Dependencies
- `pkg-config` for reqwest
- `libssl-dev` for reqwest

## License

MIT 2022 Lars Quentin

Some code is also copied from [syndication](https://github.com/rust-syndication/syndication/blob/master/LICENSE-MIT), Copyright (c) 2015 The rust-syndication Developers
