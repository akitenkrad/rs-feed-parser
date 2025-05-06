use super::*;

#[test]
fn test_parse_rss1_1() {
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

#[test]
fn test_parse_rss1_2() {
    let xml = r#"
        <rdf:RDF>
            <channel>
                <title>RSS Title</title>
                <link>http://www.example.com/main.html</link>
                <description>This is an example of an RSS feed</description>
                <item>
                    <title>Item 1</title>
                    <link>http://www.example.com/item1.html</link>
                    <description><p>Item 1 description</p><br></description>
                    <dc:creator>John Doe</dc:creator>
                    <dc:date>2003-12-13T18:30:02Z</dc:date>
                    <pubDate>2024-01-01T23:59:02Z</pubDate>
                </item>
                <item>
                    <title>Item 2</title>
                    <link>http://www.example.com/item2.html</link>
                    <description><b>Item 2 description</b><image></description>
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

#[test]
fn test_parse_rss1_3() {
    let xml = r#"
        <?xml version="1.0"?>
        <rdf:RDF>
            <channel>
                <title>RSS Title</title>
                <link>http://www.example.com/main.html</link>
                <description>This is an example of an RSS feed</description>
                <description>Sample Description</description>
                <lastBuildDate>Sat, 19 Apr 2025 03:13:04 +0000</lastBuildDate>
                <language>ja</language>
                <sy:updatePeriod>
                hourly	</sy:updatePeriod>
                <sy:updateFrequency>1</sy:updateFrequency>
                <generator>https://wordpress.org/?v=6.2.6</generator>
                <image>
                    <url>https://sample.com/sample.png</url>
                    <title>Sample</title>
                    <link>https://sample.com</link>
                    <width>32</width>
                    <height>32</height>
                </image> 
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
