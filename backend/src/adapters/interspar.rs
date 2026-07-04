use anyhow::Result;
use reqwest::Client;

mod fetcher;
mod structure;

pub async fn fetch_items(client: &Client) -> Result<Vec<crate::product::Product>> {
    let items = fetcher::fetch(client).await?;
    Ok(items)
}
