use anyhow::Result;
use sqlx::{PgPool, QueryBuilder, postgres::PgPoolOptions};

use crate::{
    filter::{Filter, Sort},
    product::Product,
};

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

    pub async fn get_products(
        &self,
        start: i32,
        length: i32,
        filter: Filter,
    ) -> Result<Vec<Product>> {
        let mut query = QueryBuilder::<sqlx::Postgres>::new("SELECT * FROM products WHERE TRUE");

        if let Some(search) = filter.search_string {
            query.push(" AND (name ILIKE ");
            query.push_bind(format!("%{search}%"));
            query.push(" OR brand ILIKE ");
            query.push_bind(format!("%{search}%"));
            query.push(")");
        }

        if let Some(vendor) = filter.vendor {
            query.push(" AND vendor = ");
            query.push_bind(vendor);
        }

        if let Some(tag) = filter.tag {
            query.push(" AND ");
            query.push_bind(tag);
            query.push(" = ANY(tags)");
        }

        if let Some(max_price) = filter.max_price {
            query.push(" AND price <= ");
            query.push_bind(max_price);
        }

        query.push(" ORDER BY ");

        match filter.sort {
            Some(Sort::PriceAsc) => query.push("price ASC"),
            Some(Sort::PriceDesc) => query.push("price DESC"),
            Some(Sort::NameAsc) => query.push("name ASC"),
            Some(Sort::NameDesc) => query.push("name DESC"),
            Some(Sort::RefAsc) => query.push("reference_price ASC"),
            Some(Sort::RefDesc) => query.push("reference_price DESC"),
            None => query.push("name ASC"),
        };

        query.push(" LIMIT ");
        query.push_bind(length);

        query.push(" OFFSET ");
        query.push_bind(start);

        let products = query
            .build_query_as::<Product>()
            .fetch_all(&self.pool)
            .await?;

        Ok(products)
    }
}
