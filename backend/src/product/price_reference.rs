use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, sqlx::FromRow)]
pub struct PriceReference {
    #[sqlx(rename = "reference_price")]
    pub price: i16,
    #[sqlx(rename = "reference_unit")]
    pub unit: String,
}

impl FromStr for PriceReference {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 2 {
            println!("Invalid price reference format: {}", s);
            return Err(anyhow::anyhow!("Invalid price reference format"));
        }

        let price = parts[0]
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<f32>()? as i16;
        let unit = parts[1].to_string();

        Ok(PriceReference { price, unit })
    }
}
