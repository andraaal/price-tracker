use crate::product::Product;

use super::structure::ProductList;
use anyhow::Result;
use reqwest::Client;

pub async fn fetch(client: &Client) -> Result<Vec<Product>> {
    let mut products: Vec<Product> = Vec::new();

    for i in 1..=487 {
        let url = format!(
            "https://search-spar.spar-ics.com/fact-finder/rest/v4/search/products_lmos_at?query=*&q=*&hitsPerPage=80&page={}",
            i
        );

        let response = client.get(url).send().await?;
        let text = response.text().await?;
        println!("Parsing page {}", i);
        let items = serde_json::from_str::<ProductList>(&text)?;
        for item in items.products {
            products.push(item.into());
        }

        // TODO remove when done testing
        break;

        #[expect(unreachable_code)]
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }

    Ok(products)
}
