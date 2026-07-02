pub mod price_reference;
pub mod tag;
pub mod vendor;

#[derive(sqlx::FromRow, serde::Serialize, Debug, Clone)]
pub struct Product {
    /// This is the vendor, and underscore and the vendor's product id.
    #[sqlx(rename = "external_id")]
    pub id: String,
    pub name: String,
    pub brand: String,
    pub vendor: vendor::Vendor,
    pub price: i16,
    pub quantity: String,
    pub image_url: String,

    #[sqlx(flatten)]
    pub reference: price_reference::PriceReference,
    pub tags: Vec<tag::Tag>,
}
