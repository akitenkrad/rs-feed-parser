use serde::Deserialize;
pub mod atom;
pub mod rss1;
pub mod rss2;

#[derive(Deserialize, PartialEq, Debug)]
pub struct Feed {
    pub title: String,
    pub link: String,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub updated: Option<String>,
    pub publish_date: Option<String>,
    pub creator: Option<String>,
    pub date: Option<String>,
    pub other: Option<String>,
}
