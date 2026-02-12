use rss::Channel;
use std::borrow::Cow;
use std::time::Duration;

use crate::error::FeedError;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(30);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);
const MAX_RESPONSE_BYTES: usize = 5 * 1024 * 1024; // 5 MB

/// Fetch all feeds in parallel and return merged items.
/// Partial failures are logged but don't prevent other feeds from loading.
pub async fn fetch_all(urls: Vec<String>) -> Result<Vec<rss::Item>, FeedError> {
    if urls.is_empty() {
        return Ok(Vec::new());
    }

    let client = build_client()?;

    let futures: Vec<_> = urls
        .into_iter()
        .map(|url| fetch(&client, url))
        .collect();
    let results = futures_util::future::join_all(futures).await;

    let mut items = Vec::new();
    let mut last_error = None;

    for result in results {
        match result {
            Ok(channel) => items.extend(channel.items().to_vec()),
            Err(e) => {
                eprintln!("feed error: {e}");
                last_error = Some(e);
            }
        }
    }

    if !items.is_empty() {
        Ok(items)
    } else if let Some(e) = last_error {
        Err(e)
    } else {
        Ok(Vec::new())
    }
}

/// Validate that a URL is safe to open in a browser.
pub fn validate_url(url: &str) -> Result<(), FeedError> {
    let parsed = url::Url::parse(url)
        .map_err(|_| FeedError::InvalidUrl(url.to_string()))?;

    match parsed.scheme() {
        "http" | "https" => {}
        _ => return Err(FeedError::InvalidUrl(format!("unsupported scheme: {}", parsed.scheme()))),
    }

    if let Some(host) = parsed.host_str()
        && (host == "localhost"
            || host.starts_with("127.")
            || host.starts_with("10.")
            || host.starts_with("192.168.")
            || host == "0.0.0.0"
            || host == "[::1]")
    {
        return Err(FeedError::InvalidUrl("private/local addresses not allowed".to_string()));
    }

    Ok(())
}

fn build_client() -> Result<reqwest::Client, FeedError> {
    reqwest::Client::builder()
        .user_agent("cosmic-ext-applet-feeds")
        .connect_timeout(CONNECT_TIMEOUT)
        .timeout(REQUEST_TIMEOUT)
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()
        .map_err(|e| FeedError::Request(e.to_string()))
}

async fn fetch(client: &reqwest::Client, url: String) -> Result<Channel, FeedError> {
    validate_url(&url)?;

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| FeedError::Request(e.to_string()))?;

    if !resp.status().is_success() {
        return Err(FeedError::HttpStatus {
            status: resp.status().as_u16(),
            url,
        });
    }

    if let Some(len) = resp.content_length()
        && len as usize > MAX_RESPONSE_BYTES
    {
        return Err(FeedError::Request(format!(
            "response too large: {} bytes (max {})",
            len, MAX_RESPONSE_BYTES
        )));
    }

    let bytes = resp
        .bytes()
        .await
        .map_err(|e| FeedError::Request(e.to_string()))?;

    if bytes.len() > MAX_RESPONSE_BYTES {
        return Err(FeedError::Request(format!(
            "response too large: {} bytes (max {})",
            bytes.len(),
            MAX_RESPONSE_BYTES
        )));
    }

    Channel::read_from(&bytes[..]).map_err(|e| {
        let body = String::from_utf8_lossy(&bytes);
        let lower = body.trim_start().to_lowercase();
        if lower.starts_with("<!doctype html") || lower.starts_with("<html") {
            FeedError::NotAFeed
        } else {
            FeedError::Parse(e.to_string())
        }
    })
}

/// Strip HTML tags and decode common HTML entities from feed text.
#[must_use]
pub fn sanitize_text(input: &str) -> Cow<'_, str> {
    // Fast path: no HTML-like content at all
    if !input.contains('<') && !input.contains('&') {
        return Cow::Borrowed(input);
    }

    let mut out = String::with_capacity(input.len());
    let mut in_tag = false;

    for ch in input.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }

    // Decode common HTML entities
    let decoded = out
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&#x27;", "'")
        .replace("&apos;", "'")
        .replace("&nbsp;", " ");

    Cow::Owned(decoded.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_url_accepts_https() {
        assert!(validate_url("https://example.com/feed.xml").is_ok());
    }

    #[test]
    fn validate_url_accepts_http() {
        assert!(validate_url("http://example.com/feed.xml").is_ok());
    }

    #[test]
    fn validate_url_rejects_ftp() {
        assert!(validate_url("ftp://example.com/feed.xml").is_err());
    }

    #[test]
    fn validate_url_rejects_javascript() {
        assert!(validate_url("javascript:alert(1)").is_err());
    }

    #[test]
    fn validate_url_rejects_localhost() {
        assert!(validate_url("http://localhost:8080/feed").is_err());
    }

    #[test]
    fn validate_url_rejects_loopback() {
        assert!(validate_url("http://127.0.0.1/feed").is_err());
    }

    #[test]
    fn validate_url_rejects_private_10() {
        assert!(validate_url("http://10.0.0.1/feed").is_err());
    }

    #[test]
    fn validate_url_rejects_private_192() {
        assert!(validate_url("http://192.168.1.1/feed").is_err());
    }

    #[test]
    fn validate_url_rejects_invalid() {
        assert!(validate_url("not a url").is_err());
    }

    #[test]
    fn sanitize_plain_text_unchanged() {
        let input = "Hello World";
        assert!(matches!(sanitize_text(input), Cow::Borrowed(_)));
        assert_eq!(sanitize_text(input), "Hello World");
    }

    #[test]
    fn sanitize_strips_html_tags() {
        assert_eq!(sanitize_text("<b>Bold</b> text"), "Bold text");
    }

    #[test]
    fn sanitize_decodes_html_entities() {
        assert_eq!(sanitize_text("A &amp; B"), "A & B");
        assert_eq!(sanitize_text("1 &lt; 2"), "1 < 2");
        assert_eq!(sanitize_text("2 &gt; 1"), "2 > 1");
        assert_eq!(sanitize_text("&quot;quoted&quot;"), "\"quoted\"");
    }

    #[test]
    fn sanitize_handles_mixed_content() {
        assert_eq!(
            sanitize_text("<p>News &amp; <em>updates</em></p>"),
            "News & updates"
        );
    }

    #[test]
    fn sanitize_trims_whitespace() {
        assert_eq!(sanitize_text("  <br>  hello  "), "hello");
    }

    #[test]
    fn sanitize_empty_string() {
        assert_eq!(sanitize_text(""), "");
    }

    #[tokio::test]
    async fn fetch_all_empty_urls() {
        let result = fetch_all(Vec::new()).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }
}
