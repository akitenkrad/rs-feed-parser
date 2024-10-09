use serde::Deserialize;
pub mod atom;
pub mod rss1;
pub mod rss2;

/// Represents a feed with various metadata fields.
///
/// This struct is used to deserialize feed data from different formats such as Atom, RSS1, and RSS2.
///
/// # Fields
///
/// * `title` - The title of the feed.
/// * `link` - The URL link to the feed.
/// * `description` - An optional description of the feed.
/// * `summary` - An optional summary of the feed.
/// * `updated` - An optional field representing the last updated date of the feed.
/// * `publish_date` - An optional field representing the publish date of the feed.
/// * `creator` - An optional field representing the creator of the feed.
/// * `date` - An optional field representing the date associated with the feed.
/// * `other` - An optional field for any other additional information.
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
