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
                <link href="http://example.org/2003/12/13/atom03" type="image/png" rel="enclosure" />
                <id>urn:uuid:1225c695-cfb8-4ebb-aaaa-80da344efa6a</id>
                <updated>2003-12-13T18:30:02Z</updated>
                <summary>Some text.</summary>
            </entry>
            <entry>
                <title>Item 2</title>
                <link href="http://www.example.com/item2.html"/>
                <link href="http://www.example.com/item2.html" type="image/png" rel="enclosure" />
                <summary>Item 2 description</summary>
            </entry>
            <entry>
                <title>Item 3</title>
                <link href="http://www.example.com/item3.html"/>
                <link href="http://www.example.com/item3.html" type="image/png" rel="enclosure" />
                <summary>Item 2 description</summary>
                <published>2003-12-13T08:29:29-04:00</published>
            </entry>
            <entry>
                <title>Item 4</title>
                <link href="http://www.example.com/item4.html" rel="alternate" type="text/html" />
                <link href="http://www.example.com/item4.html" type="image/png" rel="enclosure" />
                <link href="http://www.example.com/item4.xml" rel="self" type="application/atom+xml" />
                <link href="http://www.example.com/item4.html" rel="replies" type="text/html" />
                <summary>Item 4 description</summary>
                <published>2003-12-13T08:29:29-04:00</published>
            </entry>
        </feed>"#;
    let feeds = parse(xml);
    match feeds {
        Ok(feeds) => {
            assert_eq!(feeds.len(), 4);
        }
        Err(_e) => {}
    }
    let feeds = parse(xml).unwrap();
    let feed = feeds.get(2).unwrap();
    assert_eq!(
        feed.publish_date,
        Some("2003-12-13T08:29:29-04:00".to_string())
    );
}
