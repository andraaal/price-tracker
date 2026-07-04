use anyhow::Result;
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::product::Product;

#[derive(Clone)]
pub struct DB {
    pub pool: PgPool,
}

impl DB {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        Ok(Self { pool })
    }

    pub async fn save_product(&self, product: Product) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO products (external_id, name, brand, vendor, price, base_price, quantity, image_url, shop_url, reference_price, reference_unit, tags) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) ON CONFLICT (external_id) DO NOTHING")
            .bind(product.id)
            .bind(product.name)
            .bind(product.brand)
            .bind(product.vendor)
            .bind(product.price)
            .bind(product.base_price)
            .bind(product.quantity)
            .bind(product.image_url)
            .bind(product.shop_url)
            .bind(product.reference.price)
            .bind(product.reference.unit)
            .bind(product.tags)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_all_products(&self) -> Result<Vec<Product>> {
        let products = sqlx::query_as::<_, Product>("SELECT * FROM products")
            .fetch_all(&self.pool)
            .await?;
        Ok(products)
    }
}
