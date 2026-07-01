use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProductList {
    #[serde(rename = "hits")]
    pub products: Vec<Product>,
}

#[derive(Debug, Deserialize)]
pub struct Product {
    pub id: String,
    #[serde(rename = "masterValues")]
    pub details: ProductDetails,
}

#[derive(Debug, Deserialize)]
pub struct ProductDetails {
    pub name1: String,
    pub name2: Option<String>,
    pub name3: Option<String>,
}
