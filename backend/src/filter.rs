use crate::product::tag::Tag;
use crate::product::vendor::Vendor;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct Filter {
    pub search_string: Option<String>,
    pub vendor: Option<Vendor>,
    pub tag: Option<Tag>,
    pub max_price: Option<i16>,
    pub sort: Option<Sort>,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum Sort {
    PriceAsc,
    PriceDesc,
    NameAsc,
    NameDesc,
    RefAsc,
    RefDesc,
}
