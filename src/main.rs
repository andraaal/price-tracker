use anyhow::Result;
use std::env;

mod adapters;
mod db;
mod fetcher;
mod product;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let db = db::DB::new(&database_url).await?;

    sqlx::migrate!().run(&db.pool).await?;

    let items = fetcher::fetch_spar_items().await?;

    for item in items {
        let product_id = item.id;
        let product_name = format!(
            "{} {} {}",
            item.details.name1,
            item.details.name2.unwrap_or_default(),
            item.details.name3.unwrap_or_default(),
        );
        db.save_product(&product_id, &product_name).await?;
        println!("Saved product: {} - {}", product_id, product_name);
    }

    Ok(())
}
