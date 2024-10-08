use super::*;

#[test]
fn test_parse_atom() {
    let xml = r#"
        <feed xmlns="http://www.w3.org/2005/Atom">
            <title>Example Feed</title>
            <link href="http://example.org/"/>
            <updated>2003-12-13T18:30:02Z</updated>
            <author>
                <name>John Doe</name>
            </author>
            <entry>
                <title>Atom-Powered Robots Run Amok</title>
                <link href="http://example.org/2003/12/13/atom03"/>
                <id>urn:uuid:1225c695-cfb8-4ebb-aaaa-80da344efa6a</id>
                <updated>2003-12-13T18:30:02Z</updated>
                <summary>Some text.</summary>
            </entry>
            <entry>
                <title>Item 2</title>
                <link href="http://www.example.com/item2.html"/>
                <summary>Item 2 description</summary>
            </entry>
        </feed>"#;
    let feeds = parse(xml);
    assert_eq!(feeds.len(), 2);
}
