pub use url::*;

static PROTOCOL_HTTP: &str = "http://";
static PROTOCOL_HTTPS: &str = "https://";
static PROTOCOL_FILE: &str = "file:////";
static PROTOCOL_DATA: &str = "data:";
static PROTOCOLS: &[&str] = &[PROTOCOL_HTTP, PROTOCOL_HTTPS, PROTOCOL_FILE, PROTOCOL_DATA];

/// Adds the protocol in front of the url if it is missing from the input
fn add_missing_protocol(url_str: &str) -> String {
    for protocol in PROTOCOLS {
        if url_str.starts_with(protocol) {
            return url_str.to_string();
        }
    }

    format!("{}{}", PROTOCOL_HTTPS, url_str)
}

/// Parses the given url into the url representation
/// Allows for fuzzier input than the original method. If no protocol is provided,
/// it assumes https.
#[inline]
pub fn parse_url(url_str: &str) -> Result<Url, url::ParseError> {
    let url_str = add_missing_protocol(url_str);
    Url::parse(&url_str)
}

/// Returns the domain for a given url string
/// Example
/// ```
/// use bot_coreutils::url::get_domain_for_url;
///
/// assert_eq!(get_domain_for_url("https://reddit.com/r/anime"), Some("reddit.com".to_string()));
/// assert_eq!(get_domain_for_url("reddit.com"), Some("reddit.com".to_string()));
/// assert_eq!(get_domain_for_url("invalid url"), None);
/// ```
pub fn get_domain_for_url(url_str: &str) -> Option<String> {
    let url = parse_url(url_str).ok()?;
    let domain = url.domain()?;

    Some(domain.trim_start_matches("www.").to_string())
}
