use anyhow::Result;
use reqwest::header;

pub fn create_client() -> Result<reqwest::Client> {
    let headers = create_header();
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    Ok(client)
}

/// Creates a header that is hopefully accepted by vendor APIs. Imitates a browser for this reason.
fn create_header() -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/148.0.0.0 Safari/537.36"),
    );
    headers.insert(
        header::ACCEPT_LANGUAGE,
        header::HeaderValue::from_static("en-GB,en;q=0.7"),
    );
    headers.insert(
        header::CACHE_CONTROL,
        header::HeaderValue::from_static("no-cache"),
    );
    headers.insert(
        header::UPGRADE_INSECURE_REQUESTS,
        header::HeaderValue::from_static("1"),
    );
    headers
}
