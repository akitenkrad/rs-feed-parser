use super::*;

#[test]
fn test_parse_rss2_1() {
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
                <item>
                    <title>Item 3</title>
                    <link>http://www.example.com/item3.html</link>
                    <description>
                        <![CDATA[続きを読む...</a></b></p> ]]>
                    </description>
                    <pubDate>2024-01-01T23:59:02Z</pubDate>
                </item>
            </channel>
        </rss>"#;
    let feeds = parse(xml).unwrap();
    let feed1 = &feeds[0];
    assert_eq!(feed1.title, "Item 1");
    assert_eq!(feed1.link, "http://www.example.com/item1.html");
    assert_eq!(feed1.description.clone().unwrap(), "Item 1 description");
    assert_eq!(feed1.publish_date.clone().unwrap(), "2024-01-01T23:59:02Z");

    let feed2 = &feeds[1];
    assert_eq!(feed2.title, "Item 2");
    assert_eq!(feed2.link, "http://www.example.com/item2.html");
    assert_eq!(feed2.description.clone().unwrap(), "Item 2 description");

    let feed3 = &feeds[2];
    assert_eq!(feed3.title, "Item 3");
    assert_eq!(feed3.link, "http://www.example.com/item3.html");
    assert_eq!(
        feed3.description.clone().unwrap(),
        "続きを読む...</a></b></p> "
    );
    assert_eq!(feed3.publish_date.clone().unwrap(), "2024-01-01T23:59:02Z");
}

#[test]
fn test_parse_rss2_2() {
    let xml = r#"
        <rss version="2.0">
            <channel>
                <title>RSS Title</title>
                <link>http://www.example.com/main.html</link>
                <description><p>This is an example of an RSS feed</p><br/></description>
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
                <item>
                    <title>Item 3</title>
                    <link>http://www.example.com/item3.html</link>
                    <description>
                        <![CDATA[続きを読む...</a></b></p> ]]>
                    </description>
                    <pubDate>2024-01-01T23:59:02Z</pubDate>
                </item>
            </channel>
        </rss>"#;
    let feeds = parse(xml).unwrap();
    let feed1 = &feeds[0];
    assert_eq!(feed1.title, "Item 1");
    assert_eq!(feed1.link, "http://www.example.com/item1.html");
    assert_eq!(feed1.description.clone().unwrap(), "Item 1 description");
    assert_eq!(feed1.publish_date.clone().unwrap(), "2024-01-01T23:59:02Z");

    let feed2 = &feeds[1];
    assert_eq!(feed2.title, "Item 2");
    assert_eq!(feed2.link, "http://www.example.com/item2.html");
    assert_eq!(feed2.description.clone().unwrap(), "Item 2 description");

    let feed3 = &feeds[2];
    assert_eq!(feed3.title, "Item 3");
    assert_eq!(feed3.link, "http://www.example.com/item3.html");
    assert_eq!(
        feed3.description.clone().unwrap(),
        "続きを読む...</a></b></p> "
    );
    assert_eq!(feed3.publish_date.clone().unwrap(), "2024-01-01T23:59:02Z");
}
