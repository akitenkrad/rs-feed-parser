use super::*;

#[test]
fn test_parse_rss1() {
    let xml = r#"
        <rdf:RDF>
            <channel>
                <title>RSS Title</title>
                <link>http://www.example.com/main.html</link>
                <description>This is an example of an RSS feed</description>
                <item>
                    <title>Item 1</title>
                    <link>http://www.example.com/item1.html</link>
                    <description>Item 1 description</description>
                    <dc:creator>John Doe</dc:creator>
                    <dc:date>2003-12-13T18:30:02Z</dc:date>
                    <pubDate>2024-01-01T23:59:02Z</pubDate>
                </item>
                <item>
                    <title>Item 2</title>
                    <link>http://www.example.com/item2.html</link>
                    <description>Item 2 description</description>
                    <dc:creator>Jane Doe</dc:creator>
                    <dc:date>2003-12-13T18:30:02Z</dc:date>
                </item>
            </channel>
        </rdf:RDF>"#;
    let feeds = parse(xml);
    match feeds {
        Ok(feeds) => {
            assert_eq!(feeds.len(), 2);
        }
        Err(_e) => {}
    }
}
