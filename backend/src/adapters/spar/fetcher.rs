use crate::product::Product;

use super::structure::ProductList;
use anyhow::Result;
use reqwest::Client;

pub async fn fetch(client: &Client) -> Result<Vec<Product>> {
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
        for item in items.products {
            products.push(item.into());
        }

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    Ok(products)
}
