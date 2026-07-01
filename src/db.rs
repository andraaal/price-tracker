use sqlx::{PgPool, postgres::PgPoolOptions};

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

    pub async fn save_product(&self, external_id: &str, name: &str) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO products (external_id, name) VALUES ($1, $2) ON CONFLICT (external_id) DO NOTHING")
            .bind(external_id)
            .bind(name)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
