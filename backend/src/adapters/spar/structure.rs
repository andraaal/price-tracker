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
    /// Brand name, product name if name2 is empty
    pub name1: String,
    /// Product name if name2 is not empty
    pub name2: Option<String>,
    /// Product quantity
    pub name3: Option<String>,
    #[serde(rename = "badgeAltTexts_internalPlp")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "productImage_assetUrl")]
    pub image_url: String,
    #[serde(rename = "geoInformation")]
    pub prices: Vec<MarketPrice>,
}

#[derive(Debug, Deserialize)]
pub struct MarketPrice {
    #[serde(rename = "geoValues")]
    pub price: Price,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Price {
    #[serde(rename = "calculatedPrice")]
    pub price: f32,
    #[serde(rename = "comparisonPrice_price")]
    pub comparison_price: f32,
    #[serde(rename = "comparisonPrice_unit")]
    pub comparison_unit: String,
    #[serde(rename = "comparisonPrice_quantity")]
    pub comparison_quantity: i16,
    #[serde(rename = "basePrice")]
    pub base_price: f32,
}

impl Into<crate::product::Product> for SparProduct {
    fn into(self) -> crate::product::Product {
        let details = self.details;
        // Taking would be fine, but borrow checker wants to clone
        let prices = details.prices[0].price.clone();

        let id = format!("spar_{}", self.id);
        let brand;
        let name;
        if let Some(name2) = details.name2 {
            brand = details.name1;
            name = name2;
        } else {
            brand = "No Brand".to_string();
            name = details.name1;
        }

        let vendor = Vendor::Spar;
        let price = (prices.price * 100.0) as i16;
        let quantity = details.name3.unwrap_or("1 Stk.".to_string());
        let image_url = details
            .image_url
            .replace("{ext}", "jpg")
            .replace("{size}", "940");

        let reference = PriceReference {
            price: (prices.comparison_price * 100.0) as i16,
            unit: prices.comparison_unit,
            quantity: prices.comparison_quantity,
        };

        let tags = details
            .tags
            .unwrap_or_default()
            .into_iter()
            .map(|tag| tag.parse::<crate::product::tag::Tag>())
            .filter_map(|tag| tag.ok())
            .collect();

        crate::product::Product {
            id,
            name,
            brand,
            vendor,
            price,
            quantity,
            image_url,
            reference,
            tags,
        }
    }
}
