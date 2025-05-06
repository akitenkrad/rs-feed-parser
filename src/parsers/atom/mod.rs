use crate::parsers::{
    errors::{ParseError, ParseResult},
    Feed,
};
use core::str;
use quick_xml::de::from_str;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use regex::Regex;
use std::io::Cursor;

#[cfg(test)]
mod tests;

fn preprocess(text: &str) -> String {
    let tags = vec!["title", "description", "summary", "content"];
    let mut text = text.to_string();
    for tag in tags {
        let re = Regex::new(&format!(r#"<{}>(?<content>.*?)</{}>"#, tag, tag)).unwrap();
        let m = re.replace_all(&text, |caps: &regex::Captures| {
            let content = &caps["content"];
            let content = html_escape::encode_text(content);
            format!("<{}>{}</{}>", tag, content, tag)
        });
        text = m.to_string();
    }
    return text.to_string();
}

pub fn parse(text: &str) -> ParseResult<Vec<Feed>> {
    let text = preprocess(text);

    let mut reader = Reader::from_str(&text);
    reader.config_mut().trim_text(true);

    let mut feeds = Vec::new();
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut parsing = false;
    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                if parsing {
                    if e.name().as_ref() == b"dc:creator" {
                        assert!(writer
                            .write_event(Event::Start(BytesStart::new("creator")))
                            .is_ok());
                    } else if e.name().as_ref() == b"dc:date" {
                        assert!(writer
                            .write_event(Event::Start(BytesStart::new("date")))
                            .is_ok());
                    } else if e.name().as_ref() == b"pubDate" || e.name().as_ref() == b"published" {
                        assert!(writer
                            .write_event(Event::Start(BytesStart::new("publish_date")))
                            .is_ok());
                    } else if e.name().as_ref() == b"description" {
                        assert!(writer
                            .write_event(Event::Start(BytesStart::new("description")))
                            .is_ok());
                    } else if e.name().as_ref() == b"link" {
                        continue;
                    } else {
                        assert!(writer.write_event(Event::Start(e.clone())).is_ok());
                    }
                }
                if e.name().as_ref() == b"entry" {
                    assert!(writer
                        .write_event(Event::Start(BytesStart::new("entry")))
                        .is_ok());
                    parsing = true;
                }
            }
            Ok(Event::Empty(e)) => {
                if parsing {
                    if e.name().as_ref() == b"link" {
                        let mut is_link = true;
                        for attr in e.attributes() {
                            let attr = attr.unwrap();
                            if attr.key.0 == b"type" {
                                let attr_text: &str = str::from_utf8(attr.value.as_ref()).unwrap();
                                if attr_text != "text/html" {
                                    is_link = false;
                                }
                            } else if attr.key.0 == b"rel" {
                                let attr_text: &str = str::from_utf8(attr.value.as_ref()).unwrap();
                                if attr_text != "alternate" {
                                    is_link = false;
                                }
                            }
                        }
                        if is_link == false {
                            continue;
                        }
                        for attr in e.attributes() {
                            let attr = attr.unwrap();
                            if attr.key.0 == b"href" {
                                assert!(writer
                                    .write_event(Event::Start(BytesStart::new("link")))
                                    .is_ok());
                                let attr_text: &str = str::from_utf8(attr.value.as_ref()).unwrap();
                                assert!(writer
                                    .write_event(Event::Text(BytesText::new(attr_text)))
                                    .is_ok());
                                assert!(writer
                                    .write_event(Event::End(BytesEnd::new("link")))
                                    .is_ok());
                            }
                        }
                    } else {
                        assert!(writer.write_event(Event::Empty(e)).is_ok());
                    }
                }
            }
            Ok(Event::End(e)) => {
                if e.name().as_ref() == b"entry" {
                    assert!(writer
                        .write_event(Event::End(BytesEnd::new("entry")))
                        .is_ok());
                    let feed_text = writer.into_inner().into_inner();
                    let feed = from_str::<Feed>(str::from_utf8(&feed_text).unwrap()).unwrap();
                    feeds.push(feed);

                    writer = Writer::new(Cursor::new(Vec::new()));
                    parsing = false;
                }
                if parsing {
                    if e.name().as_ref() == b"dc:creator" {
                        assert!(writer
                            .write_event(Event::End(BytesEnd::new("creator")))
                            .is_ok());
                    } else if e.name().as_ref() == b"dc:date" {
                        assert!(writer
                            .write_event(Event::End(BytesEnd::new("date")))
                            .is_ok());
                    } else if e.name().as_ref() == b"pubDate" || e.name().as_ref() == b"published" {
                        assert!(writer
                            .write_event(Event::End(BytesEnd::new("publish_date")))
                            .is_ok());
                    } else if e.name().as_ref() == b"link" {
                        continue;
                    } else {
                        assert!(writer.write_event(Event::End(e)).is_ok());
                    }
                }
            }
            Ok(Event::Text(e)) => {
                if parsing {
                    let text = str::from_utf8(&e as &[u8]).unwrap();
                    let text = html_escape::decode_html_entities(text);
                    let e = BytesText::new(&text);
                    assert!(writer.write_event(Event::Text(e)).is_ok());
                }
            }
            Ok(Event::CData(e)) => {
                if parsing {
                    assert!(writer.write_event(Event::CData(e)).is_ok());
                }
            }
            Ok(Event::Eof) => break,
            Ok(_e) => {}
            Err(e) => {
                return Err(ParseError::XmlParseError(e));
            }
        }
    }
    return Ok(feeds);
}
