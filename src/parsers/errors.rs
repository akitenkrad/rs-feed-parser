use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Failed to parse XML: {0}")]
    XmlParseError(#[from] quick_xml::Error),
    #[error("Failed to parse date: {0}")]
    DateParseError(#[from] chrono::ParseError),
    #[error("Invalid feed format: {0}")]
    InvalidFeedFormat(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
}

pub type ParseResult<T> = Result<T, ParseError>;
