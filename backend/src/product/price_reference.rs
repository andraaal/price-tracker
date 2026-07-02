#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, sqlx::FromRow)]
pub struct PriceReference {
    #[sqlx(rename = "reference_price")]
    pub price: i16,
    #[sqlx(rename = "reference_unit")]
    pub unit: String,
    #[sqlx(rename = "reference_quantity")]
    pub quantity: i16,
}
