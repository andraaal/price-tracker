pub mod price_reference;
pub mod tag;
pub mod vendor;

pub struct Product {
    /// This is the vendor, and underscore and the vendor's product id.
    pub id: String,
    pub name: String,
    pub brand: String,
    pub vendor: vendor::Vendor,
    pub price: i16,
    pub quantity: String,
    pub image_url: String,

    pub reference: price_reference::PriceReference,
    pub tags: Vec<tag::Tag>,
}
