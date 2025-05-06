# feed-parser

[![test](https://github.com/akitenkrad/rs-feed-parser/actions/workflows/unit_test.yml/badge.svg)](https://github.com/akitenkrad/rs-feed-parser/actions/workflows/unit_test.yml)

This module contains various parsers used in the feed-parser library.

The parsers module provides functionality to parse different types of
feed formats, such as RSS and Atom. Each parser is responsible for
handling the specific details of its respective feed format, ensuring
that the feed data is correctly interpreted and converted into a
standardized format for further processing.

## Quick Start

```rust
use feed_parser::parsers::{Feed, rss2};

let rss_data = r#"
<rss version="2.0">
    <channel>
        <title>RSS Title</title>
        <link>http://www.example.com/main.html</link>
        <description>This is an example of an RSS feed</description>
        <item>
            <title>Item 1</title>
            <link>http://www.example.com/item1.html</link>
            <description>Item 1 description</description>
            <pubDate>2024-01-01T23:59:02Z</pubDate>
        </item>
    </channel>
</rss>
"#;
let feeds: Vec<Feed> = rss2::parse(rss_data).unwrap();
assert_eq!(feeds[0].title, "Item 1");
assert_eq!(feeds[0].link, "http://www.example.com/item1.html");
assert_eq!(feeds[0].description.clone().unwrap(), "Item 1 description");
```

## Modules

- `rss1`: Contains the RSS1.0 parser implementation.
- `rss2`: Contains the RSS2.0 parser implementation.
- `atom`: Contains the Atom parser implementation.

## Release Notes

- v1.0.12
  - Added `AppError` and `AppResult` structs.
- v1.0.11
  - Made it possible to parse HTML in `<title></title>` and `<description></description>` sections.

## [For developers documents](docs/README.md)
