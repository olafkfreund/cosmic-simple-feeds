use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum FeedError {
    #[error("HTTP request failed: {0}")]
    Request(String),

    #[error("HTTP {status}: {url}")]
    HttpStatus { status: u16, url: String },

    #[error("failed to parse RSS feed: {0}")]
    Parse(String),

    #[error("URL is a web page, not an RSS feed")]
    NotAFeed,

    #[error("invalid feed URL: {0}")]
    InvalidUrl(String),
}
