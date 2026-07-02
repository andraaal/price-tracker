#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PriceReference {
    pub price: i16,
    pub unit: String,
    pub quantity: i16,
}
