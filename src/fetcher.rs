use crate::adapters::spar::{Product, ProductList};
use anyhow::Result;
use reqwest::header;

#[expect(dead_code)]
pub async fn fetch_items(url: &str) -> Result<ProductList> {
    let header = create_header();

    let client = reqwest::Client::builder().default_headers(header).build()?;
    let response = client.get(url).send().await?;

    let text = response.text().await?;
    let items = serde_json::from_str::<ProductList>(&text)?;

    Ok(items)
}

pub async fn fetch_spar_items() -> Result<Vec<Product>> {
    let header = create_header();
    let client = reqwest::Client::builder().default_headers(header).build()?;

    let mut products: Vec<Product> = Vec::new();

    for i in 1..=487 {
        let url = format!(
            "https://api-scp.spar-ics.com/ecom/pw/v1/search/v1/navigation?query=*&page={}&marketId=8999&showPermutedSearchParams=false&filter=pwCategoryPathIDs%3Alebensmittel&hitsPerPage=32",
            i
        );

        let response = client.get(url).send().await?;
        let text = response.text().await?;
        println!("Parsing page {}", i);
        let items = serde_json::from_str::<ProductList>(&text)?;
        products.extend(items.products);

        // TODO remove when done testing
        break;

        #[expect(unreachable_code)]
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }

    Ok(products)
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
