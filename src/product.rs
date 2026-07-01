pub mod price_reference;
pub mod tag;
pub mod vendor;

pub struct Product {
    /// This is the vendor, and underscore and the vendor's product id.
    pub id: String,
    pub name: String,
    pub brand: String,
    pub vendor: String,
    pub price: u16,
    pub quantity: String,
    pub image_url: String,

    pub reference: Option<price_reference::PriceReference>,
    pub deposit: Option<u16>,
    pub tags: Vec<tag::Tag>,
}
