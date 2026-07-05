use crate::product::price_reference::PriceReference;
use crate::product::vendor::Vendor;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProductList {
    #[serde(rename = "hits")]
    pub products: Vec<SparProduct>,
}

#[derive(Debug, Deserialize)]
pub struct SparProduct {
    pub id: String,
    #[serde(rename = "masterValues")]
    pub details: ProductDetails,
}

#[derive(Debug, Deserialize)]
pub struct ProductDetails {
    /// Brand name, don't know why array
    pub brand: Option<Vec<String>>,

    pub price: f32,

    #[serde(rename = "ecr-brand")]
    pub ecr_brand: Option<String>,

    #[serde(rename = "price-per-unit")]
    pub reference_price: String,

    #[serde(rename = "regular-price")]
    pub base_price: f32,

    #[serde(rename = "url")]
    pub shop_url: String,

    #[serde(default = "default_name")]
    #[serde(rename = "short-description")]
    pub name: String,

    #[serde(rename = "short-description-3")]
    pub quantity: Option<String>,

    #[serde(rename = "badge-names")]
    pub tags: Option<String>,

    #[serde(rename = "image-url")]
    pub image_url: String,

    #[expect(unused)]
    #[serde(rename = "category-names")]
    pub categorys: Option<String>,
}

impl Into<crate::product::Product> for SparProduct {
    fn into(self) -> crate::product::Product {
        let details = self.details;

        let id = format!("spar_{}", self.id);
        let brand = details.ecr_brand.unwrap_or(
            details
                .brand
                .unwrap_or(vec!["No brand".to_string()])
                .join(", "),
        );
        let name = details.name;

        let vendor = Vendor::Spar;
        let price = (details.price * 100.0) as i16;
        let base_price = (details.base_price * 100.0) as i16;
        let quantity = details.quantity.unwrap_or("1 Stk.".to_string());
        let image_url = details.image_url;
        let shop_url = "https://www.interspar.at/shop/lebensmittel".to_string() + &details.shop_url;

        let reference =
            details
                .reference_price
                .parse::<PriceReference>()
                .unwrap_or(PriceReference {
                    price: 100,
                    unit: "Stk.".to_string(),
                });

        let tags = details
            .tags
            .unwrap_or_default()
            .split('|')
            .map(|tag| tag.parse::<crate::product::tag::Tag>())
            .filter_map(|tag| tag.ok())
            .collect();

        crate::product::Product {
            id,
            name,
            brand,
            vendor,
            price,
            base_price,
            quantity,
            image_url,
            shop_url,
            reference,
            tags,
        }
    }
}

fn default_name() -> String {
    "No name".to_string()
}
