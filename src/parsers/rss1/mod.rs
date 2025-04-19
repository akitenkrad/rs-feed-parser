use crate::parsers::Feed;
use core::str;
use quick_xml::de::from_str;
use quick_xml::events::{BytesEnd, BytesStart, Event};
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

pub fn parse(text: &str) -> Result<Vec<Feed>, String> {
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
                    } else if e.name().as_ref() == b"pubDate" {
                        assert!(writer
                            .write_event(Event::Start(BytesStart::new("publish_date")))
                            .is_ok());
                    } else {
                        assert!(writer.write_event(Event::Start(e.clone())).is_ok());
                    }
                }
                if e.name().as_ref() == b"item" {
                    assert!(writer
                        .write_event(Event::Start(BytesStart::new("item")))
                        .is_ok());
                    parsing = true;
                }
            }
            Ok(Event::End(e)) => {
                if e.name().as_ref() == b"item" {
                    assert!(writer
                        .write_event(Event::End(BytesEnd::new("item")))
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
                    } else if e.name().as_ref() == b"pubDate" {
                        assert!(writer
                            .write_event(Event::End(BytesEnd::new("publish_date")))
                            .is_ok());
                    } else {
                        assert!(writer.write_event(Event::End(e)).is_ok());
                    }
                }
            }
            Ok(Event::Text(e)) => {
                if parsing {
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
                return Err(format!(
                    "Error at position {}: {:?}",
                    reader.error_position(),
                    e
                ))
            }
        }
    }
    return Ok(feeds);
}
