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
