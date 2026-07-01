pub mod spar;

pub trait VendorAdapter {
    async fn fetch_products(&self) -> anyhow::Result<Vec<super::product::Product>>;
}
