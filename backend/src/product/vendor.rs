#[derive(sqlx::Type, Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[sqlx(type_name = "vendor")]
pub enum Vendor {
    Spar,
}

impl Into<String> for Vendor {
    fn into(self) -> String {
        match self {
            Vendor::Spar => "SPAR".to_string(),
        }
    }
}
