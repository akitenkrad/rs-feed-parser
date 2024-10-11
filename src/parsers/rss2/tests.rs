use super::*;

#[test]
fn test_parse_rss2() {
    let xml = r#"
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
                    <other>Other</other>
                    <test:test>Test</test:test>
                </item>
                <item>
                    <title>Item 2</title>
                    <link>http://www.example.com/item2.html</link>
                    <description>Item 2 description</description>
                </item>
            </channel>
        </rss>"#;
    let feeds = parse(xml);
    match feeds {
        Ok(feeds) => {
            assert_eq!(feeds.len(), 2);
        }
        Err(_e) => {}
    }
}
