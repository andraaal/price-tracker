use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::product::Product;

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
        sqlx::query("INSERT INTO products (external_id, name, brand, vendor, price, quantity, image_url, reference_price, reference_quantity, reference_unit, tags) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) ON CONFLICT (external_id) DO NOTHING")
            .bind(product.id)
            .bind(product.name)
            .bind(product.brand)
            .bind(product.vendor)
            .bind(product.price)
            .bind(product.quantity)
            .bind(product.image_url)
            .bind(product.reference.price)
            .bind(product.reference.quantity)
            .bind(product.reference.unit)
            .bind(product.tags)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
